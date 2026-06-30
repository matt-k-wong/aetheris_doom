use aetheris::simulation::{AudioEvent, Thing, Thinker, Vertex, WorldCommand, WorldState};
use glam::Vec2;

use super::defs::DoomThingExt;
pub struct PuffThinker {
    pub position: glam::Vec2,
    pub timer: i32,
}
impl Thinker for PuffThinker {
    fn update(&mut self, _: &WorldState) -> (bool, Vec<WorldCommand>) {
        self.timer -= 1;
        (self.timer > 0, vec![])
    }
    fn on_pain(&mut self, _: usize, _: u16, _: Option<usize>, _: Option<u16>) {}
    fn on_wake(&mut self, _: usize) {}
}

pub struct ProjectileThinker {
    pub thing_idx: usize,
    pub position: Vertex,
    pub z: f32,
    pub velocity: Vec2,
    pub z_velocity: f32,
    pub damage: f32,
    pub owner_is_player: bool,
    pub owner_thing_idx: Option<usize>,
}

const PROJECTILE_RADIUS: f32 = 10.0;

impl Thinker for ProjectileThinker {
    fn update(&mut self, world: &WorldState) -> (bool, Vec<WorldCommand>) {
        let speed = self.velocity.length();
        let steps = (speed / PROJECTILE_RADIUS).ceil().max(1.0) as u32;
        let step_vec = self.velocity / steps as f32;
        let z_step = self.z_velocity / steps as f32;

        let mut current_pos = self.position;
        let mut current_z = self.z;
        let mut cmds = Vec::new();

        for _ in 0..steps {
            let next = current_pos + step_vec;
            let next_z = current_z + z_step;
            cmds.push(WorldCommand::ModifyThing {
                thing_idx: self.thing_idx,
                pos_delta: step_vec,
                z_delta: z_step,
                angle: 0.0,
            });

            for line in &world.linedefs {
                if line.is_portal() {
                    continue;
                }
                let start = match world.vertices.get(line.start_idx) {
                    Some(v) => *v,
                    None => continue,
                };
                let end = match world.vertices.get(line.end_idx) {
                    Some(v) => *v,
                    None => continue,
                };

                let closest = WorldState::closest_point_on_segment(next, start, end);
                if (next - closest).length() < PROJECTILE_RADIUS {
                    cmds.push(WorldCommand::SpawnAudioEvent(AudioEvent {
                        sound_id: "DSBAREXP".into(),
                        position: Some(closest),
                        volume: 1.0,
                    }));
                    cmds.push(WorldCommand::SetThingHealth {
                        thing_idx: self.thing_idx,
                        health: 0.0,
                    });

                    if self.damage >= 20.0 {
                        cmds.push(WorldCommand::SplashDamage {
                            center: closest,
                            damage: 128.0,
                            radius: 128.0,
                            owner_is_player: self.owner_is_player,
                        });
                        cmds.push(WorldCommand::SpawnThing {
                            kind: 9999,
                            position: closest,
                            z: next_z,
                            angle: 0.0,
                        });

                        if self.damage >= 100.0 {
                            let bfg_origin = self
                                .owner_thing_idx
                                .map(|idx| world.things.get(idx).map(|t| t.position))
                                .flatten()
                                .unwrap_or(closest);
                            for i in 0..40 {
                                let angle_offset =
                                    (i as f32 / 40.0 - 0.5) * std::f32::consts::PI * 0.5;
                                let tracer_angle =
                                    (closest - bfg_origin).y.atan2((closest - bfg_origin).x)
                                        + angle_offset
                                        + (rand::random::<f32>() - 0.5) * 0.2;
                                cmds.push(WorldCommand::FireHitscan {
                                    origin: closest,
                                    angle: tracer_angle,
                                    damage: 15.0,
                                    attacker_idx: self.owner_thing_idx,
                                });
                            }
                        }
                    } else {
                        cmds.push(WorldCommand::SpawnAudioEvent(AudioEvent {
                            sound_id: "DSOWHIT".into(),
                            position: Some(closest),
                            volume: 0.5,
                        }));
                        cmds.push(WorldCommand::SpawnThing {
                            kind: 9998,
                            position: closest,
                            z: next_z,
                            angle: 0.0,
                        });
                    }
                    return (false, cmds);
                }
            }
            if self.owner_is_player {
                for (i, t) in world.things.iter().enumerate() {
                    if (t.is_monster() || t.is_barrel())
                        && !t.picked_up
                        && t.health > 0.0
                        && (next - t.position).length() < 20.0
                        && (next_z - t.z).abs() < 40.0
                    {
                        cmds.push(WorldCommand::DamageThing {
                            thing_idx: i,
                            amount: self.damage,
                            inflictor_idx: self.owner_thing_idx,
                        });
                        cmds.push(WorldCommand::InflictPain {
                            thing_idx: i,
                            inflictor_idx: self.owner_thing_idx,
                        });
                        cmds.push(WorldCommand::SpawnAudioEvent(AudioEvent {
                            sound_id: "DSPOPAIN".into(),
                            position: Some(t.position),
                            volume: 1.0,
                        }));
                        cmds.push(WorldCommand::SetThingHealth {
                            thing_idx: self.thing_idx,
                            health: 0.0,
                        });

                        if self.damage >= 20.0 {
                            cmds.push(WorldCommand::SplashDamage {
                                center: t.position,
                                damage: 128.0,
                                radius: 128.0,
                                owner_is_player: true,
                            });
                            let b_kind = if t.kind == 3003 || t.kind == 3005 {
                                9997
                            } else {
                                9999
                            };
                            cmds.push(WorldCommand::SpawnThing {
                                kind: b_kind,
                                position: t.position,
                                z: t.z + 20.0,
                                angle: 0.0,
                            });
                        }
                        return (false, cmds);
                    }
                }
            } else {
                for (i, t) in world.things.iter().enumerate() {
                    if i == self.thing_idx {
                        continue;
                    }
                    if let Some(owner) = self.owner_thing_idx {
                        if i == owner {
                            continue;
                        }
                    }

                    if (t.is_monster() || t.is_barrel())
                        && !t.picked_up
                        && t.health > 0.0
                        && (next - t.position).length() < 20.0
                        && (next_z - t.z).abs() < 40.0
                    {
                        cmds.push(WorldCommand::DamageThing {
                            thing_idx: i,
                            amount: self.damage,
                            inflictor_idx: self.owner_thing_idx,
                        });
                        cmds.push(WorldCommand::InflictPain {
                            thing_idx: i,
                            inflictor_idx: self.owner_thing_idx,
                        });
                        cmds.push(WorldCommand::SetThingHealth {
                            thing_idx: self.thing_idx,
                            health: 0.0,
                        });
                        return (false, cmds);
                    }
                }

                if (next - world.player.position).length() < 20.0
                    && (next_z - world.player.z).abs() < 40.0
                {
                    cmds.push(WorldCommand::DamagePlayer {
                        amount: self.damage,
                        angle: Some(
                            (world.player.position - next)
                                .y
                                .atan2((world.player.position - next).x),
                        ),
                    });
                    cmds.push(WorldCommand::SpawnAudioEvent(AudioEvent {
                        sound_id: "DSPLPAIN".into(),
                        position: Some(world.player.position),
                        volume: 1.0,
                    }));
                    cmds.push(WorldCommand::SetThingHealth {
                        thing_idx: self.thing_idx,
                        health: 0.0,
                    });
                    return (false, cmds);
                }
            }
            current_pos = next;
            current_z = next_z;
        }
        self.position = current_pos;
        self.z = current_z;
        (true, cmds)
    }

    fn on_pain(
        &mut self,
        _target_idx: usize,
        _target_kind: u16,
        _inflictor_idx: Option<usize>,
        _inflictor_kind: Option<u16>,
    ) {
    }
    fn on_wake(&mut self, _thing_idx: usize) {}
}

pub(crate) fn fire_hitscan(
    world: &mut WorldState,
    origin: aetheris::simulation::Vertex,
    angle: f32,
    damage: f32,
    attacker_idx: Option<usize>,
) {
    let dir = glam::Vec2::new(angle.cos(), angle.sin());
    let max_dist = 2000.0;
    let end = origin + dir * max_dist;

    let mut best_dist = max_dist;
    let mut hit_thing_idx = None;
    let mut hit_player = false;
    let mut hit_pos = end;
    for line in &world.linedefs {
        if line.is_portal() {
            continue;
        }
        let p3 = world.vertices[line.start_idx];
        let p4 = world.vertices[line.end_idx];
        if let Some(hit) = WorldState::intersect(origin, end, p3, p4) {
            let d = (hit - origin).length();
            if d < best_dist {
                best_dist = d;
                hit_pos = hit;
                hit_thing_idx = None;
                hit_player = false;
            }
        }
    }

    for (i, thing) in world.things.iter().enumerate() {
        if (!thing.is_monster() && !thing.is_barrel()) || thing.health <= 0.0 || thing.picked_up {
            continue;
        }
        if attacker_idx == Some(i) {
            continue;
        }
        let v = thing.position - origin;
        let t = v.dot(dir);
        if t < 0.0 || t > best_dist {
            continue;
        }

        let closest = origin + dir * t;
        let dist_sq = (thing.position - closest).length_squared();
        if dist_sq < (20.0 * 20.0) {
            best_dist = t;
            hit_pos = closest;
            hit_thing_idx = Some(i);
            hit_player = false;
        }
    }

    if attacker_idx.is_some() && world.player.health > 0.0 {
        let v = world.player.position - origin;
        let t = v.dot(dir);
        if t >= 0.0 && t < best_dist {
            let closest = origin + dir * t;
            let dist_sq = (world.player.position - closest).length_squared();
            if dist_sq < (20.0 * 20.0) {
                best_dist = t;
                hit_pos = closest;
                hit_thing_idx = None;
                hit_player = true;
            }
        }
    }

    if let Some(idx) = hit_thing_idx {
        let t_kind = world.things[idx].kind;
        let i_kind = attacker_idx.and_then(|id| world.things.get(id).map(|th| th.kind));

        if let Some(t) = world.things.get_mut(idx) {
            t.health -= damage;

            for i in 0..world.thinkers.len() {
                let mut thinker = world.thinkers.remove(i);
                thinker.on_pain(idx, t_kind, attacker_idx, i_kind);
                world.thinkers.insert(i, thinker);
            }

            let b_kind = if t_kind == 3003 || t_kind == 3005 {
                9997
            } else {
                9999
            };
            let puff = Thing {
                position: hit_pos,
                angle: 0.0,
                kind: b_kind,
                flags: 0,
                health: 10.0,
                picked_up: false,
                state_idx: 0,
                ai_timer: 0,
                target_thing_idx: None,
                attack_cooldown: 0,
                z: 0.0,
            };
            super::world::DoomWorldExt::spawn_effect_thing(world, puff);
            world.thinkers.push(Box::new(PuffThinker {
                position: hit_pos,
                timer: 15,
            }));
        }
        if let Some(t) = world.things.get(idx) {
            world.audio_events.push(AudioEvent {
                sound_id: "DSPOPAIN".into(),
                position: Some(t.position),
                volume: 1.0,
            });
        }
    } else if hit_player {
        if world.player.invuln_timer == 0 {
            let absorbed = (damage * 0.333).min(world.player.armor);
            world.player.armor -= absorbed;
            world.player.health -= damage - absorbed;
            if world.player.damage_flash < 0.1 {
                world.player.damage_flash = 0.5;
            }
            world.player.last_damage_angle = Some(angle);
        }
        let puff = Thing {
            position: hit_pos,
            angle: 0.0,
            kind: 9999, // Blood
            flags: 0,
            health: 10.0,
            picked_up: false,
            state_idx: 0,
            ai_timer: 0,
            target_thing_idx: None,
            attack_cooldown: 0,
            z: 0.0,
        };
        super::world::DoomWorldExt::spawn_effect_thing(world, puff);
        world.thinkers.push(Box::new(PuffThinker {
            position: hit_pos,
            timer: 15,
        }));
        world.audio_events.push(AudioEvent {
            sound_id: "DSPLPAIN".into(),
            position: Some(world.player.position),
            volume: 1.0,
        });
    } else if best_dist < max_dist {
        world.audio_events.push(AudioEvent {
            sound_id: "DSNOWHIT".into(),
            position: Some(hit_pos),
            volume: 0.5,
        });
        let puff = Thing {
            position: hit_pos,
            angle: 0.0,
            kind: 9998,
            flags: 0,
            health: 5.0,
            picked_up: false,
            state_idx: 0,
            ai_timer: 0,
            target_thing_idx: None,
            attack_cooldown: 0,
            z: 0.0,
        };
        super::world::DoomWorldExt::spawn_effect_thing(world, puff);
    }
}
