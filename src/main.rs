mod bridge;
mod doom;
use doom::{DoomThingExt, DoomWorldExt};

use aetheris::assets::AssetWarehouse;
use aetheris::assets::wad::WadLoader;
use aetheris::infrastructure::InputManager;
use aetheris::infrastructure::audio::AudioBridge;
use aetheris::infrastructure::music::MusicEngine;
use aetheris::presentation::{VisualBridge, classic_engine::ClassicSoftwareEngine};
use aetheris::simulation::{GameAction, HudMessage, MenuState, WorldState};
use winit::event::{DeviceEvent, ElementState, Event, MouseButton, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{CursorGrabMode, Window};

const HELP_TEXT: &str = "\
Aetheris DOOM - a 100% memory-safe Rust recreation of the 1993 classic.

USAGE:
    aetheris_doom [OPTIONS]

OPTIONS:
    --wad <FILE>       WAD file to load (default: freedoom1.wad, falls back to DOOM1.WAD)
    --profile          Print renderer/simulation performance histograms to the console
    --telemetry        Periodically export world state snapshots to telemetry.json
    --golden-test      Render fixed scenes, compare against tests/goldens/, then exit
    --update-goldens   Render fixed scenes and (re)write the golden baselines
    -h, --help         Show this help

CONTROLS:
    W/S or Up/Down     Move forward / backward
    A/D                Strafe left / right
    Left/Right         Turn (hold Alt or Z to strafe)
    Mouse              Turn; left button fires
    Ctrl / Space       Fire
    E or F             Use (open doors, flip switches)
    1-7                Select weapon (1 toggles fist/chainsaw)
    Tab                Toggle automap        P      Pause
    Esc                Menu                  F12    Screenshot
    F2 / F3            Save / load menu      F5/F9  Quicksave / quickload
";

const MOUSE_SENSITIVITY: f32 = 0.0025;
const TAU: f32 = 2.0 * std::f32::consts::PI;

#[derive(Clone, Copy, PartialEq)]
enum MapScheme {
    /// DOOM 1 style ExMy map names.
    Episodic,
    /// DOOM 2 style MAPxx map names.
    MapXX,
}

fn map_lump_name(scheme: MapScheme, episode: u32, map: u32) -> String {
    match scheme {
        MapScheme::Episodic => format!("E{}M{}", episode, map),
        MapScheme::MapXX => format!("MAP{:02}", map),
    }
}

struct DoomEntity<'a> {
    thing: &'a aetheris::simulation::Thing,
    world: &'a WorldState,
}
impl<'a> aetheris::presentation::AetherisEntity for DoomEntity<'a> {
    fn position(&self) -> glam::Vec2 {
        self.thing.position
    }
    fn z(&self) -> f32 {
        self.thing.z
    }
    fn get_sprites(&self, viewer_pos: glam::Vec2, frame_count: u64) -> Vec<String> {
        crate::bridge::PresentationMapper::get_animated_sprite(
            self.thing,
            viewer_pos,
            frame_count,
            self.world,
        )
    }
    fn should_draw(&self) -> bool {
        !self.thing.picked_up && self.thing.kind != 1
    }
    fn is_spectral(&self) -> bool {
        self.thing.kind == 58 // Demon Invisibility effect
    }
}

struct DoomPlayer<'a> {
    player: &'a aetheris::simulation::Player,
}
impl<'a> aetheris::presentation::AetherisPlayer for DoomPlayer<'a> {
    fn position(&self) -> glam::Vec2 {
        self.player.position
    }
    fn z(&self) -> f32 {
        self.player.z
    }
    fn angle(&self) -> f32 {
        self.player.angle
    }
    fn fov(&self) -> f32 {
        self.player.fov
    }
    fn damage_flash(&self) -> f32 {
        self.player.damage_flash
    }
    fn bonus_flash(&self) -> f32 {
        self.player.bonus_flash
    }
    fn invuln_timer(&self) -> u32 {
        self.player.invuln_timer
    }
    fn radsuit_timer(&self) -> u32 {
        self.player.radsuit_timer
    }
}

/// Respawn monster/barrel thinkers for the current world, preserving any AI
/// state carried on the things themselves (used after loads and level swaps).
fn spawn_thinkers(w: &mut WorldState) {
    w.thinkers.clear();
    for (idx, thing) in w.things.iter().enumerate() {
        if thing.is_monster() || thing.is_barrel() {
            let state_idx = thing.state_idx;
            let tics = if thing.ai_timer > 0 {
                thing.ai_timer as i32
            } else {
                crate::doom::STATES[state_idx].duration
            };
            w.thinkers.push(Box::new(crate::doom::MonsterThinker::new(
                idx,
                state_idx,
                tics,
                thing.target_thing_idx,
                thing.attack_cooldown,
            )));
        }
    }
}

fn show_message(world: &mut WorldState, text: &str, color: [u8; 3]) {
    world.hud_messages.push(HudMessage {
        text: text.to_string(),
        timer: 3.0,
        color,
    });
}

/// Load `map_name` and swap it in as the active level, carrying over the
/// player's option settings. Restarts thinkers, static render geometry, music,
/// and the window title. On error the current world is left untouched.
fn switch_level(
    loader: &WadLoader,
    map_name: &str,
    skill: usize,
    world: &mut WorldState,
    renderer: &mut dyn VisualBridge,
    music: &mut Option<MusicEngine>,
    window: &Window,
) -> anyhow::Result<()> {
    let mut new_world = loader.load_map(map_name)?;
    loader.load_textures(&mut new_world)?;
    doom::apply_skill_and_count_totals(&mut new_world, skill);
    new_world.options = world.options;
    new_world.player.fov = world.player.fov;
    *world = new_world;
    spawn_thinkers(world);
    renderer.on_map_loaded(world);
    if let Some(m) = music {
        let _ = m.play_map_music(loader, map_name);
    }
    window.set_title(&format!("Aetheris DOOM \u{2014} {}", map_name));
    log::info!("Level loaded: {}", map_name);
    Ok(())
}

/// Install a deserialized savegame as the active world. Returns false (with
/// the current world untouched) if the save data cannot be parsed.
fn install_loaded_world(
    json: &str,
    world: &mut WorldState,
    renderer: &mut dyn VisualBridge,
) -> bool {
    match serde_json::from_str::<WorldState>(json) {
        Ok(mut loaded) => {
            loaded.textures = world.textures.clone();
            loaded.thinkers.clear();
            loaded.player.fire_cooldown = 0;
            loaded.menu_state = MenuState::None;
            loaded.menu_selection = 0;
            loaded.audio_events.clear();
            *world = loaded;
            spawn_thinkers(world);
            renderer.on_map_loaded(world);
            true
        }
        Err(e) => {
            log::error!("Failed to parse savegame: {}", e);
            false
        }
    }
}

fn save_slot_exists(slot: usize) -> bool {
    std::path::Path::new(&format!("save{}.json", slot)).exists()
}

pub async fn run_game(
    event_loop: EventLoop<()>,
    window: Window,
    warehouse: Box<dyn AssetWarehouse>,
) -> anyhow::Result<()> {
    let window_size = window.inner_size();
    let mut input = InputManager::new();

    let args: Vec<String> = std::env::args().collect();
    let is_golden_test = args.iter().any(|arg| arg == "--golden-test");
    let is_update_goldens = args.iter().any(|arg| arg == "--update-goldens");
    let enable_profiler = args.iter().any(|arg| arg == "--profile");
    let enable_telemetry = args.iter().any(|arg| arg == "--telemetry");
    let is_test_mode = is_golden_test || is_update_goldens;

    let mut target_wad = "freedoom1.wad".to_string();
    let mut wad_was_specified = false;
    if let Some(idx) = args.iter().position(|a| a == "--wad") {
        match args.get(idx + 1) {
            Some(name) if !name.starts_with("--") => {
                target_wad = name.clone();
                wad_was_specified = true;
            }
            _ => anyhow::bail!("--wad requires a file name, e.g. --wad DOOM1.WAD"),
        }
    }
    for arg in args.iter().skip(1) {
        if arg.starts_with("--")
            && !matches!(
                arg.as_str(),
                "--wad"
                    | "--golden-test"
                    | "--update-goldens"
                    | "--profile"
                    | "--telemetry"
                    | "--modern"
            )
        {
            log::warn!("Unknown option '{}' ignored (see --help)", arg);
        }
    }
    if args.iter().any(|arg| arg == "--modern") {
        log::warn!(
            "--modern flag ignored: aetheris_pro WGPU renderer is not included in the open-source release."
        );
    }

    // Initialize the simulation (The World) from WAD via Warehouse
    let wad_data = match warehouse.load_raw(&target_wad).await {
        Ok(data) => {
            log::info!("Loaded {}", target_wad);
            data
        }
        Err(_) => {
            if !wad_was_specified {
                log::info!("freedoom1.wad not found, trying DOOM1.WAD");
                match warehouse.load_raw("DOOM1.WAD").await {
                    Ok(data) => data,
                    Err(_) => anyhow::bail!(
                        "No WAD file found.\n\n\
                         Place freedoom1.wad or DOOM1.WAD in the current directory, or pass\n\
                         --wad <FILE>. Free options:\n\
                         - Freedoom: https://freedoom.github.io/download.html\n\
                         - DOOM shareware: https://www.doomworld.com/idgames/idstuff/doom/doom19s"
                    ),
                }
            } else {
                anyhow::bail!(
                    "Could not read WAD file '{}'. Check the path and try again.",
                    target_wad
                );
            }
        }
    };
    let loader = WadLoader::new(wad_data)?;

    // Try to load DEHACKED patch if present
    let _dehacked_patch = if let Ok(deh_data) = warehouse.load_raw("DOOM1.DEH").await {
        log::info!("Loading DEHACKED patch...");
        match aetheris::assets::dehacked::DehackedPatch::parse(&String::from_utf8_lossy(&deh_data))
        {
            Ok(patch) => {
                log::info!(
                    "DEHACKED patch loaded with {} thing patches, {} weapon patches",
                    patch.things.len(),
                    patch.weapons.len()
                );
                Some(patch)
            }
            Err(e) => {
                log::warn!("Failed to parse DEHACKED patch: {}", e);
                None
            }
        }
    } else {
        None
    };

    // Level management. Detect whether this WAD uses DOOM 1 (ExMy) or
    // DOOM 2 (MAPxx) map naming so both kinds are playable.
    let mut current_episode: u32 = 1;
    let mut current_map_index: u32 = 1;
    let mut current_skill: usize = 2; // Hurt Me Plenty
    let (mut world, map_scheme, mut map_name) = match loader.load_map("E1M1") {
        Ok(w) => (w, MapScheme::Episodic, "E1M1".to_string()),
        Err(_) => match loader.load_map("MAP01") {
            Ok(w) => (w, MapScheme::MapXX, "MAP01".to_string()),
            Err(e) => anyhow::bail!(
                "WAD '{}' contains no E1M1 or MAP01 map lump: {}",
                target_wad,
                e
            ),
        },
    };
    loader.load_textures(&mut world)?;
    doom::apply_skill_and_count_totals(&mut world, current_skill);
    window.set_title(&format!("Aetheris DOOM \u{2014} {}", map_name));

    // Boot into the main menu (the loaded level acts as the backdrop).
    // Test modes skip the menu and drive the game directly.
    if !is_test_mode {
        world.menu_state = MenuState::Main;
        world.menu_selection = 0;
    }

    // Initialize the renderer (The View) - Open Source Version (Software Render Only)
    let mut renderer: Box<dyn VisualBridge> = Box::new(ClassicSoftwareEngine::new(
        &window,
        window_size.width,
        window_size.height,
    )?);

    renderer.on_map_loaded(&world);

    if is_test_mode {
        for i in 0..8 {
            world.player.owned_weapons[i] = true;
        }
        for i in 0..4 {
            world.player.ammo[i] = 500;
        }
    }

    spawn_thinkers(&mut world);

    // Timing
    const TICK_RATE: f32 = 35.0;
    const TICK_DURATION: std::time::Duration =
        std::time::Duration::from_nanos((1_000_000_000.0 / TICK_RATE) as u64);
    let mut last_tick_time = std::time::Instant::now();
    let mut accumulator = std::time::Duration::ZERO;

    let mut cheat_buffer = String::new();

    // Audio
    let sound_data = loader.load_sounds();
    let mut audio: Box<dyn AudioBridge> =
        match aetheris::infrastructure::audio::SampleAudioEngine::new_with_wad_sounds(sound_data) {
            Ok(engine) => {
                log::info!("✅ Audio initialized successfully");
                Box::new(engine)
            }
            Err(e) => {
                log::error!("❌ Audio initialization failed: {:?}", e);
                log::error!("   Falling back to NullAudioEngine (no sound)");
                log::error!("   Possible causes:");
                log::error!("   1. No audio output device available");
                log::error!("   2. WAD file contains invalid/missing sound lumps");
                log::error!("   3. Rodio cannot initialize on this platform");
                log::error!("   4. Sound format parsing failed (DMX header issue)");
                Box::new(aetheris::infrastructure::audio::NullAudioEngine)
            }
        };

    // Music
    let mut music = if let Some(handle) = audio.handle() {
        match MusicEngine::new(handle) {
            Ok(mut m) => {
                if let Err(e) = m.play_map_music(&loader, &map_name) {
                    log::warn!("Music: Failed to start track: {:?}", e);
                }
                m.set_volume(world.options.music_volume);
                Some(m)
            }
            Err(e) => {
                log::warn!("Music: Failed to initialize MusicEngine: {:?}", e);
                None
            }
        }
    } else {
        println!("Music: [ERROR] Audio handle missing!");
        None
    };

    let mut profiler = aetheris::infrastructure::PerformanceProfiler::new();
    let mut telemetry = aetheris::infrastructure::Telemetry::new();

    // Intermission state
    let mut intermission_timer = 0.0f32;
    let mut intermission_advance = false;

    // Mouse state
    let mut mouse_dx = 0.0f32;
    let mut cursor_captured = false;
    let mut window_focused = true;

    let mut last_golden: i32 = -1;
    let mut exiting = false;

    event_loop.run(move |event, _, control_flow| {
        if exiting { return; }

        match event {
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                log::info!("Exit requested via window close.");
                exiting = true;
                *control_flow = ControlFlow::Exit;
            }
            Event::WindowEvent { event: WindowEvent::Resized(new_size), .. } => {
                renderer.handle_resize(new_size.width, new_size.height, false);
            }
            Event::WindowEvent { event: WindowEvent::Focused(focused), .. } => {
                window_focused = focused;
                if !focused {
                    input.pressed_keys.clear();
                }
            }
            Event::DeviceEvent { event: DeviceEvent::MouseMotion { delta }, .. } => {
                mouse_dx += delta.0 as f32;
            }
            Event::WindowEvent { event: WindowEvent::MouseInput { state, button: MouseButton::Left, .. }, .. } => {
                if state == ElementState::Pressed {
                    if world.is_intermission {
                        if intermission_timer > 0.7 {
                            intermission_advance = true;
                        }
                    } else if world.menu_state == MenuState::None {
                        // Left mouse button acts as a fire key.
                        input.pressed_keys.insert(VirtualKeyCode::LControl);
                    }
                } else {
                    input.pressed_keys.remove(&VirtualKeyCode::LControl);
                }
            }
            Event::WindowEvent { event: WindowEvent::KeyboardInput { input: keyboard_input, .. }, .. } => {
                let state = keyboard_input.state;
                let mut mapped_key = keyboard_input.virtual_keycode;
                #[cfg(target_os = "macos")]
                {
                    if keyboard_input.scancode == 53 {
                        mapped_key = Some(VirtualKeyCode::Escape);
                    } else if mapped_key.is_none() {
                        match keyboard_input.scancode {
                            36 => mapped_key = Some(VirtualKeyCode::Return),
                            49 => mapped_key = Some(VirtualKeyCode::Space),
                            _ => {}
                        }
                    }
                }
                #[cfg(not(target_os = "macos"))]
                {
                    if mapped_key.is_none() {
                        match keyboard_input.scancode {
                            1 => mapped_key = Some(VirtualKeyCode::Escape), // Generic fallback
                            _ => {}
                        }
                    }
                }

                let Some(key) = mapped_key else { return; };
                log::debug!(
                    "Key input: {:?} {:?} (scancode {})",
                    key,
                    state,
                    keyboard_input.scancode
                );
                if state != ElementState::Pressed {
                    input.pressed_keys.remove(&key);
                    return;
                }
                if !input.pressed_keys.insert(key) { return; }

                if key == VirtualKeyCode::F12 {
                    let timestamp = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_secs();
                    let filename = format!("screenshot_{}.png", timestamp);
                    if renderer.take_screenshot(&filename).is_ok() {
                        show_message(&mut world, &format!("Screenshot saved: {}", filename), [0, 255, 0]);
                    }
                }

                if world.menu_state == MenuState::None {
                    // --- Cheat code detection (gameplay only) ---
                    let key_char = match key {
                        VirtualKeyCode::A => Some('a'), VirtualKeyCode::B => Some('b'), VirtualKeyCode::C => Some('c'),
                        VirtualKeyCode::D => Some('d'), VirtualKeyCode::E => Some('e'), VirtualKeyCode::F => Some('f'),
                        VirtualKeyCode::G => Some('g'), VirtualKeyCode::H => Some('h'), VirtualKeyCode::I => Some('i'),
                        VirtualKeyCode::J => Some('j'), VirtualKeyCode::K => Some('k'), VirtualKeyCode::L => Some('l'),
                        VirtualKeyCode::M => Some('m'), VirtualKeyCode::N => Some('n'), VirtualKeyCode::O => Some('o'),
                        VirtualKeyCode::P => Some('p'), VirtualKeyCode::Q => Some('q'), VirtualKeyCode::R => Some('r'),
                        VirtualKeyCode::S => Some('s'), VirtualKeyCode::T => Some('t'), VirtualKeyCode::U => Some('u'),
                        VirtualKeyCode::V => Some('v'), VirtualKeyCode::W => Some('w'), VirtualKeyCode::X => Some('x'),
                        VirtualKeyCode::Y => Some('y'), VirtualKeyCode::Z => Some('z'),
                        VirtualKeyCode::Key0 => Some('0'), VirtualKeyCode::Key1 => Some('1'),
                        VirtualKeyCode::Key2 => Some('2'), VirtualKeyCode::Key3 => Some('3'),
                        VirtualKeyCode::Key4 => Some('4'), VirtualKeyCode::Key5 => Some('5'),
                        VirtualKeyCode::Key6 => Some('6'), VirtualKeyCode::Key7 => Some('7'),
                        VirtualKeyCode::Key8 => Some('8'), VirtualKeyCode::Key9 => Some('9'),
                        _ => None,
                    };
                    if let Some(c) = key_char {
                        cheat_buffer.push(c);
                        if cheat_buffer.len() > 10 { cheat_buffer.remove(0); }

                        if cheat_buffer.ends_with("iddqd") {
                            if world.player.invuln_timer > 0 {
                                world.player.invuln_timer = 0;
                                show_message(&mut world, "Degreelessness Mode Off", [255, 0, 0]);
                            } else {
                                world.player.health = 100.0;
                                world.player.invuln_timer = u32::MAX;
                                show_message(&mut world, "Degreelessness Mode On", [255, 255, 0]);
                            }
                            cheat_buffer.clear();
                        } else if cheat_buffer.ends_with("idkfa") {
                            for i in 0..8 { world.player.owned_weapons[i] = true; }
                            for i in 0..3 { world.player.keys[i] = true; }
                            for i in 0..4 { world.player.ammo[i] = 500; }
                            show_message(&mut world, "Very Happy Ammo Added!", [0, 255, 0]);
                            cheat_buffer.clear();
                        } else if cheat_buffer.ends_with("idfa") {
                            for i in 0..8 { world.player.owned_weapons[i] = true; }
                            for i in 0..4 { world.player.ammo[i] = 500; }
                            show_message(&mut world, "Ammo (no keys) Added!", [0, 255, 0]);
                            cheat_buffer.clear();
                        } else if cheat_buffer.len() >= 8 {
                            let (head, digits) = cheat_buffer.split_at(cheat_buffer.len() - 2);
                            if head.ends_with("idclev") && digits.chars().all(|d| d.is_ascii_digit()) {
                                let d1 = digits.as_bytes()[0] - b'0';
                                let d2 = digits.as_bytes()[1] - b'0';
                                let (ep, mp) = match map_scheme {
                                    MapScheme::Episodic => (d1 as u32, d2 as u32),
                                    MapScheme::MapXX => (1, (d1 * 10 + d2) as u32),
                                };
                                let name = map_lump_name(map_scheme, ep, mp);
                                match switch_level(&loader, &name, current_skill, &mut world, renderer.as_mut(), &mut music, &window) {
                                    Ok(()) => {
                                        current_episode = ep;
                                        current_map_index = mp;
                                        map_name = name;
                                        world.is_intermission = false;
                                        intermission_timer = 0.0;
                                        show_message(&mut world, "Changing Level...", [0, 255, 0]);
                                    }
                                    Err(_) => show_message(&mut world, &format!("No such level: {}", name), [255, 0, 0]),
                                }
                                cheat_buffer.clear();
                            }
                        }
                    }

                    // --- Gameplay function keys ---
                    match key {
                        VirtualKeyCode::Escape => {
                            world.menu_state = MenuState::Main;
                            world.menu_selection = 0;
                        }
                        VirtualKeyCode::Tab => {
                            world.is_automap = !world.is_automap;
                        }
                        VirtualKeyCode::F2 => {
                            if world.player.health > 0.0 {
                                world.menu_state = MenuState::SaveGame;
                                world.menu_selection = 0;
                            }
                        }
                        VirtualKeyCode::F3 => {
                            world.menu_state = MenuState::LoadGame;
                            world.menu_selection = 0;
                        }
                        VirtualKeyCode::F5 => {
                            match aetheris::infrastructure::savegame::io::quick_save(&world) {
                                Ok(_) => show_message(&mut world, "Quick save complete.", [0, 255, 0]),
                                Err(e) => {
                                    log::error!("Quick save failed: {}", e);
                                    show_message(&mut world, "Quick save FAILED (see log).", [255, 0, 0]);
                                }
                            }
                        }
                        VirtualKeyCode::F9 => {
                            match aetheris::infrastructure::savegame::io::quick_load() {
                                Ok(json) => {
                                    if install_loaded_world(&json, &mut world, renderer.as_mut()) {
                                        intermission_timer = 0.0;
                                        show_message(&mut world, "Quick load complete.", [0, 255, 0]);
                                    } else {
                                        show_message(&mut world, "Quick load FAILED (see log).", [255, 0, 0]);
                                    }
                                }
                                Err(_) => show_message(&mut world, "No quicksave found. Press F5 to quicksave.", [255, 255, 0]),
                            }
                        }
                        _ => {}
                    }

                    if world.is_intermission
                        && intermission_timer > 0.7
                        && matches!(
                            key,
                            VirtualKeyCode::Space | VirtualKeyCode::Return | VirtualKeyCode::NumpadEnter
                                | VirtualKeyCode::LControl | VirtualKeyCode::RControl
                        )
                    {
                        intermission_advance = true;
                    }
                    window.request_redraw();
                    return;
                }

                // --- Menu input ---
                let menu_len = match world.menu_state {
                    MenuState::Main => 4,
                    MenuState::EpisodeSelect => 1,
                    MenuState::DifficultySelect => 5,
                    MenuState::Options => 2,
                    MenuState::LoadGame | MenuState::SaveGame => 6,
                    MenuState::None => unreachable!(),
                };
                if matches!(key, VirtualKeyCode::Up | VirtualKeyCode::W) {
                    world.menu_selection = (world.menu_selection + menu_len - 1) % menu_len;
                }
                if matches!(key, VirtualKeyCode::Down | VirtualKeyCode::S) {
                    world.menu_selection = (world.menu_selection + 1) % menu_len;
                }
                let confirmed = matches!(key, VirtualKeyCode::Return | VirtualKeyCode::NumpadEnter | VirtualKeyCode::Space);

                match world.menu_state {
                    MenuState::Main => {
                        if key == VirtualKeyCode::Escape {
                            world.menu_state = MenuState::None;
                        } else if confirmed {
                            match world.menu_selection {
                                0 => {
                                    // DOOM 2 WADs have no episodes; go straight to skill select.
                                    if map_scheme == MapScheme::Episodic {
                                        world.menu_state = MenuState::EpisodeSelect;
                                        world.menu_selection = 0;
                                    } else {
                                        world.menu_state = MenuState::DifficultySelect;
                                        world.menu_selection = 2;
                                    }
                                }
                                1 => { world.menu_state = MenuState::LoadGame; world.menu_selection = 0; }
                                2 => {
                                    world.menu_state = MenuState::Options;
                                    world.menu_selection = 0;
                                    show_message(&mut world, "UP/DOWN to select, LEFT/RIGHT to adjust", [255, 255, 0]);
                                }
                                3 => { exiting = true; *control_flow = ControlFlow::Exit; }
                                _ => {}
                            }
                        }
                    }
                    MenuState::EpisodeSelect => {
                        if key == VirtualKeyCode::Escape {
                            world.menu_state = MenuState::Main;
                            world.menu_selection = 0;
                        } else if confirmed {
                            world.menu_state = MenuState::DifficultySelect;
                            world.menu_selection = 2; // Default to 'Hurt Me Plenty'
                        }
                    }
                    MenuState::DifficultySelect => {
                        if key == VirtualKeyCode::Escape {
                            if map_scheme == MapScheme::Episodic {
                                world.menu_state = MenuState::EpisodeSelect;
                                world.menu_selection = 0;
                            } else {
                                world.menu_state = MenuState::Main;
                                world.menu_selection = 0;
                            }
                        } else if confirmed {
                            current_skill = world.menu_selection.min(4);
                            let name = map_lump_name(map_scheme, 1, 1);
                            match switch_level(&loader, &name, current_skill, &mut world, renderer.as_mut(), &mut music, &window) {
                                Ok(()) => {
                                    current_episode = 1;
                                    current_map_index = 1;
                                    map_name = name;
                                    world.menu_state = MenuState::None;
                                    world.menu_selection = 0;
                                    intermission_timer = 0.0;
                                    log::info!("New game started: {} at skill {}", map_name, current_skill);
                                }
                                Err(e) => {
                                    log::error!("Failed to load map for new game: {}", e);
                                    world.menu_state = MenuState::None;
                                    show_message(&mut world, "Failed to start new game (see log).", [255, 0, 0]);
                                }
                            }
                        }
                    }
                    MenuState::Options => {
                        if key == VirtualKeyCode::Escape {
                            world.menu_state = MenuState::Main;
                            world.menu_selection = 2;
                        } else {
                            let dir: i32 = match key {
                                VirtualKeyCode::Left => -1,
                                VirtualKeyCode::Right => 1,
                                _ if confirmed => 1,
                                _ => 0,
                            };
                            if dir != 0 {
                                match world.menu_selection {
                                    0 => {
                                        // Sound: master volume for SFX and music.
                                        let v = (world.options.sfx_volume as i32 + dir * 10).clamp(0, 100) as u32;
                                        world.options.sfx_volume = v;
                                        world.options.music_volume = v;
                                        show_message(&mut world, &format!("Volume: {}%", v), [255, 255, 0]);
                                    }
                                    1 => {
                                        // Video: field of view.
                                        let fov = (world.player.fov as i32 + dir * 5).clamp(60, 110) as f32;
                                        world.player.fov = fov;
                                        show_message(&mut world, &format!("Field of View: {} degrees", fov), [255, 255, 0]);
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                    MenuState::LoadGame | MenuState::SaveGame => {
                        let saving = world.menu_state == MenuState::SaveGame;
                        if key == VirtualKeyCode::Escape {
                            world.menu_state = if saving { MenuState::None } else { MenuState::Main };
                            world.menu_selection = if saving { 0 } else { 1 };
                        } else if confirmed {
                            let slot = world.menu_selection + 1;
                            let filename = format!("save{}.json", slot);
                            if saving {
                                match aetheris::infrastructure::savegame::io::save_with_checksum(&filename, &world) {
                                    Ok(_) => {
                                        world.menu_state = MenuState::None;
                                        world.menu_selection = 0;
                                        show_message(&mut world, &format!("Game saved to slot {}.", slot), [0, 255, 0]);
                                    }
                                    Err(e) => {
                                        log::error!("Save failed: {}", e);
                                        show_message(&mut world, "Save FAILED (see log).", [255, 0, 0]);
                                    }
                                }
                            } else if !save_slot_exists(slot) {
                                show_message(&mut world, &format!("Slot {} is empty.", slot), [255, 255, 0]);
                            } else {
                                match aetheris::infrastructure::savegame::io::load_with_checksum(&filename) {
                                    Ok(json) => {
                                        if install_loaded_world(&json, &mut world, renderer.as_mut()) {
                                            intermission_timer = 0.0;
                                            show_message(&mut world, &format!("Game loaded from slot {}.", slot), [0, 255, 0]);
                                        } else {
                                            show_message(&mut world, "Load FAILED (see log).", [255, 0, 0]);
                                        }
                                    }
                                    Err(e) => {
                                        log::error!("Load failed: {}", e);
                                        show_message(&mut world, "Load FAILED (see log).", [255, 0, 0]);
                                    }
                                }
                            }
                        }
                    }
                    MenuState::None => unreachable!(),
                }
                window.request_redraw();
            }
            Event::MainEventsCleared => {
                if exiting { return; }
                *control_flow = ControlFlow::Poll;
                let now = std::time::Instant::now();
                let frame_time = now - last_tick_time;
                last_tick_time = now;

                // Capture the mouse during gameplay; release it for menus.
                let want_capture = world.menu_state == MenuState::None
                    && window_focused
                    && !is_test_mode;
                if want_capture != cursor_captured {
                    cursor_captured = want_capture;
                    if want_capture {
                        let _ = window
                            .set_cursor_grab(CursorGrabMode::Locked)
                            .or_else(|_| window.set_cursor_grab(CursorGrabMode::Confined));
                    } else {
                        let _ = window.set_cursor_grab(CursorGrabMode::None);
                    }
                    window.set_cursor_visible(!want_capture);
                }

                // Mouse look (horizontal turn). Consume the delta every frame
                // so stale motion never leaks into a later game state.
                let dx = std::mem::take(&mut mouse_dx);
                if cursor_captured
                    && !world.is_paused
                    && !world.is_intermission
                    && world.player.health > 0.0
                {
                    world.player.angle =
                        (world.player.angle - dx * MOUSE_SENSITIVITY).rem_euclid(TAU);
                }

                let actions = input.get_active_actions();
                renderer.handle_input(&actions);

                let current_fps = 1.0 / frame_time.as_secs_f32().max(0.001);
                world.fps = world.fps * 0.9 + current_fps * 0.1;

                // Keep music playing (and volume changes live) even in menus.
                if let Some(m) = &mut music {
                    m.set_volume(world.options.music_volume);
                    m.update(world.frame_count);
                }

                if world.menu_state != MenuState::None {
                    window.request_redraw();
                    return;
                }

                if world.is_intermission {
                    intermission_timer += frame_time.as_secs_f32();
                }

                if intermission_advance {
                    intermission_advance = false;
                    if world.is_intermission {
                        world.is_intermission = false;
                        let (next_episode, next_map) = match map_scheme {
                            MapScheme::Episodic => {
                                if current_map_index >= 9 {
                                    (current_episode + 1, 1)
                                } else {
                                    (current_episode, current_map_index + 1)
                                }
                            }
                            MapScheme::MapXX => (1, current_map_index + 1),
                        };
                        let name = map_lump_name(map_scheme, next_episode, next_map);
                        match switch_level(&loader, &name, current_skill, &mut world, renderer.as_mut(), &mut music, &window) {
                            Ok(()) => {
                                current_episode = next_episode;
                                current_map_index = next_map;
                                map_name = name;
                                intermission_timer = 0.0;
                            }
                            Err(e) => {
                                // No next map: the episode is over. Back to the menu.
                                log::info!("No further map after {} ({}); episode complete.", map_name, e);
                                current_episode = 1;
                                current_map_index = 1;
                                world.menu_state = MenuState::Main;
                                world.menu_selection = 0;
                                show_message(&mut world, "Episode complete! Thanks for playing.", [255, 255, 0]);
                            }
                        }
                        window.request_redraw();
                        return;
                    }
                }

                accumulator += frame_time;
                accumulator = accumulator.min(TICK_DURATION * 5);
                while accumulator >= TICK_DURATION {
                    let start_sim = std::time::Instant::now();
                    let tick_actions = input.get_active_actions();

                    if tick_actions.contains(&GameAction::Pause) {
                        world.is_paused = !world.is_paused;
                        input.pressed_keys.remove(&VirtualKeyCode::P);
                    }

                    if !world.is_paused {
                        // Allow update() even during intermission so it can increment intermission_tic
                        world.update(&tick_actions);

                        if world.is_win {
                            world.is_intermission = true;
                            intermission_timer = 0.0;
                            world.is_win = false;
                        }

                        if !world.is_intermission {
                            audio.update_listener(world.player.position, world.player.angle);
                            let _ = audio.update(&world);
                        }
                        world.audio_events.clear();
                    }

                    accumulator -= TICK_DURATION;
                    profiler.record("simulation_tick", start_sim.elapsed());
                }
                window.request_redraw();
            }
            Event::RedrawRequested(_) => {
                if is_test_mode {
                    let test_cases = [
                        (60, "golden_1", glam::Vec2::new(1056.0, -3616.0), 0.0, aetheris::simulation::WeaponType::Pistol),
                        (90, "golden_2", glam::Vec2::new(1056.0, -3616.0), 0.0, aetheris::simulation::WeaponType::Shotgun),
                        (120, "golden_3", glam::Vec2::new(3072.0, -4736.0), 1.57, aetheris::simulation::WeaponType::Chaingun),
                        (150, "golden_4", glam::Vec2::new(3800.0, -3200.0), -0.78, aetheris::simulation::WeaponType::Fist),
                    ];
                    for (frame, name, pos, angle, weapon) in test_cases {
                        if world.frame_count >= frame && last_golden < frame as i32 {
                            last_golden = frame as i32;
                            log::info!("Golden Case: {} (Frame {})", name, world.frame_count);
                            world.player.position = pos;
                            world.player.angle = angle;
                            if let Some(sid) = world.find_sector_at(pos) {
                                world.player.z = world.sectors[sid].floor_height;
                            }
                            world.player.current_weapon = weapon;
                            world.player.bob_phase = 0.0;

                            // Render the scene before the screen capture.
                            let doom_player = DoomPlayer { player: &world.player };
                            let wrapped_things: Vec<DoomEntity> = world.things.iter().map(|t| DoomEntity { thing: t, world: &world }).collect();
                            let entities: Vec<&dyn aetheris::presentation::AetherisEntity> = wrapped_things.iter().map(|wt| wt as &dyn aetheris::presentation::AetherisEntity).collect();

                            let mut dummy_profiler = aetheris::infrastructure::PerformanceProfiler::new();
                            let _ = renderer.render_scene(&world, &entities, &doom_player, &mut dummy_profiler);
                            let _ = renderer.render_hud(&world);

                            let actual_path = format!("temp_{}.png", name);
                            let golden_path = format!("tests/goldens/{}.png", name);
                            let diff_path = format!("diff_{}.png", name);
                            let _ = renderer.take_screenshot(&actual_path);
                            if is_golden_test {
                                let engine = aetheris::presentation::visual_test::VisualRegressionEngine::new(5);
                                if std::path::Path::new(&golden_path).exists() {
                                    match engine.compare_images(std::path::Path::new(&actual_path), std::path::Path::new(&golden_path), std::path::Path::new(&diff_path)) {
                                        Ok(score) => {
                                            if score > 0.001 { log::error!("GOLDEN TEST FAILED: {} (score: {:.4})", name, score); }
                                            else { log::info!("GOLDEN TEST PASSED: {} (score: {:.4})", name, score); }
                                        }
                                        Err(e) => log::error!("Comparison error: {:?}", e),
                                    }
                                }
                            } else if is_update_goldens {
                                let _ = std::fs::copy(&actual_path, &golden_path);
                            }
                        }
                    }
                    if world.frame_count > 160 { log::info!("Golden tests complete."); *control_flow = ControlFlow::Exit; }
                }

                let doom_player = DoomPlayer { player: &world.player };
                let wrapped_things: Vec<DoomEntity> = world.things.iter().map(|t| DoomEntity { thing: t, world: &world }).collect();
                let entities: Vec<&dyn aetheris::presentation::AetherisEntity> = wrapped_things.iter().map(|wt| wt as &dyn aetheris::presentation::AetherisEntity).collect();

                let _ = renderer.render_scene(&world, &entities, &doom_player, &mut profiler);
                let _ = renderer.render_hud(&world);
                let _ = renderer.render_automap(&world);
                let _ = renderer.present();
                if enable_profiler {
                    profiler.print_histogram();
                } else {
                    profiler.stage_times.clear();
                }
                if enable_telemetry {
                    telemetry.snapshot(&world);
                }
            }
            Event::LoopDestroyed => {
                log::info!("Exiting.");
                #[cfg(not(target_arch = "wasm32"))]
                std::process::exit(0);
            }
            _ => {}
        }
    });
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub async fn wasm_main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(log::Level::Info).expect("Failed to init logger");
    let (event_loop, window) = infrastructure::create_window();
    use winit::platform::web::WindowExtWebSys;
    web_sys::window()
        .and_then(|win| win.document())
        .and_then(|doc| doc.body())
        .and_then(|body| {
            let canvas = window.canvas();
            canvas.set_id("doom-canvas");
            body.append_child(&canvas).ok()
        })
        .expect("Failed to append canvas");
    let warehouse = Box::new(aetheris::assets::WebWarehouse);
    let _ = run_game(event_loop, window, warehouse).await;
}
fn main() -> anyhow::Result<()> {
    if std::env::args().any(|a| a == "--help" || a == "-h") {
        print!("{}", HELP_TEXT);
        return Ok(());
    }
    env_logger::init();
    let (event_loop, window) = aetheris::infrastructure::create_window();
    let warehouse = Box::new(aetheris::assets::FileSystemWarehouse);
    pollster::block_on(run_game(event_loop, window, warehouse))
}
