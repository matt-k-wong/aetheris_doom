//! Skeleton: the per-tick simulation driver and `WorldCommand` interpreter.
//!
//! Real items to move here from `src/doom.rs`:
//! - the `DoomWorldExt` trait
//! - `impl DoomWorldExt for WorldState`: `update`, `apply_commands`,
//!   `spawn_effect_thing`, `spread_noise`
//! - `init_world`
//!
//! Per the ADR's "Trait-splitting note", `DoomWorldExt` stays whole here and its
//! geometry/combat methods delegate to free fns in `doom::linedefs` /
//! `doom::combat`. `update` / `apply_commands` are consumed by `main.rs`, so the
//! trait stays `pub` and is re-exported as `crate::doom::DoomWorldExt`.

use aetheris::simulation::{GameAction, Thing, Vertex, WorldCommand, WorldState};
use std::collections::HashSet;

/// Doom-specific `WorldState` methods: the update loop, command application, and
/// linedef/door activation (the latter delegate into `doom::linedefs`).
pub trait DoomWorldExt {
    fn is_walk_trigger(special: u16) -> bool;
    fn spread_noise(&mut self, start_sid: usize, hops: u32);
    fn spawn_effect_thing(&mut self, thing: Thing) -> usize;
    fn fire_hitscan(
        &mut self,
        origin: Vertex,
        angle: f32,
        damage: f32,
        attacker_idx: Option<usize>,
    );
    fn update(&mut self, actions: &HashSet<GameAction>);
    fn apply_commands(&mut self, cmds: Vec<WorldCommand>);
    fn activate_linedef_manual(
        &mut self,
        line_idx: usize,
        override_back: Option<usize>,
        cmds: &mut Vec<WorldCommand>,
    );
    fn activate_linedef(
        &mut self,
        special: u16,
        tag: u16,
        sector_back: Option<usize>,
        cmds: &mut Vec<WorldCommand>,
    );
    fn find_lowest_adjacent_ceiling(&self, sector_idx: usize) -> f32;
    fn trigger_door(&mut self, sector_idx: usize, speed: f32, wait: f32) -> bool;
    fn do_door_tagged(&mut self, tag: u16, speed: f32, wait: f32) -> bool;
    fn do_lift_tagged(&mut self, tag: u16);
    fn do_crusher_tagged(&mut self, tag: u16, speed: f32, damage: f32);
    fn do_stairs_tagged(&mut self, tag: u16, step_height: f32);
    fn update_environmental_damage(&mut self);
}

/// No-op hook kept for API parity (matches `doom::init_world`).
pub fn init_world(_world: &mut WorldState) {}
