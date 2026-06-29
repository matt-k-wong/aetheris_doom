//! Skeleton: damage delivery and projectile/puff lifetime.
//!
//! Real items to move here from `src/doom.rs`:
//! - `PuffThinker`, `ProjectileThinker` and their `Thinker` impls
//! - `PROJECTILE_RADIUS` (becomes module-private)
//! - `DoomWorldExt::fire_hitscan` (as a delegated free fn — see ADR
//!   "Trait-splitting note")
//! - the per-weapon firing `match` currently inlined in `DoomWorldExt::update`,
//!   extracted into a `fire_weapon(...)` helper
//!
//! These thinkers' fields are read by `world`/`apply_commands` when spawning, so
//! the structs stay `pub`.

use aetheris::simulation::{Thinker, Vertex, WorldCommand, WorldState};
use glam::Vec2;

const PROJECTILE_RADIUS: f32 = 10.0;

/// Short-lived bullet-puff / blood marker.
pub struct PuffThinker {
    pub position: Vec2,
    pub timer: i32,
}

impl Thinker for PuffThinker {
    fn update(&mut self, world: &WorldState) -> (bool, Vec<WorldCommand>) {
        unimplemented!("moved verbatim from doom.rs during B-exec")
    }
    fn on_pain(&mut self, _: usize, _: u16, _: Option<usize>, _: Option<u16>) {}
    fn on_wake(&mut self, _: usize) {}
}

/// Moving projectile (fireball / rocket / plasma / BFG ball).
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

impl Thinker for ProjectileThinker {
    fn update(&mut self, world: &WorldState) -> (bool, Vec<WorldCommand>) {
        unimplemented!("moved verbatim from doom.rs during B-exec")
    }
    fn on_pain(&mut self, _: usize, _: u16, _: Option<usize>, _: Option<u16>) {}
    fn on_wake(&mut self, _: usize) {}
}

/// Hitscan trace against walls/things/player (delegated from `DoomWorldExt`).
pub fn fire_hitscan(
    world: &mut WorldState,
    origin: Vertex,
    angle: f32,
    damage: f32,
    attacker_idx: Option<usize>,
) {
    unimplemented!("extracted from DoomWorldExt::fire_hitscan during B-exec")
}
