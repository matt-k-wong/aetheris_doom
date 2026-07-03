# Aetheris DOOM — Architecture Map

Bootstrap notes for future work sessions (human or agent). Diagrams are Mermaid;
GitHub and most IDEs render them inline. Verified against the code on 2026-07-02.

## Crate & module graph

```mermaid
graph TD
    subgraph game["aetheris_doom (this repo)"]
        main["src/main.rs<br/>window + event loop, CLI flags,<br/>menu input, save/load, mouse capture,<br/>level switching (switch_level),<br/>golden-test harness"]
        doom["src/doom.rs (~6400 lines)<br/>DoomWorldExt::update() ~L4090 (player sim),<br/>apply_commands() ~L4800 (command interpreter),<br/>MonsterThinker AI + STATES tables,<br/>apply_skill_and_count_totals() (file end)"]
        bridge["src/bridge.rs<br/>PresentationMapper:<br/>thing state -> sprite name"]
    end

    subgraph engine["aetheris engine (git dep; local override via .cargo/config.toml -> ../aetheris_engine)"]
        sim["simulation/engine.rs<br/>WorldState, Player, Thing, WorldCommand,<br/>GameOptions, MenuState, weapon_ammo_type"]
        wad["assets/wad.rs<br/>WadLoader: load_map, load_textures,<br/>load_sounds (counts total_secrets)"]
        render["presentation/classic_engine.rs<br/>software rasterizer, render_hud<br/>(menus HARD-CODED ~L1176,<br/>intermission tally, messages)"]
        infra["infrastructure/<br/>InputManager (key->GameAction),<br/>savegame::io (checksummed JSON),<br/>music, audio, Telemetry, Profiler"]
    end

    main -->|"world.update(actions) per 35Hz tick"| doom
    main -->|"render_scene / render_hud / present"| render
    main -->|"load_map / load_textures"| wad
    main -->|"quick_save / save_with_checksum"| infra
    doom -->|"emits WorldCommand vec"| sim
    doom -->|"mutates WorldState via DoomWorldExt"| sim
    bridge -->|"reads STATES + Thing"| doom
    render -->|"reads WorldState (menu_state, hud_messages)"| sim
    infra -->|"produces GameAction set"| main
    wad -->|"builds WorldState"| sim
```

## Runtime flow (one frame)

```mermaid
flowchart LR
    A[winit events] --> B{menu_state?}
    B -->|"!= None"| C["menu key handling in main.rs<br/>(labels must match engine render_hud)"]
    B -->|None| D["cheats / F-keys / mouse-look"]
    D --> E["35Hz tick loop:<br/>world.update(actions)"]
    E --> F["apply_commands:<br/>damage, pickups (items_collected),<br/>kills (monsters_killed), doors, msgs"]
    F --> G["is_win -> is_intermission"]
    G --> H["fire/enter after 0.7s -><br/>switch_level(next map)<br/>fail -> back to Main menu"]
    C & E --> I["render_scene + render_hud + present"]
```

## Load-bearing constraints (learned the hard way)

- **Menus are rendered by the engine** with hard-coded titles/labels/counts
  (`classic_engine.rs` `render_hud`). Game-side handlers in `main.rs` must match:
  Main=4, Episode=1, Difficulty=5, Options=2 ("Sound","Video"), Save/Load=6 slots.
- Engine honors `options.sfx_volume`, `options.music_volume`, `player.fov`;
  it **ignores** `gamma`, `screen_size`, `show_fps` (gamma hard-coded 1.2).
- Saves are `WorldState` JSON only — **no map name** (`save_with_checksum` has a
  `TODO`), so episode/map progression counters can't be restored on load.
- `.cargo/config.toml` (gitignored) path-overrides the git dep to `../aetheris_engine`;
  local engine edits don't reach users unless pushed to the engine repo.
- Skill filtering uses WAD thing flags (0x1 easy / 0x2 medium / 0x4 hard,
  0x10 = multiplayer-only) via `doom::apply_skill_and_count_totals`, called on
  every level (re)load in `main.rs::switch_level`.
- `--golden-test` / `--update-goldens` skip the boot menu and drive fixed camera
  shots; no baselines are committed, CI doesn't run them.

## Deferred work (pick up when budget allows)

1. **Engine:** add map name to `WorldState`/save format (fixes progression-after-load);
   move menu labels out of `render_hud` so the game owns its menus.
2. Secret exits (linedef special 52) → E1M9; currently progression is linear.
3. Clippy: many warnings, CI non-blocking; dead-code sweep in `doom.rs` constants.
4. DEHACKED patch is parsed but never applied.
5. Vertical mouse aim / weapon-bob polish; menu navigation sounds (needs audio
   update while menu open — sim tick is skipped when a menu is up).
