use aetheris::simulation::*;
use glam::{Mat2, Vec2};

use super::defs::{DEFAULT_THING_DEFS, DoomThingExt};
use super::rng::p_random;
use super::states::*;

pub struct MonsterThinker {
    pub thing_idx: usize,
    pub state_idx: usize,
    pub tics: i32,
    pub target_thing_idx: Option<usize>,
    pub attack_cooldown: u32,     // Cooldown between attacks to prevent spam
    pub just_entered_state: bool, // True when state was just set, action should fire
}

impl MonsterThinker {
    pub fn new(
        thing_idx: usize,
        state_idx: usize,
        tics: i32,
        target: Option<usize>,
        cooldown: u32,
    ) -> Self {
        Self {
            thing_idx,
            state_idx,
            tics,
            target_thing_idx: target,
            attack_cooldown: cooldown,
            just_entered_state: true, // Fire action on first tick
        }
    }

    fn is_in_death_sequence(&self, die_state: usize, states: &[MobjState]) -> bool {
        // Walk the death sequence from die_state until we hit a terminal state (duration == -1)
        // Check if our current state_idx is any of those states
        let mut s = die_state;
        let mut visited = std::collections::HashSet::new();
        loop {
            if !visited.insert(s) {
                break;
            } // cycle guard
            if self.state_idx == s {
                return true;
            }
            if s >= states.len() {
                break;
            }
            if states[s].duration == -1 {
                break;
            } // terminal
            s = states[s].next_state;
        }
        false
    }

    fn set_state(&mut self, state_idx: usize, states: &[MobjState]) {
        self.state_idx = state_idx;
        self.just_entered_state = true;
        if state_idx < states.len() {
            self.tics = states[state_idx].duration;
        } else {
            self.tics = -1;
        }
    }

    fn execute_action(
        &mut self,
        action: MonsterAction,
        world: &WorldState,
        cmds: &mut Vec<WorldCommand>,
    ) {
        let monster = &world.things[self.thing_idx];

        let (target_pos, target_z) = if let Some(t_idx) = self.target_thing_idx {
            if let Some(t) = world.things.get(t_idx) {
                (t.position, t.z)
            } else {
                (world.player.position, world.player.z)
            }
        } else {
            (world.player.position, world.player.z)
        };
        let target_dist = (target_pos - monster.position).length();

        match action {
            MonsterAction::Look => {
                // Monsters wake up if:
                // 1. Player is within noise radius AND has line of sight, OR
                // 2. Player is within sight radius (increased to 3000 for better aggro) AND has line of sight
                let within_noise = target_dist < world.player.noise_radius;
                let within_sight_range = target_dist < 3000.0;

                if (within_noise || within_sight_range)
                    && world.has_line_of_sight(monster.position, target_pos)
                {
                    // Alert!
                    let run_state = match monster.kind {
                        3004 => S_POSS_RUN,
                        9 => S_SPOS_RUN,
                        3001 => S_TROO_RUN,
                        3002 => S_SARG_RUN,
                        3003 => S_BOSS_RUN,
                        3005 => S_HEAD_RUN,
                        3006 => S_SKULL_RUN,
                        65 => S_CPOS_RUN,
                        66 => S_SKEL_RUN,
                        67 => S_FATT_RUN,
                        68 => S_BSPI_RUN,
                        69 => S_BOS2_RUN,
                        71 => S_PAIN_RUN,
                        64 => S_VILE_RUN,
                        7 => S_SPID_RUN,
                        16 => S_CYBR_RUN,
                        84 => S_SSWV_RUN,
                        _ => S_POSS_RUN,
                    };
                    self.set_state(run_state, STATES);
                    cmds.push(WorldCommand::SpawnAudioEvent(AudioEvent {
                        sound_id: "DSSIGHT".into(),
                        position: Some(monster.position),
                        volume: 1.0,
                    }));
                }
            }
            MonsterAction::Chase => {
                let speed = DEFAULT_THING_DEFS
                    .iter()
                    .find(|&&(k, _)| k == monster.kind)
                    .map(|&(_, d)| d.speed)
                    .unwrap_or(MONSTER_SPEED);
                let dir = (target_pos - monster.position).normalize_or_zero();
                let mut move_vec = dir * speed;

                // Try to move directly
                if !self.try_move(world, move_vec) {
                    // Blocked! Try to slide along the walls that blocked us.
                    let left = Mat2::from_angle(0.785).mul_vec2(move_vec); // 45 deg
                    let right = Mat2::from_angle(-0.785).mul_vec2(move_vec); // -45 deg

                    if !self.try_move(world, left) {
                        if !self.try_move(world, right) {
                            move_vec = Vec2::ZERO;
                        } else {
                            move_vec = right;
                        }
                    } else {
                        move_vec = left;
                    }
                }

                let mut z_move = 0.0;

                // Vertical movement for flying monsters
                if monster.is_flying() {
                    let target_z_eye = target_z + 28.0;
                    if (monster.z - target_z_eye).abs() > 8.0 {
                        z_move = if monster.z < target_z_eye { 2.0 } else { -2.0 };
                    }
                }

                if move_vec.length() > 0.0 {
                    cmds.push(WorldCommand::ModifyThing {
                        thing_idx: self.thing_idx,
                        pos_delta: move_vec,
                        z_delta: z_move,
                        angle: dir.y.atan2(dir.x),
                    });
                }

                // Attack chance - with line-of-sight check and cooldown
                if self.attack_cooldown > 0 {
                    self.attack_cooldown -= 1;
                }
                // INCREASED ATTACK CHANCE: p_random() < 32 (~12% per frame)
                if self.attack_cooldown == 0
                    && target_dist < 2048.0
                    && p_random() < 32
                    && world.has_line_of_sight(monster.position, target_pos)
                {
                    let atk_state = match monster.kind {
                        3004 => S_POSS_ATK,
                        9 => S_SPOS_ATK,
                        3001 => S_TROO_ATK,
                        3002 => S_SARG_ATK,
                        3003 => S_BOSS_ATK,
                        3005 => S_HEAD_ATK,
                        3006 => S_SKULL_ATK,
                        65 => S_CPOS_ATK,
                        66 => S_SKEL_ATK,
                        67 => S_FATT_ATK,
                        68 => S_BSPI_ATK,
                        69 => S_BOS2_ATK,
                        71 => S_PAIN_ATK,
                        64 => S_VILE_ATK,
                        7 => S_SPID_ATK,
                        16 => S_CYBR_ATK,
                        84 => S_SSWV_ATK,
                        _ => S_POSS_ATK,
                    };
                    self.set_state(atk_state, STATES);
                    self.attack_cooldown = 20; // Slightly shorter cooldown
                }
            }
            MonsterAction::SkullAttack => {
                // Lost Soul charge logic
                let speed = DEFAULT_THING_DEFS
                    .iter()
                    .find(|&&(k, _)| k == monster.kind)
                    .map(|&(_, d)| d.speed)
                    .unwrap_or(MONSTER_SPEED);
                let dir = (target_pos - monster.position).normalize_or_zero();
                let z_dir = ((target_z + 20.0) - monster.z).clamp(-1.0, 1.0);
                let move_vec = dir * (speed * 4.0);
                let z_move = z_dir * (speed * 2.0);

                cmds.push(WorldCommand::ModifyThing {
                    thing_idx: self.thing_idx,
                    pos_delta: move_vec,
                    z_delta: z_move,
                    angle: dir.y.atan2(dir.x),
                });

                if target_dist < 48.0 {
                    cmds.push(WorldCommand::DamagePlayer {
                        amount: 10.0,
                        angle: Some(dir.y.atan2(dir.x)),
                    });
                    cmds.push(WorldCommand::SpawnAudioEvent(AudioEvent {
                        sound_id: "DSATK".into(),
                        position: Some(monster.position),
                        volume: 1.0,
                    }));
                    // After hit, stop charging (return to chase state after current state duration)
                }
            }
            MonsterAction::FaceTarget => {
                let dir = (target_pos - monster.position).normalize_or_zero();
                cmds.push(WorldCommand::ModifyThing {
                    thing_idx: self.thing_idx,
                    pos_delta: Vec2::ZERO,
                    z_delta: 0.0,
                    angle: dir.y.atan2(dir.x),
                });
            }
            MonsterAction::PosAttack => {
                let dir = (target_pos - monster.position).normalize_or_zero();
                let angle = dir.y.atan2(dir.x);
                let spread = (p_random() as f32 - 128.0) / 256.0 * 0.1;
                cmds.push(WorldCommand::FireHitscan {
                    origin: monster.position,
                    angle: angle + spread,
                    damage: 10.0,
                    attacker_idx: Some(self.thing_idx),
                });
                cmds.push(WorldCommand::SpawnAudioEvent(AudioEvent {
                    sound_id: "DSPISTOL".into(),
                    position: Some(monster.position),
                    volume: 1.0,
                }));
            }
            MonsterAction::TroopAttack => {
                let dir = (target_pos - monster.position).normalize_or_zero();
                if target_dist < 72.0 {
                    cmds.push(WorldCommand::DamagePlayer {
                        amount: 10.0,
                        angle: Some(dir.y.atan2(dir.x)),
                    });
                    cmds.push(WorldCommand::SpawnAudioEvent(AudioEvent {
                        sound_id: "DSCLAW".into(),
                        position: Some(monster.position),
                        volume: 1.0,
                    }));
                } else {
                    let z_speed = ((target_z + 32.0) - (monster.z + 32.0)) / (target_dist / 20.0);
                    cmds.push(WorldCommand::SpawnProjectile {
                        kind: 10031, // Imp Fireball
                        position: monster.position,
                        z: monster.z + 32.0,
                        velocity: dir * 20.0,
                        z_velocity: z_speed,
                        damage: 10.0,
                        owner_is_player: false,
                        owner_thing_idx: Some(self.thing_idx),
                    });
                }
            }
            MonsterAction::SargAttack => {
                let dir = (target_pos - monster.position).normalize_or_zero();
                if target_dist < 72.0 {
                    let damage = ((p_random() % 10) + 1) as f32 * 4.0;
                    cmds.push(WorldCommand::DamagePlayer {
                        amount: damage,
                        angle: Some(dir.y.atan2(dir.x)),
                    });
                    cmds.push(WorldCommand::SpawnAudioEvent(AudioEvent {
                        sound_id: "DSBGSITE".into(),
                        position: Some(monster.position),
                        volume: 1.0,
                    }));
                }
            }
            MonsterAction::Scream => {
                cmds.push(WorldCommand::SpawnAudioEvent(AudioEvent {
                    sound_id: "DSPDIE".into(),
                    position: Some(monster.position),
                    volume: 1.0,
                }));
            }
            MonsterAction::Fall => {
                // In Doom, Fall makes the monster non-blocking
            }
            MonsterAction::Explode => {
                // Barrel explosion
                let barrel_pos = monster.position;

                cmds.push(WorldCommand::SplashDamage {
                    center: barrel_pos,
                    damage: 128.0,
                    radius: 128.0,
                    owner_is_player: false,
                });

                // Spawn explosion effect
                cmds.push(WorldCommand::SpawnAudioEvent(AudioEvent {
                    sound_id: "DSBAREXP".into(),
                    position: Some(barrel_pos),
                    volume: 1.0,
                }));
            }
            _ => {}
        }
    }
    fn try_move(&self, world: &WorldState, move_vec: Vec2) -> bool {
        let monster = &world.things[self.thing_idx];
        let trial = monster.position + move_vec;
        let radius = DEFAULT_THING_DEFS
            .iter()
            .find(|&&(k, _)| k == monster.kind)
            .map(|&(_, d)| d.radius)
            .unwrap_or(20.0);

        // 1. Collision avoidance against other monsters/player
        // Check against player
        if (trial - world.player.position).length() < radius + PLAYER_RADIUS {
            // Allow sliding off the player if we aren't getting closer
            if (trial - world.player.position).length()
                < (monster.position - world.player.position).length() - 0.01
            {
                return false;
            }
        }

        // Check against other monsters
        for (i, other) in world.things.iter().enumerate() {
            if i == self.thing_idx {
                continue;
            }
            if other.health <= 0.0 || other.picked_up || !other.is_monster() {
                continue;
            }

            let other_radius = DEFAULT_THING_DEFS
                .iter()
                .find(|&&(k, _)| k == other.kind)
                .map(|&(_, d)| d.radius)
                .unwrap_or(20.0);
            if (trial - other.position).length() < radius + other_radius {
                if (trial - other.position).length()
                    < (monster.position - other.position).length() - 0.01
                {
                    return false;
                }
            }
        }

        // 2. Collision avoidance against walls
        for line in &world.linedefs {
            let s = world.vertices[line.start_idx];
            let e = world.vertices[line.end_idx];
            let closest = WorldState::closest_point_on_segment(trial, s, e);
            let dist = (trial - closest).length();

            if dist < radius {
                let closest_old = WorldState::closest_point_on_segment(monster.position, s, e);
                let dist_old = (monster.position - closest_old).length();

                // Only block if we are actually moving geometrically closer to the wall segment
                if dist < dist_old - 0.01 {
                    let mut should_block = true;
                    if line.is_portal() {
                        if let (Some(fs), Some(bs)) = (line.sector_front, line.sector_back) {
                            let front = &world.sectors[fs];
                            let back = &world.sectors[bs];
                            let lowest_ceiling = front.ceiling_height.min(back.ceiling_height);
                            let highest_floor = front.floor_height.max(back.floor_height);
                            let gap = lowest_ceiling - highest_floor;
                            let step_up = highest_floor - monster.z;

                            if gap >= 56.0 && step_up <= STEP_HEIGHT {
                                should_block = false;
                            }
                        }
                    }
                    if should_block {
                        return false;
                    }
                }
            }
        }
        true
    }
}
impl Thinker for MonsterThinker {
    fn on_pain(
        &mut self,
        target_idx: usize,
        target_kind: u16,
        inflictor_idx: Option<usize>,
        _inflictor_kind: Option<u16>,
    ) {
        if self.thing_idx == target_idx {
            // Barrels explode immediately when damaged
            if target_kind == 2035 {
                self.set_state(S_BEXP, STATES);
                return;
            }

            let chance = DEFAULT_THING_DEFS
                .iter()
                .find(|&&(k, _)| k == target_kind)
                .map(|&(_, d)| d.pain_chance)
                .unwrap_or(0);
            if p_random() < chance {
                let pain_state = match target_kind {
                    3004 => S_POSS_PAIN,
                    9 => S_SPOS_PAIN,
                    3001 => S_TROO_PAIN,
                    3002 => S_SARG_PAIN,
                    3003 => S_BOSS_PAIN,
                    3005 => S_HEAD_PAIN,
                    3006 => S_SKULL_PAIN,
                    65 => S_CPOS_PAIN,
                    66 => S_SKEL_PAIN,
                    67 => S_FATT_PAIN,
                    68 => S_BSPI_PAIN,
                    69 => S_BOS2_PAIN,
                    71 => S_PAIN_PAIN,
                    64 => S_VILE_PAIN,
                    7 => S_SPID_PAIN,
                    16 => S_CYBR_PAIN,
                    84 => S_SSWV_PAIN,
                    _ => S_POSS_PAIN,
                };
                self.set_state(pain_state, STATES);

                if let Some(inflictor) = inflictor_idx {
                    if inflictor != self.thing_idx {
                        self.target_thing_idx = Some(inflictor);
                    }
                }
            }
        }
    }

    fn on_wake(&mut self, thing_idx: usize) {
        if self.thing_idx == thing_idx {
            // Monster was woken by noise - switch to chase state if not already active
            // This implements Doom's "monsters hear through doors" behavior
            // Only wake up if in a non-active state (Look/Pain/Idle states)
            // Check if monster is in a Look/idle state (any monster type)
            let current_state = self.state_idx;
            let idle_to_run: Option<usize> = match current_state {
                s if s == S_POSS_STND || s == S_POSS_STND + 1 => Some(S_POSS_RUN),
                s if s == S_SPOS_STND || s == S_SPOS_STND + 1 => Some(S_SPOS_RUN),
                s if s == S_TROO_STND || s == S_TROO_STND + 1 => Some(S_TROO_RUN),
                s if s == S_SARG_STND || s == S_SARG_STND + 1 => Some(S_SARG_RUN),
                s if s == S_HEAD_STND || s == S_HEAD_STND + 1 => Some(S_HEAD_RUN),
                s if s == S_BOSS_STND || s == S_BOSS_STND + 1 => Some(S_BOSS_RUN),
                s if s == S_SKULL_STND || s == S_SKULL_STND + 1 => Some(S_SKULL_RUN),
                s if s == S_CPOS_STND || s == S_CPOS_STND + 1 => Some(S_CPOS_RUN),
                s if s == S_SKEL_STND || s == S_SKEL_STND + 1 => Some(S_SKEL_RUN),
                s if s == S_FATT_STND || s == S_FATT_STND + 1 => Some(S_FATT_RUN),
                s if s == S_BSPI_STND || s == S_BSPI_STND + 1 => Some(S_BSPI_RUN),
                s if s == S_BOS2_STND || s == S_BOS2_STND + 1 => Some(S_BOS2_RUN),
                s if s == S_PAIN_STND || s == S_PAIN_STND + 1 => Some(S_PAIN_RUN),
                s if s == S_VILE_STND || s == S_VILE_STND + 1 => Some(S_VILE_RUN),
                s if s == S_SPID_STND || s == S_SPID_STND + 1 => Some(S_SPID_RUN),
                s if s == S_CYBR_STND || s == S_CYBR_STND + 1 => Some(S_CYBR_RUN),
                s if s == S_SSWV_STND || s == S_SSWV_STND + 1 => Some(S_SSWV_RUN),
                _ => None,
            };
            if let Some(run_state) = idle_to_run {
                self.set_state(run_state, STATES);
            }
        }
    }

    fn update(&mut self, world: &WorldState) -> (bool, Vec<WorldCommand>) {
        let monster = match world.things.get(self.thing_idx) {
            Some(m) => m,
            None => return (false, vec![]),
        };

        if monster.health <= 0.0 {
            let die_state = match monster.kind {
                3004 => S_POSS_DIE,
                9 => S_SPOS_DIE,
                3001 => S_TROO_DIE,
                3002 => S_SARG_DIE,
                3003 => S_BOSS_DIE,
                3005 => S_HEAD_DIE,
                3006 => S_SKULL_DIE,
                65 => S_CPOS_DIE,
                66 => S_SKEL_DIE,
                67 => S_FATT_DIE,
                68 => S_BSPI_DIE,
                69 => S_BOS2_DIE,
                71 => S_PAIN_DIE,
                64 => S_VILE_DIE,
                7 => S_SPID_DIE,
                16 => S_CYBR_DIE,
                84 => S_SSWV_DIE,
                2035 => S_BEXP,
                _ => S_POSS_DIE,
            };
            // Check if NOT already in a death sequence
            if !self.is_in_death_sequence(die_state, STATES) {
                self.set_state(die_state, STATES);
            }
        }

        let mut commands = Vec::new();

        if self.tics > 0 {
            self.tics -= 1;
        }

        if self.tics == 0 {
            if self.state_idx < STATES.len() {
                let next = STATES[self.state_idx].next_state;
                self.set_state(next, STATES);
            }
        }

        // Fire action only on state entry (matching vanilla Doom behavior).
        // In Doom, P_SetMobjState calls the action function once when the state is set.
        if self.just_entered_state {
            self.just_entered_state = false;
            if self.state_idx < STATES.len() {
                if let Some(action) = STATES[self.state_idx].action {
                    self.execute_action(action, world, &mut commands);
                }
            }
        }

        // Sync tics back to thing.ai_timer for save/load preservation
        // This ensures the AI state is saved with the thing
        commands.push(WorldCommand::SyncAiState {
            thing_idx: self.thing_idx,
            state_idx: self.state_idx,
            timer: self.tics.max(0) as u32,
            target: self.target_thing_idx,
            cooldown: self.attack_cooldown,
        });

        // Gravity and step logic for ALL ground monsters
        if !monster.is_flying() {
            if let Some(s_idx) = world.find_sector_at(monster.position) {
                let floor_z = world.sectors[s_idx].floor_height;
                let mut z_snap = 0.0;
                if monster.z > floor_z {
                    // Fall down
                    z_snap = -8.0;
                    if monster.z + z_snap < floor_z {
                        z_snap = floor_z - monster.z;
                    }
                } else if monster.z < floor_z {
                    // Step up climbing
                    z_snap = floor_z - monster.z;
                }

                if z_snap != 0.0 {
                    commands.push(WorldCommand::ModifyThing {
                        thing_idx: self.thing_idx,
                        pos_delta: Vec2::ZERO,
                        z_delta: z_snap,
                        angle: monster.angle,
                    });
                }
            }
        }

        // Keep the thinker alive unless in a terminal death state (duration == -1)
        let keep = if monster.health <= 0.0 {
            // Still animating death sequence — keep until final frame
            self.state_idx < STATES.len() && STATES[self.state_idx].duration != -1
        } else {
            true
        };
        (keep, commands)
    }
}
