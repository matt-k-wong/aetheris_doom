use aetheris::simulation::*;
use glam::Vec2;
use std::collections::HashSet;

use super::combat::ProjectileThinker;
use super::defs::{DEFAULT_THING_DEFS, DoomThingExt};
use super::rng::p_random;

pub trait DoomWorldExt {
    fn is_walk_trigger(special: u16) -> bool;
    fn spread_noise(&mut self, start_sid: usize, hops: u32);
    fn spawn_effect_thing(&mut self, thing: Thing) -> usize;
    fn fire_hitscan(
        &mut self,
        origin: aetheris::simulation::Vertex,
        angle: f32,
        damage: f32,
        attacker_idx: Option<usize>,
    );
    fn update(&mut self, actions: &std::collections::HashSet<aetheris::simulation::GameAction>);
    fn apply_commands(&mut self, cmds: Vec<aetheris::simulation::WorldCommand>);
    fn activate_linedef_manual(
        &mut self,
        line_idx: usize,
        override_back: Option<usize>,
        cmds: &mut Vec<aetheris::simulation::WorldCommand>,
    );
    fn activate_linedef(
        &mut self,
        special: u16,
        tag: u16,
        sector_back: Option<usize>,
        cmds: &mut Vec<aetheris::simulation::WorldCommand>,
    );
    fn find_lowest_adjacent_ceiling(&self, sector_idx: usize) -> f32;
    fn trigger_door(&mut self, sector_idx: usize, speed: f32, wait: f32) -> bool;
    fn do_door_tagged(&mut self, tag: u16, speed: f32, wait: f32) -> bool;
    fn do_lift_tagged(&mut self, tag: u16);
    fn do_crusher_tagged(&mut self, tag: u16, speed: f32, damage: f32);
    fn do_stairs_tagged(&mut self, tag: u16, step_height: f32);
    fn update_environmental_damage(&mut self);
}

impl DoomWorldExt for WorldState {
    fn spawn_effect_thing(&mut self, thing: Thing) -> usize {
        for (i, t) in self.things.iter_mut().enumerate() {
            if t.picked_up
                && (t.kind == 9997
                    || t.kind == 9998
                    || t.kind == 9999
                    || matches!(t.kind, 127 | 128 | 129 | 10031))
            {
                *t = thing;
                return i;
            }
        }
        let idx = self.things.len();
        self.things.push(thing);
        idx
    }
    fn spread_noise(&mut self, start_sid: usize, hops: u32) {
        let mut queue = vec![(start_sid, hops)];
        let mut visited = std::collections::HashSet::new();

        while let Some((sid, h)) = queue.pop() {
            if !visited.insert(sid) {
                continue;
            }

            for t_idx in 0..self.things.len() {
                let thing = &self.things[t_idx];
                if thing.is_monster() && thing.health > 0.0 && !thing.picked_up {
                    let sidx = self.find_subsector(thing.position.x, thing.position.y);
                    if let Some(ss) = self.subsectors.get(sidx) {
                        if let Some(seg) = self.segs.get(ss.first_seg_idx) {
                            if let Some(tsid) = self.linedefs[seg.linedef_idx].sector_front {
                                if tsid == sid {
                                    for i in 0..self.thinkers.len() {
                                        let mut thinker = self.thinkers.remove(i);
                                        thinker.on_wake(t_idx);
                                        self.thinkers.insert(i, thinker);
                                    }
                                }
                            }
                        }
                    }
                }
            }

            if h > 0 {
                if let Some(neighbors) = self.adjacent_sectors.get(sid) {
                    for &neighbor in neighbors {
                        queue.push((neighbor, h - 1));
                    }
                }
            }
        }
    }
    fn update(&mut self, actions: &HashSet<GameAction>) {
        self.frame_count += 1;

        if self.is_intermission {
            self.intermission_tic += 1;
        }

        if self.player.health <= 0.0 {
            if actions.contains(&GameAction::Fire) {
                self.apply_commands(vec![WorldCommand::RespawnPlayer]);
            }
            return;
        }

        if self.menu_state != MenuState::None && self.frame_count % 35 == 0 {
            log::info!("MENU ACTIVE: Press ESC/ENTER to start map or use ARROWS/WASD to navigate.");
        }

        if self.is_intermission {
            return;
        }

        let mut cmds = Vec::new();

        let old_pos = self.player.position;

        let mut next_angle = self.player.angle;
        if actions.contains(&GameAction::TurnLeft) {
            next_angle += TURN_SPEED;
        }
        if actions.contains(&GameAction::TurnRight) {
            next_angle -= TURN_SPEED;
        }
        next_angle = next_angle.rem_euclid(2.0 * std::f32::consts::PI);

        let dir = Vec2::new(next_angle.cos(), next_angle.sin());
        let strafe = Vec2::new(-dir.y, dir.x);
        let mut wish = Vec2::ZERO;
        if actions.contains(&GameAction::MoveForward) {
            wish += dir;
        }
        if actions.contains(&GameAction::MoveBackward) {
            wish -= dir;
        }
        if actions.contains(&GameAction::StrafeLeft) {
            wish += strafe;
        }
        if actions.contains(&GameAction::StrafeRight) {
            wish -= strafe;
        }

        // Improved movement with better acceleration and friction
        let accel = if wish.length() > 0.1 { 2.5 } else { 0.0 }; // Acceleration when moving
        let friction = 0.85; // Better friction for more control

        let mut next_velocity = self.player.velocity + wish.normalize_or_zero() * accel;
        next_velocity *= friction;

        // Stop completely if velocity is very small
        if next_velocity.length() < 0.01 {
            next_velocity = Vec2::ZERO;
        }

        let mut next_pos = self.player.position + next_velocity;
        let mut next_z = self.player.z;
        let mut next_bob = self.player.bob_phase;

        if next_velocity.length() > 0.1 {
            next_bob += 0.15;
        } else {
            next_bob = 0.0;
        }

        // Collision Detection & Wall Sliding
        let mut lines_to_activate = Vec::new();

        // Use multiple iterations for robust corner collision resolution
        for iter in 0..2 {
            for (line_idx, line) in self.linedefs.iter().enumerate() {
                let (s, e) = (self.vertices[line.start_idx], self.vertices[line.end_idx]);

                // Only check intersection on first pass for efficiency
                let cross = Self::intersect(old_pos, next_pos, s, e);
                let is_crossing = iter == 0 && cross.is_some() && self.player.health > 0.0;

                if is_crossing && line.special_type != 0 && Self::is_walk_trigger(line.special_type)
                {
                    lines_to_activate.push(line_idx);
                }

                let closest = Self::closest_point_on_segment(next_pos, s, e);
                let d_v = next_pos - closest;
                let d = d_v.length();

                if d < PLAYER_RADIUS || is_crossing {
                    let mut should_block = true;

                    // Check portal (door/window) passage
                    if line.is_portal() {
                        if let (Some(fs), Some(bs)) = (line.sector_front, line.sector_back) {
                            let front = &self.sectors[fs];
                            let back = &self.sectors[bs];
                            let lowest_ceiling = front.ceiling_height.min(back.ceiling_height);
                            let highest_floor = front.floor_height.max(back.floor_height);
                            let gap = lowest_ceiling - highest_floor;
                            let step_up = highest_floor - next_z;

                            if gap >= 56.0 && step_up <= STEP_HEIGHT {
                                should_block = false;
                            }
                        }
                    }

                    if should_block {
                        let mut push_dir = if d < 0.001 {
                            let ld = (e - s).normalize_or_zero();
                            Vec2::new(-ld.y, ld.x)
                        } else {
                            d_v / d
                        };

                        let penetration = if push_dir.dot(old_pos - closest) < 0.0 {
                            // We crossed the wall (or the center did)
                            push_dir = -push_dir;
                            PLAYER_RADIUS + d
                        } else {
                            // Still on the correct side
                            PLAYER_RADIUS - d
                        };

                        if penetration > 0.0 {
                            next_pos += push_dir * (penetration + 0.01);

                            // Slide: Remove velocity component moving into the wall
                            let dot = next_velocity.dot(push_dir);
                            if dot < 0.0 {
                                next_velocity -= push_dir * dot;
                            }
                        }
                    }
                }
            }

            // Thing Collision (Monsters, Barrels, Solid Decor)
            for thing in &self.things {
                if thing.picked_up || thing.health <= 0.0 {
                    continue;
                }
                let is_solid = thing.is_monster()
                    || thing.is_barrel()
                    || thing.is_effect()
                    || matches!(thing.kind, 16 | 64..=69 | 71 | 84);
                if !is_solid {
                    continue;
                }

                let d_v = next_pos - thing.position;
                let d = d_v.length();
                // Assumed uniform 20.0 radius for solid DOOM things + 16.0 for player
                let min_dist = PLAYER_RADIUS + 20.0;

                if d < min_dist {
                    let mut push_dir = if d < 0.001 {
                        Vec2::new(1.0, 0.0) // Arbitrary push if exactly stacked
                    } else {
                        d_v / d
                    };

                    let penetration = min_dist - d;
                    next_pos += push_dir * (penetration + 0.01);

                    // Slide against the thing
                    let dot = next_velocity.dot(push_dir);
                    if dot < 0.0 {
                        next_velocity -= push_dir * dot;
                    }
                }
            }
        }

        // Activate linedefs after the collision loop
        for line_idx in lines_to_activate {
            let back = self.linedefs[line_idx].sector_back;
            self.activate_linedef_manual(line_idx, back, &mut cmds);
        }

        // Z-Physics (Gravity & Step Up) & Environmental Effects
        if !self.nodes.is_empty() {
            if let Some(sid) = self.find_sector_at(next_pos) {
                let sector = &mut self.sectors[sid];

                let target_z = sector.floor_height;
                let floor_diff = target_z - next_z;

                // Step UP: Floor is higher but climbable (within STEP_HEIGHT)
                if floor_diff > 0.0 && floor_diff <= STEP_HEIGHT {
                    next_z = target_z;
                // Step DOWN or level: Floor is at or below player - apply gravity
                } else if floor_diff <= 0.0 {
                    if next_z > target_z + 0.1 {
                        next_z -= 2.0;
                        if next_z < target_z {
                            next_z = target_z;
                        }
                    } else {
                        next_z = target_z;
                    }

                    // Damaging Floors and Secrets moved to unified handlers
                    // BLOCKED: Floor is too high to climb
                } else {
                    next_pos = old_pos;
                    next_velocity = Vec2::ZERO;
                }
            }
        }

        // Weapon & State Logic
        let mut next_cooldown = if self.player.fire_cooldown > 0 {
            self.player.fire_cooldown - 1
        } else {
            0
        };
        let mut next_noise = (self.player.noise_radius * 0.9).max(0.0);
        let next_damage_flash = (self.player.damage_flash - 0.05).max(0.0);
        let next_bonus_flash = (self.player.bonus_flash - 0.05).max(0.0);

        // Update Powerup Timers
        if self.player.invuln_timer > 0 {
            self.player.invuln_timer -= 1;
        }
        if self.player.radsuit_timer > 0 {
            self.player.radsuit_timer -= 1;
        }
        if self.player.lightamp_timer > 0 {
            self.player.lightamp_timer -= 1;
        }
        if self.player.invis_timer > 0 {
            self.player.invis_timer -= 1;
        }

        let mut next_weapon = self.player.current_weapon;
        if actions.contains(&GameAction::Weapon1) {
            next_weapon = WeaponType::Fist;
        }
        if actions.contains(&GameAction::Weapon2) {
            next_weapon = WeaponType::Pistol;
        }
        if actions.contains(&GameAction::Weapon3) {
            next_weapon = WeaponType::Shotgun;
        }
        if actions.contains(&GameAction::Weapon4) {
            next_weapon = WeaponType::Chaingun;
        }
        if actions.contains(&GameAction::Weapon5) {
            next_weapon = WeaponType::RocketLauncher;
        }
        if actions.contains(&GameAction::Weapon6) {
            next_weapon = WeaponType::PlasmaRifle;
        }
        if actions.contains(&GameAction::Weapon7) {
            next_weapon = WeaponType::BFG9000;
        }

        let mut next_weapon_state = self.player.weapon_state;
        let mut final_weapon = self.player.current_weapon;

        match next_weapon_state {
            WeaponState::Ready => {
                // Auto weapon swap if current is out of ammo
                let current_ammo_idx = weapon_ammo_type(self.player.current_weapon);
                if let Some(idx) = current_ammo_idx {
                    if self.player.ammo[idx] == 0 {
                        for next_w in [
                            WeaponType::BFG9000,
                            WeaponType::PlasmaRifle,
                            WeaponType::RocketLauncher,
                            WeaponType::Shotgun,
                            WeaponType::Chaingun,
                            WeaponType::Pistol,
                            WeaponType::Chainsaw,
                            WeaponType::Fist,
                        ] {
                            if self.player.owned_weapons[next_w as usize] {
                                let next_ammo_idx = weapon_ammo_type(next_w);
                                if next_ammo_idx.is_none()
                                    || self.player.ammo[next_ammo_idx.unwrap()] > 0
                                {
                                    next_weapon = next_w;
                                    break;
                                }
                            }
                        }
                    }
                }

                if next_weapon != self.player.current_weapon {
                    next_weapon_state = WeaponState::Lowering;
                } else if actions.contains(&GameAction::Fire) && next_cooldown == 0 {
                    let mut fired = false;
                    match final_weapon {
                        WeaponType::Pistol => {
                            if self.player.ammo[0] > 0 {
                                cmds.push(WorldCommand::SpawnAudioEvent(AudioEvent {
                                    sound_id: "DSPISTOL".into(),
                                    position: Some(next_pos),
                                    volume: 1.0,
                                }));
                                let spread = (p_random() as f32 / 255.0 - 0.5) * 0.04;
                                cmds.push(WorldCommand::FireHitscan {
                                    origin: next_pos,
                                    angle: next_angle + spread,
                                    damage: 10.0,
                                    attacker_idx: None,
                                });
                                cmds.push(WorldCommand::UpdatePlayerAmmo {
                                    weapon: WeaponType::Pistol,
                                    amount: -1,
                                    set: false,
                                });
                                next_cooldown = 10;
                                next_weapon_state = WeaponState::Firing(4);
                                fired = true;
                            }
                        }
                        WeaponType::Shotgun => {
                            if self.player.ammo[1] > 0 {
                                cmds.push(WorldCommand::SpawnAudioEvent(AudioEvent {
                                    sound_id: "DSSHOTGN".into(),
                                    position: Some(next_pos),
                                    volume: 1.0,
                                }));
                                for _ in 0..7 {
                                    let spread = (p_random() as f32 / 255.0 - 0.5) * 0.15;
                                    cmds.push(WorldCommand::FireHitscan {
                                        origin: next_pos,
                                        angle: next_angle + spread,
                                        damage: 10.0,
                                        attacker_idx: None,
                                    });
                                }
                                cmds.push(WorldCommand::UpdatePlayerAmmo {
                                    weapon: WeaponType::Shotgun,
                                    amount: -1,
                                    set: false,
                                });
                                next_cooldown = 35;
                                next_weapon_state = WeaponState::Firing(8);
                                fired = true;
                            }
                        }
                        WeaponType::Chaingun => {
                            if self.player.ammo[0] > 0 {
                                cmds.push(WorldCommand::SpawnAudioEvent(AudioEvent {
                                    sound_id: "DSPISTOL".into(),
                                    position: Some(next_pos),
                                    volume: 0.8,
                                }));
                                let spread = (p_random() as f32 / 255.0 - 0.5) * 0.12;
                                cmds.push(WorldCommand::FireHitscan {
                                    origin: next_pos,
                                    angle: next_angle + spread,
                                    damage: 8.0,
                                    attacker_idx: None,
                                });
                                cmds.push(WorldCommand::UpdatePlayerAmmo {
                                    weapon: WeaponType::Pistol,
                                    amount: -1,
                                    set: false,
                                });
                                next_cooldown = 4;
                                next_weapon_state = WeaponState::Firing(2);
                                fired = true;
                            }
                        }
                        WeaponType::RocketLauncher => {
                            if self.player.ammo[2] > 0 {
                                cmds.push(WorldCommand::SpawnAudioEvent(AudioEvent {
                                    sound_id: "DSRLAUNC".into(),
                                    position: Some(next_pos),
                                    volume: 1.0,
                                }));
                                let r_dir = Vec2::new(next_angle.cos(), next_angle.sin());
                                cmds.push(WorldCommand::SpawnProjectile {
                                    kind: 127,
                                    position: next_pos + r_dir * 20.0,
                                    z: next_z + 28.0,
                                    velocity: r_dir * 20.0,
                                    z_velocity: 0.0,
                                    damage: 20.0,
                                    owner_is_player: true,
                                    owner_thing_idx: None,
                                });
                                cmds.push(WorldCommand::UpdatePlayerAmmo {
                                    weapon: WeaponType::RocketLauncher,
                                    amount: -1,
                                    set: false,
                                });
                                next_cooldown = 20;
                                next_weapon_state = WeaponState::Firing(4);
                                fired = true;
                            }
                        }
                        WeaponType::PlasmaRifle => {
                            if self.player.ammo[3] > 0 {
                                cmds.push(WorldCommand::SpawnAudioEvent(AudioEvent {
                                    sound_id: "DSPLASMA".into(),
                                    position: Some(next_pos),
                                    volume: 1.0,
                                }));
                                let r_dir = Vec2::new(next_angle.cos(), next_angle.sin());
                                cmds.push(WorldCommand::SpawnProjectile {
                                    kind: 128,
                                    position: next_pos + r_dir * 20.0,
                                    z: next_z + 28.0,
                                    velocity: r_dir * 25.0,
                                    z_velocity: 0.0,
                                    damage: 5.0,
                                    owner_is_player: true,
                                    owner_thing_idx: None,
                                });
                                cmds.push(WorldCommand::UpdatePlayerAmmo {
                                    weapon: WeaponType::PlasmaRifle,
                                    amount: -1,
                                    set: false,
                                });
                                next_cooldown = 3;
                                next_weapon_state = WeaponState::Firing(2);
                                fired = true;
                            }
                        }
                        WeaponType::BFG9000 => {
                            if self.player.ammo[3] >= 40 {
                                cmds.push(WorldCommand::SpawnAudioEvent(AudioEvent {
                                    sound_id: "DSBFG".into(),
                                    position: Some(next_pos),
                                    volume: 1.0,
                                }));
                                let r_dir = Vec2::new(next_angle.cos(), next_angle.sin());
                                cmds.push(WorldCommand::SpawnProjectile {
                                    kind: 129,
                                    position: next_pos + r_dir * 20.0,
                                    z: next_z + 28.0,
                                    velocity: r_dir * 15.0,
                                    z_velocity: 0.0,
                                    damage: 100.0,
                                    owner_is_player: true,
                                    owner_thing_idx: None,
                                });
                                cmds.push(WorldCommand::UpdatePlayerAmmo {
                                    weapon: WeaponType::BFG9000,
                                    amount: -40,
                                    set: false,
                                });
                                next_cooldown = 40;
                                next_weapon_state = WeaponState::Firing(10);
                                fired = true;
                            }
                        }
                        WeaponType::Chainsaw => {
                            cmds.push(WorldCommand::SpawnAudioEvent(AudioEvent {
                                sound_id: "DSSAWFUL".into(),
                                position: Some(next_pos),
                                volume: 1.0,
                            }));
                            cmds.push(WorldCommand::FireHitscan {
                                origin: next_pos,
                                angle: next_angle,
                                damage: 20.0,
                                attacker_idx: None,
                            });
                            next_cooldown = 4;
                            next_weapon_state = WeaponState::Firing(2);
                            fired = true;
                        }
                        WeaponType::Fist => {
                            let dmg = if self.player.berserk_timer > 0 {
                                200.0
                            } else {
                                20.0
                            };
                            cmds.push(WorldCommand::FireHitscan {
                                origin: next_pos,
                                angle: next_angle,
                                damage: dmg,
                                attacker_idx: None,
                            });
                            next_cooldown = 15;
                            next_weapon_state = WeaponState::Firing(4);
                            fired = true;
                        }
                    }
                    if fired {
                        next_noise = NOISE_RADIUS_FIRE;

                        if let Some(sector_idx) = self.find_sector_at(next_pos) {
                            self.spread_noise(sector_idx, 3);

                            // Trigger Muzzle Flash in the current sector
                            let current_light = self.sectors[sector_idx].light_level;
                            cmds.push(WorldCommand::SetSectorState {
                                sector_idx,
                                floor: self.sectors[sector_idx].floor_height,
                                ceiling: self.sectors[sector_idx].ceiling_height,
                                light: 1.0, // Flash to full brightness
                                action: SectorAction::MuzzleFlash {
                                    timer: 0.05, // 2 tics approx
                                    original_light: current_light,
                                },
                                texture_floor: None,
                                texture_ceiling: None,
                            });
                        }
                    }
                }
            }
            WeaponState::Lowering => {
                if next_cooldown == 0 {
                    next_cooldown = 10;
                    final_weapon = next_weapon;
                    next_weapon_state = WeaponState::Raising;
                }
            }
            WeaponState::Raising => {
                if next_cooldown == 0 {
                    next_weapon_state = WeaponState::Ready;
                }
            }
            WeaponState::Firing(frames_left) => {
                if frames_left > 1 {
                    next_weapon_state = WeaponState::Firing(frames_left - 1);
                } else {
                    next_weapon_state = WeaponState::Ready;
                }
            }
            _ => {}
        }

        cmds.push(WorldCommand::UpdatePlayer {
            position: Some(next_pos),
            angle: Some(next_angle),
            velocity: Some(next_velocity),
            z: Some(next_z),
            health: None,
            armor: None,
            weapon_state: Some(next_weapon_state),
            fire_cooldown: Some(next_cooldown),
            noise_radius: Some(next_noise),
            current_weapon: Some(final_weapon),
            damage_flash: Some(next_damage_flash),
            bonus_flash: Some(next_bonus_flash),
            bob_phase: Some(next_bob),
        });

        // Check for pickups
        for (i, thing) in self.things.iter().enumerate() {
            if thing.is_pickup() && !thing.picked_up {
                let dist = (self.player.position - thing.position).length();
                if dist < PICKUP_RADIUS {
                    cmds.push(WorldCommand::PickupItem { thing_idx: i });
                }
            }
        }

        // Check Linedef Crossings (Walk Triggers)
        // Line crossing triggers are handled in the movement loop above.

        // Use Actions (Manual Triggers)
        // Use Actions (Manual Triggers)
        if actions.contains(&GameAction::Use) {
            // Simple Raycast for Use Line
            let p_pos = self.player.position;
            let reach = 128.0; // Boosted for better accessibility (standard is 64)

            let mut best_line = None;
            let mut best_dist = reach;

            for (idx, line) in self.linedefs.iter().enumerate() {
                if line.special_type == 0 {
                    continue;
                }
                let v1 = self.vertices[line.start_idx];
                let v2 = self.vertices[line.end_idx];

                let line_vec = v2 - v1;
                let line_len_sq = line_vec.length_squared();
                if line_len_sq < 1.0 {
                    continue;
                }

                let line_normal = Vec2::new(-line_vec.y, line_vec.x).normalize();
                let player_to_v1 = v1 - p_pos;
                let dist_to_line = player_to_v1.dot(line_normal).abs();

                if dist_to_line < best_dist {
                    let t = (p_pos - v1).dot(line_vec) / line_len_sq;
                    if t >= -0.1 && t <= 1.1 {
                        best_dist = dist_to_line;
                        best_line = Some(idx);
                    }
                }
            }

            if let Some(idx) = best_line {
                let line = &self.linedefs[idx];
                let mut sector_back = line.sector_back;

                if line.sector_tag == 0 && line.is_portal() {
                    let p_pos = self.player.position;
                    let v1 = self.vertices[line.start_idx];
                    let v2 = self.vertices[line.end_idx];
                    let side = (p_pos.x - v1.x) * (v2.y - v1.y) - (p_pos.y - v1.y) * (v2.x - v1.x);
                    if side > 0.0 {
                        sector_back = line.sector_back;
                    } else {
                        sector_back = line.sector_front;
                    }
                }

                log::info!(
                    "Player used line {} with special {} (target sector_back: {:?})",
                    idx,
                    line.special_type,
                    sector_back
                );
                self.activate_linedef_manual(idx, sector_back, &mut cmds);
            }
        }

        // Apply Commands
        self.apply_commands(cmds);

        // Decay HUD messages
        for msg in &mut self.hud_messages {
            msg.timer -= 1.0 / 35.0; // Decay at 35 FPS
        }
        self.hud_messages.retain(|m| m.timer > 0.0);

        // Decay temporary things (Puffs/Sparks/Projectiles)
        // IMPORTANT: Never remove things from the vector — it invalidates thinker indices.
        // Instead, mark them as picked_up so they're skipped during rendering.

        // Pre-calculate sector IDs to avoid double-borrow during thing update loop
        let sector_ids: Vec<Option<usize>> = self
            .things
            .iter()
            .map(|t| self.find_sector_at(t.position))
            .collect();

        for (i, t) in self.things.iter_mut().enumerate() {
            // Apply Gravity to all non-flying things
            if !t.is_effect() && t.health > -100.0 {
                // Determine floor height at thing position
                if let Some(sid) = sector_ids[i] {
                    if sid < self.sectors.len() {
                        let floor_z = self.sectors[sid].floor_height;
                        if t.z > floor_z + 0.1 {
                            t.z -= 4.0; // Gravity fall rate
                            if t.z < floor_z {
                                t.z = floor_z;
                            }
                        } else if t.z < floor_z - 0.1 {
                            // Snap up (step up) for monsters/items if they are on stairs
                            t.z = floor_z;
                        }
                    }
                }
            }

            if t.is_effect() {
                t.health -= 1.0;
                if t.health <= 0.0 {
                    t.picked_up = true;
                }
            }
            // Mark dead projectiles as picked_up so they don't accumulate
            if matches!(t.kind, 127 | 128 | 129 | 10031) && t.health <= 0.0 {
                t.picked_up = true;
            }
        }

        // Apply Environmental Damage (Slime/Acid) and Sector Effects (Secrets)
        self.update_environmental_damage();

        // Update Sector Actions (Elevators, Doors)
        for i in 0..self.sectors.len() {
            let s_cmds = self.sectors[i].calculate_update(1.0 / 35.0, i, self.frame_count);
            self.apply_commands(s_cmds);
        }

        // Sequential thinker updates — preserves deterministic RNG order
        // (Doom's static PRND_INDEX is not thread-safe, and determinism
        // matters for demo recording. ~20 monsters need no parallelism.)
        let thinkers: Vec<Box<dyn Thinker + Send + Sync>> = std::mem::take(&mut self.thinkers);
        let results: Vec<(Box<dyn Thinker + Send + Sync>, bool, Vec<WorldCommand>)> = thinkers
            .into_iter()
            .map(|mut t| {
                let (keep, cmds) = t.update(self);
                (t, keep, cmds)
            })
            .collect();

        for (t, keep, cmds) in results {
            self.apply_commands(cmds);
            if keep {
                self.thinkers.push(t);
            }
        }

        // Update palette based on flash state (authentic Doom PLAYPAL switching)
        if self.player.damage_flash > 0.3 && self.palettes.len() > 1 {
            self.current_palette_idx = 1; // Red palette (damage)
        } else if self.player.bonus_flash > 0.3 && self.palettes.len() > 4 {
            self.current_palette_idx = 4; // Yellow palette (bonus)
        } else {
            self.current_palette_idx = 0; // Normal palette
        }

        // NOTE: Do NOT clear audio_events here - they are consumed by the audio engine
        // in lib.rs AFTER world.update() returns, then cleared there.
    }

    fn apply_commands(&mut self, cmds: Vec<WorldCommand>) {
        for cmd in cmds {
            match cmd {
                WorldCommand::SpawnThinker(t) => self.thinkers.push(t),
                WorldCommand::SpawnAudioEvent(e) => self.audio_events.push(e),
                WorldCommand::ShowMessage {
                    text,
                    duration_secs,
                    color,
                } => {
                    self.hud_messages.push(HudMessage {
                        text,
                        timer: duration_secs,
                        color,
                    });
                }
                WorldCommand::ModifySector {
                    sector_idx,
                    floor_delta,
                    ceiling_delta,
                } => {
                    let final_floor_delta = floor_delta;
                    let mut final_ceil_delta = ceiling_delta;
                    let mut crush_damage = 0.0;
                    let mut reverse_door = false;

                    // Collision Logic (Immutable Phase)
                    {
                        // We need to check against the *proposed* new heights.
                        // But we can't easily get 'new_ceil' without accessing 's'.
                        // We can lookup 's' inside the loop safely if we don't hold it across 'find_sector_at'.

                        for i in 0..self.things.len() {
                            let (pos, z, height, health) = {
                                let t = &self.things[i];
                                let h = DEFAULT_THING_DEFS
                                    .iter()
                                    .find(|&&(k, _)| k == t.kind)
                                    .map(|&(_, d)| d.height)
                                    .unwrap_or(56.0);
                                (t.position, t.z, h, t.health)
                            };

                            // Check sector
                            if self.find_sector_at(pos) == Some(sector_idx) {
                                // Now safe to borrow sector
                                let s = &self.sectors[sector_idx];
                                let new_ceil = s.ceiling_height + ceiling_delta; // Current + delta

                                // Check Ceiling Collision
                                if z + height > new_ceil {
                                    match &s.action {
                                        SectorAction::Door { state, .. } => {
                                            if *state == DoorState::Closing {
                                                reverse_door = true;
                                                final_ceil_delta = 0.0; // Stop movement
                                            }
                                        }
                                        SectorAction::Crusher { damage, .. } => {
                                            if health > 0.0 {
                                                crush_damage = *damage;
                                                // Clamp to thing top
                                                let clamp_delta = (z + height) - s.ceiling_height;
                                                if clamp_delta > final_ceil_delta {
                                                    // Usually negative when moving down
                                                    final_ceil_delta = clamp_delta;
                                                }
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                            }
                        }
                    }

                    // Mutation Phase
                    if let Some(s) = self.sectors.get_mut(sector_idx) {
                        s.floor_height += final_floor_delta;
                        s.ceiling_height += final_ceil_delta;

                        if reverse_door {
                            if let SectorAction::Door { state, .. } = &mut s.action {
                                *state = DoorState::Opening;
                            }
                        }
                    }

                    // Apply deferred damage if any
                    if crush_damage > 0.0 {
                        // Apply damage to things in sector that are crushing
                        let mut targets = Vec::new();
                        for i in 0..self.things.len() {
                            let (pos, z, height) = {
                                let t = &self.things[i];
                                let h = DEFAULT_THING_DEFS
                                    .iter()
                                    .find(|&&(k, _)| k == t.kind)
                                    .map(|&(_, d)| d.height)
                                    .unwrap_or(56.0);
                                (t.position, t.z, h)
                            };
                            if self.find_sector_at(pos) == Some(sector_idx) {
                                // Need to re-access sector height?
                                // self.sectors is borrowed mutably? No, scope ended?
                                // Wait, self.sectors[sector_idx] usage in loop 's' borrow ended?
                                // Line 1083 '}' ended 'if let Some(s) ...'.
                                // So self.sectors is free.
                                if z + height > (self.sectors[sector_idx].ceiling_height - 0.1) {
                                    targets.push(i);
                                }
                            }
                        }
                        for idx in targets {
                            self.apply_commands(vec![WorldCommand::DamageThing {
                                thing_idx: idx,
                                amount: crush_damage,
                                inflictor_idx: None,
                            }]);
                        }
                    }
                }
                WorldCommand::SetSectorState {
                    sector_idx,
                    floor,
                    ceiling,
                    light,
                    action,
                    texture_floor,
                    texture_ceiling,
                } => {
                    if let Some(s) = self.sectors.get_mut(sector_idx) {
                        s.floor_height = floor;
                        s.ceiling_height = ceiling;
                        s.light_level = light;
                        s.action = action;
                        if let Some(tex) = texture_floor {
                            s.texture_floor = tex;
                        }
                        if let Some(tex) = texture_ceiling {
                            s.texture_ceiling = tex;
                        }
                    }
                }
                WorldCommand::ModifyThing {
                    thing_idx,
                    pos_delta,
                    z_delta,
                    angle,
                } => {
                    if let Some(t) = self.things.get_mut(thing_idx) {
                        t.position += pos_delta;
                        t.z += z_delta;
                        t.angle = angle;
                    }
                }
                WorldCommand::SetThingHealth { thing_idx, health } => {
                    if let Some(t) = self.things.get_mut(thing_idx) {
                        t.health = health;
                    }
                }
                WorldCommand::UpdatePlayer {
                    position,
                    angle,
                    velocity,
                    z,
                    health,
                    armor,
                    weapon_state,
                    fire_cooldown,
                    noise_radius,
                    current_weapon,
                    damage_flash,
                    bonus_flash,
                    bob_phase,
                } => {
                    if let Some(pos) = position {
                        self.player.position = pos;
                    }
                    if let Some(ang) = angle {
                        self.player.angle = ang;
                    }
                    if let Some(vel) = velocity {
                        self.player.velocity = vel;
                    }
                    if let Some(zv) = z {
                        self.player.z = zv;
                    }
                    if let Some(h) = health {
                        self.player.health = h;
                    }
                    if let Some(a) = armor {
                        self.player.armor = a;
                    }
                    if let Some(ws) = weapon_state {
                        self.player.weapon_state = ws;
                    }
                    if let Some(fc) = fire_cooldown {
                        self.player.fire_cooldown = fc;
                    }
                    if let Some(nr) = noise_radius {
                        self.player.noise_radius = nr;
                    }
                    if let Some(cw) = current_weapon {
                        self.player.current_weapon = cw;
                    }
                    if let Some(df) = damage_flash {
                        self.player.damage_flash = df;
                    }
                    if let Some(bf) = bonus_flash {
                        self.player.bonus_flash = bf;
                    }
                    if let Some(bp) = bob_phase {
                        self.player.bob_phase = bp;
                    }
                }
                WorldCommand::UpdatePlayerAmmo {
                    weapon,
                    amount,
                    set,
                } => {
                    let idx = match weapon {
                        WeaponType::Pistol | WeaponType::Chaingun => 0,
                        WeaponType::Shotgun => 1,
                        WeaponType::RocketLauncher => 2,
                        WeaponType::PlasmaRifle | WeaponType::BFG9000 => 3,
                        _ => 0,
                    };
                    if set {
                        self.player.ammo[idx] = amount as u32;
                    } else {
                        self.player.ammo[idx] =
                            (self.player.ammo[idx] as i32 + amount).max(0) as u32;
                    }
                }
                WorldCommand::PickupItem { thing_idx } => {
                    if let Some(thing) = self.things.get(thing_idx) {
                        let kind = thing.kind;
                        let mut success = false;

                        match kind {
                            // Health
                            2011 => {
                                if self.player.health < 100.0 {
                                    self.player.health = (self.player.health + 10.0).min(100.0);
                                    success = true;
                                }
                            }
                            2012 => {
                                if self.player.health < 100.0 {
                                    self.player.health = (self.player.health + 25.0).min(100.0);
                                    success = true;
                                }
                            }
                            2013 => {
                                self.player.health = (self.player.health + 100.0).min(200.0);
                                success = true;
                            }
                            2014 => {
                                self.player.health = (self.player.health + 1.0).min(200.0);
                                success = true;
                            }
                            2045 => {
                                self.player.lightamp_timer = 4200;
                                success = true;
                            }

                            // Powerups
                            2022 => {
                                self.player.invuln_timer = 1050;
                                success = true;
                            }
                            2023 => {
                                self.player.berserk_timer = 40000;
                                self.player.health = 100.0;
                                success = true;
                            }
                            2024 => {
                                self.player.invis_timer = 2100;
                                success = true;
                            }
                            2025 => {
                                self.player.radsuit_timer = 2100;
                                success = true;
                            }
                            2026 => {
                                self.is_automap_follow = true;
                                success = true;
                            }

                            // Armor
                            2018 => {
                                if self.player.armor < 100.0 {
                                    self.player.armor = 100.0;
                                    success = true;
                                }
                            }
                            2019 => {
                                if self.player.armor < 200.0 {
                                    self.player.armor = 200.0;
                                    success = true;
                                }
                            }
                            2015 => {
                                self.player.armor = (self.player.armor + 1.0).min(200.0);
                                success = true;
                            }

                            // Weapons
                            2001 => {
                                self.player.owned_weapons[WeaponType::Shotgun as usize] = true;
                                self.apply_commands(vec![WorldCommand::UpdatePlayerAmmo {
                                    weapon: WeaponType::Shotgun,
                                    amount: 8,
                                    set: false,
                                }]);
                                if (self.player.current_weapon as usize)
                                    < WeaponType::Shotgun as usize
                                {
                                    self.player.current_weapon = WeaponType::Shotgun;
                                    self.player.weapon_state = WeaponState::Raising;
                                    self.player.fire_cooldown = 10;
                                }
                                success = true;
                            }
                            2002 => {
                                self.player.owned_weapons[WeaponType::Chaingun as usize] = true;
                                self.apply_commands(vec![WorldCommand::UpdatePlayerAmmo {
                                    weapon: WeaponType::Pistol,
                                    amount: 20,
                                    set: false,
                                }]);
                                if (self.player.current_weapon as usize)
                                    < WeaponType::Chaingun as usize
                                {
                                    self.player.current_weapon = WeaponType::Chaingun;
                                    self.player.weapon_state = WeaponState::Raising;
                                    self.player.fire_cooldown = 10;
                                }
                                success = true;
                            }
                            2003 => {
                                self.player.owned_weapons[WeaponType::RocketLauncher as usize] =
                                    true;
                                self.apply_commands(vec![WorldCommand::UpdatePlayerAmmo {
                                    weapon: WeaponType::RocketLauncher,
                                    amount: 2,
                                    set: false,
                                }]);
                                if (self.player.current_weapon as usize)
                                    < WeaponType::RocketLauncher as usize
                                {
                                    self.player.current_weapon = WeaponType::RocketLauncher;
                                    self.player.weapon_state = WeaponState::Raising;
                                    self.player.fire_cooldown = 10;
                                }
                                success = true;
                            }
                            2004 => {
                                self.player.owned_weapons[WeaponType::PlasmaRifle as usize] = true;
                                self.apply_commands(vec![WorldCommand::UpdatePlayerAmmo {
                                    weapon: WeaponType::PlasmaRifle,
                                    amount: 40,
                                    set: false,
                                }]);
                                if (self.player.current_weapon as usize)
                                    < WeaponType::PlasmaRifle as usize
                                {
                                    self.player.current_weapon = WeaponType::PlasmaRifle;
                                    self.player.weapon_state = WeaponState::Raising;
                                    self.player.fire_cooldown = 10;
                                }
                                success = true;
                            }
                            2005 => {
                                self.player.owned_weapons[WeaponType::Chainsaw as usize] = true;
                                // Chainsaw is usually preferred over fist/pistol
                                if self.player.current_weapon == WeaponType::Fist
                                    || self.player.current_weapon == WeaponType::Pistol
                                {
                                    self.player.current_weapon = WeaponType::Chainsaw;
                                    self.player.weapon_state = WeaponState::Raising;
                                    self.player.fire_cooldown = 10;
                                }
                                success = true;
                            }
                            2006 => {
                                self.player.owned_weapons[WeaponType::BFG9000 as usize] = true;
                                self.apply_commands(vec![WorldCommand::UpdatePlayerAmmo {
                                    weapon: WeaponType::BFG9000,
                                    amount: 40,
                                    set: false,
                                }]);
                                if (self.player.current_weapon as usize)
                                    < WeaponType::BFG9000 as usize
                                {
                                    self.player.current_weapon = WeaponType::BFG9000;
                                    self.player.weapon_state = WeaponState::Raising;
                                    self.player.fire_cooldown = 10;
                                }
                                success = true;
                            }

                            // Ammo
                            2007 => {
                                self.apply_commands(vec![WorldCommand::UpdatePlayerAmmo {
                                    weapon: WeaponType::Pistol,
                                    amount: 10,
                                    set: false,
                                }]);
                                success = true;
                            }
                            2048 => {
                                self.apply_commands(vec![WorldCommand::UpdatePlayerAmmo {
                                    weapon: WeaponType::Pistol,
                                    amount: 50,
                                    set: false,
                                }]);
                                success = true;
                            }
                            2008 => {
                                self.apply_commands(vec![WorldCommand::UpdatePlayerAmmo {
                                    weapon: WeaponType::Shotgun,
                                    amount: 4,
                                    set: false,
                                }]);
                                success = true;
                            }
                            2049 => {
                                self.apply_commands(vec![WorldCommand::UpdatePlayerAmmo {
                                    weapon: WeaponType::Shotgun,
                                    amount: 20,
                                    set: false,
                                }]);
                                success = true;
                            }
                            2010 => {
                                self.apply_commands(vec![WorldCommand::UpdatePlayerAmmo {
                                    weapon: WeaponType::RocketLauncher,
                                    amount: 1,
                                    set: false,
                                }]);
                                success = true;
                            }
                            2046 => {
                                self.apply_commands(vec![WorldCommand::UpdatePlayerAmmo {
                                    weapon: WeaponType::RocketLauncher,
                                    amount: 5,
                                    set: false,
                                }]);
                                success = true;
                            }
                            2047 => {
                                self.apply_commands(vec![WorldCommand::UpdatePlayerAmmo {
                                    weapon: WeaponType::PlasmaRifle,
                                    amount: 20,
                                    set: false,
                                }]);
                                success = true;
                            }
                            17 => {
                                self.apply_commands(vec![WorldCommand::UpdatePlayerAmmo {
                                    weapon: WeaponType::PlasmaRifle,
                                    amount: 100,
                                    set: false,
                                }]);
                                success = true;
                            }

                            // Keys
                            5 | 40 => {
                                self.player.keys[1] = true;
                                success = true;
                            }
                            6 | 39 => {
                                self.player.keys[2] = true;
                                success = true;
                            }
                            13 | 38 => {
                                self.player.keys[0] = true;
                                success = true;
                            }

                            _ => {
                                success = true;
                            }
                        }

                        if success {
                            if let Some(thing) = self.things.get_mut(thing_idx) {
                                thing.picked_up = true;
                            }
                            self.player.bonus_flash = 0.4;
                            self.audio_events.push(AudioEvent {
                                sound_id: "DSGETPOW".into(),
                                position: None,
                                volume: 1.0,
                            });
                        }
                    }
                }
                WorldCommand::DamageThing {
                    thing_idx,
                    amount,
                    inflictor_idx,
                } => {
                    let t_kind = self.things.get(thing_idx).map(|t| t.kind).unwrap_or(0);
                    let i_kind =
                        inflictor_idx.and_then(|idx| self.things.get(idx).map(|th| th.kind));

                    if let Some(t) = self.things.get_mut(thing_idx) {
                        t.health -= amount;
                        for thinker in &mut self.thinkers {
                            thinker.on_pain(thing_idx, t_kind, inflictor_idx, i_kind);
                        }
                    }
                }
                WorldCommand::DamagePlayer { amount, angle } => {
                    if self.player.invuln_timer == 0 {
                        let absorbed = (amount * 0.333).min(self.player.armor);
                        self.player.armor -= absorbed;
                        self.player.health -= amount - absorbed;
                        if self.player.damage_flash < 0.1 {
                            self.player.damage_flash = 0.5;
                        }
                        self.player.last_damage_angle = angle;
                    }
                }
                WorldCommand::DamageThingsInSector { sector_idx, amount } => {
                    // Check player
                    if let Some(sid) = self.find_sector_at(self.player.position) {
                        if sid == sector_idx {
                            if self.player.invuln_timer == 0 {
                                let absorbed = (amount * 0.333).min(self.player.armor);
                                self.player.armor -= absorbed;
                                self.player.health -= amount - absorbed;
                                if self.player.damage_flash < 0.1 {
                                    self.player.damage_flash = 0.5;
                                }
                            }
                        }
                    }

                    // Check monsters
                    let mut impacts = Vec::new();
                    for (i, t) in self.things.iter().enumerate() {
                        if !t.is_monster() || t.health <= 0.0 || t.picked_up {
                            continue;
                        }
                        if let Some(sid) = self.find_sector_at(t.position) {
                            if sid == sector_idx {
                                impacts.push((i, t.kind));
                            }
                        }
                    }
                    for (idx, kind) in impacts {
                        if let Some(t) = self.things.get_mut(idx) {
                            t.health -= amount;
                        }
                        for thinker in &mut self.thinkers {
                            thinker.on_pain(idx, kind, None, None);
                        }
                    }
                }
                WorldCommand::FireHitscan {
                    origin,
                    angle,
                    damage,
                    attacker_idx,
                } => self.fire_hitscan(origin, angle, damage, attacker_idx),
                WorldCommand::SplashDamage {
                    center,
                    damage,
                    radius,
                    owner_is_player,
                } => {
                    // Collect impacted monsters first to avoid borrow conflicts
                    let mut impacts = Vec::new();
                    for (i, thing) in self.things.iter().enumerate() {
                        if thing.picked_up || thing.health <= 0.0 {
                            continue;
                        }
                        if !thing.is_monster() && !thing.is_barrel() && !(owner_is_player && i == 0)
                        {
                            continue;
                        }
                        let dist = (thing.position - center).length();
                        if dist < radius {
                            let dmg = damage * ((radius - dist) / radius);
                            if dmg > 0.0 {
                                impacts.push((i, dmg, thing.kind));
                            }
                        }
                    }

                    let mut sorted_things: Vec<(usize, &Thing)> =
                        self.things.iter().enumerate().collect();
                    sorted_things.sort_by(|a, b| {
                        let d1 = (a.1.position - center).length_squared();
                        let d2 = (b.1.position - center).length_squared();
                        // Defensive: handle NaN positions gracefully
                        match (d1.is_nan(), d2.is_nan()) {
                            (true, true) => std::cmp::Ordering::Equal,
                            (true, false) => std::cmp::Ordering::Greater,
                            (false, true) => std::cmp::Ordering::Less,
                            (false, false) => {
                                d2.partial_cmp(&d1).unwrap_or(std::cmp::Ordering::Equal)
                            }
                        }
                    });

                    for (idx, dmg, kind) in impacts {
                        if let Some(t) = self.things.get_mut(idx) {
                            t.health -= dmg;
                        }
                        for thinker in &mut self.thinkers {
                            thinker.on_pain(idx, kind, None, None);
                        }
                    }

                    let p_dist = (self.player.position - center).length();
                    if p_dist < radius {
                        let p_dmg = damage * ((radius - p_dist) / radius);
                        if p_dmg > 0.0 {
                            if self.player.invuln_timer == 0 {
                                let absorbed = (p_dmg * 0.333).min(self.player.armor);
                                self.player.armor -= absorbed;
                                self.player.health -= p_dmg - absorbed;
                                self.player.damage_flash =
                                    (self.player.damage_flash + 0.5).min(1.0);
                            }
                        }
                    }
                }
                WorldCommand::SpawnThing {
                    kind,
                    position,
                    z,
                    angle,
                } => {
                    let thing = Thing {
                        position,
                        z,
                        angle,
                        kind,
                        flags: 0,
                        health: if kind == 9999 || kind == 9998 || kind == 9997 {
                            4.0
                        } else {
                            50.0
                        },
                        picked_up: false,
                        state_idx: 0,
                        ai_timer: 0,
                        target_thing_idx: None,
                        attack_cooldown: 0,
                    };
                    self.spawn_effect_thing(thing);
                }
                WorldCommand::SpawnProjectile {
                    kind,
                    position,
                    z,
                    velocity,
                    z_velocity,
                    damage,
                    owner_is_player,
                    owner_thing_idx,
                } => {
                    let proj_thing = Thing {
                        position,
                        z,
                        angle: 0.0,
                        kind,
                        flags: 0,
                        health: 100.0,
                        picked_up: false,
                        state_idx: 0,
                        ai_timer: 0,
                        target_thing_idx: None,
                        attack_cooldown: 0,
                    };
                    let idx = self.spawn_effect_thing(proj_thing);
                    self.thinkers.push(Box::new(ProjectileThinker {
                        thing_idx: idx,
                        position,
                        z,
                        velocity,
                        z_velocity,
                        damage,
                        owner_is_player,
                        owner_thing_idx,
                    }));
                }
                WorldCommand::InflictPain {
                    thing_idx,
                    inflictor_idx,
                } => {
                    let t_kind = self.things.get(thing_idx).map(|t| t.kind).unwrap_or(0);
                    let i_kind = inflictor_idx.and_then(|idx| self.things.get(idx).map(|t| t.kind));
                    for t in &mut self.thinkers {
                        t.on_pain(thing_idx, t_kind, inflictor_idx, i_kind);
                    }
                }
                WorldCommand::Win => self.is_win = true,
                WorldCommand::RespawnPlayer => {
                    self.player.health = 100.0;
                    self.player.position = self.player_start_pos;
                    self.player.velocity = Vec2::ZERO;
                    self.player.invuln_timer = 105;
                    if !self.nodes.is_empty() {
                        let sidx =
                            self.find_subsector(self.player.position.x, self.player.position.y);
                        if let Some(ss) = self.subsectors.get(sidx) {
                            if let Some(seg) = self.segs.get(ss.first_seg_idx) {
                                if let Some(sid) = self.linedefs[seg.linedef_idx].sector_front {
                                    self.player.z = self.sectors[sid].floor_height;
                                }
                            }
                        }
                    }
                }
                WorldCommand::SyncAiState {
                    thing_idx,
                    state_idx,
                    timer,
                    target,
                    cooldown,
                } => {
                    if let Some(t) = self.things.get_mut(thing_idx) {
                        t.state_idx = state_idx;
                        t.ai_timer = timer;
                        t.target_thing_idx = target;
                        t.attack_cooldown = cooldown;
                    }
                }
            }
        }
    }

    fn is_walk_trigger(special: u16) -> bool {
        super::linedefs::is_walk_trigger(special)
    }

    fn fire_hitscan(
        &mut self,
        origin: aetheris::simulation::Vertex,
        angle: f32,
        damage: f32,
        attacker_idx: Option<usize>,
    ) {
        super::combat::fire_hitscan(self, origin, angle, damage, attacker_idx);
    }

    fn activate_linedef_manual(
        &mut self,
        line_idx: usize,
        override_back: Option<usize>,
        cmds: &mut Vec<aetheris::simulation::WorldCommand>,
    ) {
        super::linedefs::activate_linedef_manual(self, line_idx, override_back, cmds);
    }

    fn activate_linedef(
        &mut self,
        special: u16,
        tag: u16,
        sector_back: Option<usize>,
        cmds: &mut Vec<aetheris::simulation::WorldCommand>,
    ) {
        super::linedefs::activate_linedef(self, special, tag, sector_back, cmds);
    }

    fn find_lowest_adjacent_ceiling(&self, sector_idx: usize) -> f32 {
        super::linedefs::find_lowest_adjacent_ceiling(self, sector_idx)
    }

    fn trigger_door(&mut self, sector_idx: usize, speed: f32, wait: f32) -> bool {
        super::linedefs::trigger_door(self, sector_idx, speed, wait)
    }

    fn do_door_tagged(&mut self, tag: u16, speed: f32, wait: f32) -> bool {
        super::linedefs::do_door_tagged(self, tag, speed, wait)
    }

    fn do_lift_tagged(&mut self, tag: u16) {
        super::linedefs::do_lift_tagged(self, tag);
    }

    fn do_crusher_tagged(&mut self, tag: u16, speed: f32, damage: f32) {
        super::linedefs::do_crusher_tagged(self, tag, speed, damage);
    }

    fn do_stairs_tagged(&mut self, tag: u16, step_height: f32) {
        super::linedefs::do_stairs_tagged(self, tag, step_height);
    }

    fn update_environmental_damage(&mut self) {
        super::linedefs::update_environmental_damage(self);
    }
}

pub fn init_world(_world: &mut WorldState) {}
