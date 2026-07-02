use aetheris::simulation::*;
use glam::Vec2;

use super::defs::DoomThingExt;
use super::world::DoomWorldExt;

pub(crate) fn is_walk_trigger(special: u16) -> bool {
    matches!(
        special,
        // W1 types
        2 | 3 | 4 | 5 | 16 | 38 | 39 | 44 | 52 | 56 | 58 | 59 |
        // WR types
        72 | 73 | 74 | 75 | 76 | 77 | 79 | 80 | 86 | 87 | 88 | 90 | 91 |
        97 | 105 | 106 | 107 | 120 | 126 | 128 | 129
    )
}

pub(crate) fn activate_linedef_manual(
    world: &mut WorldState,
    line_idx: usize,
    override_back: Option<usize>,
    cmds: &mut Vec<WorldCommand>,
) {
    if line_idx >= world.linedefs.len() {
        return;
    }
    let (special, tag, activated) = {
        let line = &world.linedefs[line_idx];
        (line.special_type, line.sector_tag, line.activated)
    };

    let is_repeatable = matches!(special,
        1 | 117 | 26 | 27 | 28 | 32 | 33 | 34 |
        72..=80 | 86..=88 | 90 | 91 | 97 | 105..=107 | 120 | 126 | 128 | 129 |
        11 | 51 | 52 | 100 | 127 | 141 | 48
    );

    if activated && !is_repeatable {
        return;
    }

    let mut changed_tex = false;
    {
        let line = &mut world.linedefs[line_idx];
        let check_and_toggle = |tex: &mut Option<String>| {
            if let Some(t) = tex {
                let up = t.to_uppercase();
                if up.starts_with("SW1") {
                    *t = t.replace("SW1", "SW2");
                    true
                } else if up.starts_with("SW2") {
                    *t = t.replace("SW2", "SW1");
                    true
                } else {
                    false
                }
            } else {
                false
            }
        };
        if let Some(front) = &mut line.front {
            if check_and_toggle(&mut front.texture_middle) {
                changed_tex = true;
            }
            if check_and_toggle(&mut front.texture_upper) {
                changed_tex = true;
            }
            if check_and_toggle(&mut front.texture_lower) {
                changed_tex = true;
            }
        }
    }

    if changed_tex {
        world.audio_events.push(AudioEvent {
            sound_id: "DSSWTCHN".into(),
            position: Some(world.player.position),
            volume: 1.0,
        });
    }

    activate_linedef(world, special, tag, override_back, cmds);
    world.linedefs[line_idx].activated = true;
}

pub(crate) fn activate_linedef(
    world: &mut WorldState,
    special: u16,
    tag: u16,
    sector_back: Option<usize>,
    cmds: &mut Vec<WorldCommand>,
) {
    log::info!("Activated Linedef Special: {} Tag: {}", special, tag);

    match special {
        1 | 117 | 31 | 118 | 46 | 103 | 61 | 114 | 115 => {
            let (speed, wait) = match special {
                118 | 114 | 115 => (16.0, 4.0),
                _ => (4.0, 4.0),
            };

            // DR Doors: If tag is 0, it affects the sector on the other side of the line.
            // If tag is NOT 0, it affects all sectors with that tag.
            if tag == 0 {
                if let Some(sid) = sector_back {
                    if trigger_door(world, sid, speed, wait) {
                        let sound = if speed > 4.0 { "DSBDOPN" } else { "DSDOROPN" };
                        world.audio_events.push(AudioEvent {
                            sound_id: sound.into(),
                            position: Some(world.player.position),
                            volume: 1.0,
                        });
                    }
                }
            } else {
                if do_door_tagged(world, tag, speed, wait) {
                    let sound = if speed > 4.0 { "DSBDOPN" } else { "DSDOROPN" };
                    world.audio_events.push(AudioEvent {
                        sound_id: sound.into(),
                        position: Some(world.player.position),
                        volume: 1.0,
                    });
                }
            }
        }
        11 | 51 | 52 => {
            // Exit Level
            log::info!("EXIT LEVEL ACTIVATED!");
            world.is_intermission = true;
            world.intermission_tic = 0;
            world.audio_events.push(AudioEvent {
                sound_id: "DSPISTOL".into(),
                position: None,
                volume: 1.0,
            });
        }
        26 | 32 => {
            if world.player.keys[1] {
                let (speed, wait) = (2.0, 4.0);
                if tag == 0 {
                    if let Some(sid) = sector_back {
                        if trigger_door(world, sid, speed, wait) {
                            world.audio_events.push(AudioEvent {
                                sound_id: "DSDOROPN".into(),
                                position: Some(world.player.position),
                                volume: 1.0,
                            });
                        }
                    }
                } else {
                    if do_door_tagged(world, tag, speed, wait) {
                        world.audio_events.push(AudioEvent {
                            sound_id: "DSDOROPN".into(),
                            position: Some(world.player.position),
                            volume: 1.0,
                        });
                    }
                }
            } else {
                world.audio_events.push(AudioEvent {
                    sound_id: "DSOOF".into(),
                    position: Some(world.player.position),
                    volume: 1.0,
                });
                log::info!("Blue Key Required!");
            }
        }
        27 | 34 => {
            if world.player.keys[2] {
                let (speed, wait) = (2.0, 4.0);
                if tag == 0 {
                    if let Some(sid) = sector_back {
                        if trigger_door(world, sid, speed, wait) {
                            world.audio_events.push(AudioEvent {
                                sound_id: "DSDOROPN".into(),
                                position: Some(world.player.position),
                                volume: 1.0,
                            });
                        }
                    }
                } else {
                    if do_door_tagged(world, tag, speed, wait) {
                        world.audio_events.push(AudioEvent {
                            sound_id: "DSDOROPN".into(),
                            position: Some(world.player.position),
                            volume: 1.0,
                        });
                    }
                }
            } else {
                world.audio_events.push(AudioEvent {
                    sound_id: "DSOOF".into(),
                    position: Some(world.player.position),
                    volume: 1.0,
                });
                log::info!("Yellow Key Required!");
            }
        }
        28 | 33 => {
            if world.player.keys[0] {
                let (speed, wait) = (2.0, 4.0);
                if tag == 0 {
                    if let Some(sid) = sector_back {
                        if trigger_door(world, sid, speed, wait) {
                            world.audio_events.push(AudioEvent {
                                sound_id: "DSDOROPN".into(),
                                position: Some(world.player.position),
                                volume: 1.0,
                            });
                        }
                    }
                } else {
                    if do_door_tagged(world, tag, speed, wait) {
                        world.audio_events.push(AudioEvent {
                            sound_id: "DSDOROPN".into(),
                            position: Some(world.player.position),
                            volume: 1.0,
                        });
                    }
                }
            } else {
                world.audio_events.push(AudioEvent {
                    sound_id: "DSOOF".into(),
                    position: Some(world.player.position),
                    volume: 1.0,
                });
                log::info!("Red Key Required!");
            }
        }
        88 => {
            do_lift_tagged(world, tag);
            world.audio_events.push(AudioEvent {
                sound_id: "DSPSTART".into(),
                position: Some(world.player.position),
                volume: 1.0,
            });
        }
        39 | 97 => {
            // Teleport (W1 / WR)
            let mut best_dest = None;
            for (s_idx, s) in world.sectors.iter().enumerate() {
                if s.tag == tag as i16 {
                    for t in &world.things {
                        if t.kind == 14 {
                            let ss_idx = world.find_subsector(t.position.x, t.position.y);
                            if let Some(ss) = world.subsectors.get(ss_idx) {
                                if let Some(seg) = world.segs.get(ss.first_seg_idx) {
                                    if let Some(sid) = world.linedefs[seg.linedef_idx].sector_front
                                    {
                                        if sid == s_idx {
                                            best_dest = Some(t);
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    if best_dest.is_some() {
                        break;
                    }
                }
            }

            if let Some(dest) = best_dest {
                // TELEFRAG nearby things at destination
                for i in 0..world.things.len() {
                    let dist = (world.things[i].position - dest.position).length();
                    if dist < 32.0 {
                        cmds.push(WorldCommand::DamageThing {
                            thing_idx: i,
                            amount: 10000.0,
                            inflictor_idx: None,
                        });
                    }
                }
                world.player.position = dest.position;
                world.player.angle = dest.angle;
                world.player.velocity = Vec2::ZERO;
                // Snap to destination sector floor
                if let Some(s_idx) = world.find_sector_at(dest.position) {
                    world.player.z = world.sectors[s_idx].floor_height;
                }
                world.audio_events.push(AudioEvent {
                    sound_id: "DSTELEPT".into(),
                    position: Some(world.player.position),
                    volume: 1.0,
                });
            }
        }
        6 | 25 | 77 | 141 => {
            // Crusher Specials
            let (speed, damage) = match special {
                6 | 77 => (2.0, 100.0),
                25 => (0.5, 10.0),
                141 => (0.5, 1000.0),
                _ => (2.0, 100.0),
            };
            do_crusher_tagged(world, tag, speed, damage);
            world.audio_events.push(AudioEvent {
                sound_id: "DSPSTART".into(),
                position: Some(world.player.position),
                volume: 1.0,
            });
        }
        8 | 127 | 100 => {
            // Stair Specials
            let step = if special == 8 { 8.0 } else { 16.0 };
            do_stairs_tagged(world, tag, step);
            world.audio_events.push(AudioEvent {
                sound_id: "DSPSTART".into(),
                position: Some(world.player.position),
                volume: 1.0,
            });
        }
        48 => {
            // Scrolling Texture (Left) - Handled globally in some engines, but we can flag it
            log::info!("Sidedef scrolling (Special 48) active for tag {}", tag);
        }
        _ => {
            log::warn!("Unimplemented Special: {}", special);
        }
    }
}

pub(crate) fn find_lowest_adjacent_ceiling(world: &WorldState, sector_idx: usize) -> f32 {
    let mut min_ceil = f32::MAX;
    let mut found = false;
    let floor = world.sectors[sector_idx].floor_height;

    if let Some(adjs) = world.adjacent_sectors.get(sector_idx) {
        for &adj_idx in adjs {
            if adj_idx < world.sectors.len() {
                let adj_ceil = world.sectors[adj_idx].ceiling_height;
                // Standard Doom: Only consider adjacent ceilings that are actually above the current floor
                // to avoid getting stuck by a neighboring closed door.
                if adj_ceil > floor + 16.0 {
                    // Increased threshold to skip closed doors more reliably
                    if adj_ceil < min_ceil {
                        min_ceil = adj_ceil;
                        found = true;
                    }
                }
            }
        }
    }

    // Safety: Ensure doors ALWAYS open to at least 88 units above their floor.
    // Standard player height is 56, so 88 gives plenty of room.
    let min_safe_height = floor + 88.0;

    if !found || min_ceil < min_safe_height {
        log::info!(
            "WadLoader: No suitable high adjacent ceiling for sector {}, using safe height {}",
            sector_idx,
            min_safe_height
        );
        min_safe_height
    } else {
        min_ceil
    }
}

pub(crate) fn trigger_door(
    world: &mut WorldState,
    sector_idx: usize,
    speed: f32,
    wait: f32,
) -> bool {
    let (floor, ceil, action) = {
        let s = &world.sectors[sector_idx];
        (s.floor_height, s.ceiling_height, s.action.clone())
    };
    log::info!(
        "DEBUG: trigger_door called for sector {} (ceil={}, floor={}, action={:?})",
        sector_idx,
        ceil,
        floor,
        action
    );

    match action {
        SectorAction::None => {
            if ceil <= floor + 4.0 {
                // Opening
                let target = find_lowest_adjacent_ceiling(world, sector_idx) - 4.0;
                log::info!(
                    "DEBUG: Door opening in sector {} to height {}",
                    sector_idx,
                    target
                );
                world.sectors[sector_idx].action = SectorAction::Door {
                    state: DoorState::Opening,
                    wait_timer: wait,
                    speed,
                    open_height: target,
                    close_height: floor,
                };
                return true;
            } else {
                // Closing
                log::info!("DEBUG: Door closing in sector {}", sector_idx);
                world.sectors[sector_idx].action = SectorAction::Door {
                    state: DoorState::Closing,
                    wait_timer: 0.0,
                    speed,
                    open_height: ceil,
                    close_height: floor,
                };
                return true;
            }
        }
        SectorAction::Door {
            state,
            close_height,
            open_height,
            ..
        } => match state {
            DoorState::Waiting => {
                log::info!("DEBUG: Door closing early in sector {}", sector_idx);
                world.sectors[sector_idx].action = SectorAction::Door {
                    state: DoorState::Closing,
                    wait_timer: 0.0,
                    speed,
                    open_height,
                    close_height,
                };
                return true;
            }
            DoorState::Closing => {
                log::info!("DEBUG: Door reversing to open in sector {}", sector_idx);
                world.sectors[sector_idx].action = SectorAction::Door {
                    state: DoorState::Opening,
                    wait_timer: wait,
                    speed,
                    open_height,
                    close_height,
                };
                return true;
            }
            _ => {
                log::info!(
                    "DEBUG: Door in sector {} is already busy in state {:?}",
                    sector_idx,
                    state
                );
                false
            }
        },
        _ => {
            log::info!(
                "DEBUG: Sector {} is busy with non-door action: {:?}",
                sector_idx,
                action
            );
            false
        }
    }
}

pub(crate) fn do_door_tagged(world: &mut WorldState, tag: u16, speed: f32, wait: f32) -> bool {
    let mut triggered = false;
    for i in 0..world.sectors.len() {
        if world.sectors[i].tag == tag as i16 {
            if trigger_door(world, i, speed, wait) {
                triggered = true;
            }
        }
    }
    triggered
}

pub(crate) fn do_lift_tagged(world: &mut WorldState, tag: u16) {
    for s in &mut world.sectors {
        if s.tag == tag as i16 {
            if let SectorAction::None = s.action {
                let target = s.floor_height - 72.0;
                s.action = SectorAction::Lift {
                    state: LiftState::GoingDown,
                    wait_timer: 3.0,
                    speed: 3.0,
                    low_height: target,
                    high_height: s.floor_height,
                };
            }
        }
    }
}

pub(crate) fn do_crusher_tagged(world: &mut WorldState, tag: u16, speed: f32, damage: f32) {
    for s in &mut world.sectors {
        if s.tag == tag as i16 {
            if let SectorAction::None = s.action {
                s.action = SectorAction::Crusher {
                    state: CrusherState::GoingDown,
                    speed,
                    low_height: s.floor_height + 8.0,
                    high_height: s.ceiling_height,
                    damage,
                };
            }
        }
    }
}

pub(crate) fn do_stairs_tagged(world: &mut WorldState, tag: u16, step_height: f32) {
    // Vanilla Doom stair building: start from tagged sector(s), then chain
    // adjacent sectors that share the same floor texture.
    let mut start_sectors = Vec::new();
    for i in 0..world.sectors.len() {
        if world.sectors[i].tag == tag as i16 {
            start_sectors.push(i);
        }
    }

    for start_sid in start_sectors {
        let floor_tex = world.sectors[start_sid].texture_floor.clone();
        let mut current_height = world.sectors[start_sid].floor_height + step_height;
        world.sectors[start_sid].action = SectorAction::FloorMove {
            target_height: current_height,
            speed: 2.0,
        };

        let mut current_sid = start_sid;
        let mut visited = std::collections::HashSet::new();
        visited.insert(start_sid);

        // Chain adjacent sectors with same floor texture
        loop {
            let mut next_sid = None;
            for line in &world.linedefs {
                let (fs, bs) = match (line.sector_front, line.sector_back) {
                    (Some(f), Some(b)) => (f, b),
                    _ => continue,
                };
                let neighbor = if fs == current_sid {
                    bs
                } else if bs == current_sid {
                    fs
                } else {
                    continue;
                };
                if visited.contains(&neighbor) {
                    continue;
                }
                if world.sectors[neighbor].texture_floor == floor_tex {
                    next_sid = Some(neighbor);
                    break;
                }
            }
            match next_sid {
                Some(sid) => {
                    visited.insert(sid);
                    current_height += step_height;
                    world.sectors[sid].action = SectorAction::FloorMove {
                        target_height: current_height,
                        speed: 2.0,
                    };
                    current_sid = sid;
                }
                None => break,
            }
        }
    }
}

pub(crate) fn update_environmental_damage(world: &mut WorldState) {
    // 1. Secret Detection (Every frame)
    if let Some(s_idx) = world.find_sector_at(world.player.position) {
        if s_idx < world.sectors.len() {
            let sector = &mut world.sectors[s_idx];
            if sector.special_type == 9 && !sector.secret_found {
                sector.secret_found = true;
                world.secrets_found += 1;
                log::info!("SECRET FOUND in sector {}!", s_idx);
                world.hud_messages.push(HudMessage {
                    text: "SECRET FOUND!".into(),
                    timer: 2.0,
                    color: [255, 255, 0],
                });
                world.audio_events.push(AudioEvent {
                    sound_id: "DSGETPOW".into(),
                    position: None,
                    volume: 1.0,
                });
            }
        }
    }

    // 2. Damage (Every 32 frames)
    if world.frame_count % 32 != 0 {
        return;
    }

    let mut damage_targets = Vec::new();

    // Check Player
    if let Some(s_idx) = world.find_sector_at(world.player.position) {
        if s_idx < world.sectors.len() {
            let special = world.sectors[s_idx].special_type;
            let damage = match special {
                5 => 10,
                7 => 5,
                16 => 20,
                4 => 20,
                11 => 20,
                _ => 0,
            };

            if damage > 0 {
                if world.player.radsuit_timer == 0 {
                    log::info!(
                        "Player taking slime damage: {} (Sector Special {})",
                        damage,
                        special
                    );
                    damage_targets.push((true, 0, damage as f32));
                }
            }
        }
    }

    // Check Monsters/Barrels
    for (i, t) in world.things.iter().enumerate() {
        if t.health <= 0.0 || t.picked_up || (!t.is_monster() && !t.is_barrel()) {
            continue;
        }
        if let Some(s_idx) = world.find_sector_at(t.position) {
            if s_idx < world.sectors.len() {
                let special = world.sectors[s_idx].special_type;
                let damage = match special {
                    5 | 7 | 16 | 4 | 11 => 5,
                    _ => 0,
                };
                if damage > 0 {
                    log::info!(
                        "Thing {} taking slime damage: {} (Sector Special {})",
                        i,
                        damage,
                        special
                    );
                    damage_targets.push((false, i, damage as f32));
                }
            }
        }
    }

    let mut cmds = Vec::new();
    for (is_player, idx, amount) in damage_targets {
        if is_player {
            cmds.push(WorldCommand::DamagePlayer {
                amount,
                angle: None,
            });
        } else {
            cmds.push(WorldCommand::DamageThing {
                thing_idx: idx,
                amount,
                inflictor_idx: None,
            });
        }
    }
    world.apply_commands(cmds);
}
