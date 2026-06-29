//! Skeleton: map-geometry interaction (line specials, doors, lifts, crushers,
//! stairs) and sector-based environmental damage.
//!
//! Real items to move here from `src/doom.rs` (currently `DoomWorldExt`
//! methods). Per the ADR these are exposed as crate-internal free fns that the
//! `DoomWorldExt` impl in `doom::world` delegates to; none are referenced
//! outside `doom`:
//! - `is_walk_trigger`
//! - `activate_linedef_manual`, `activate_linedef`
//! - `find_lowest_adjacent_ceiling`, `trigger_door`
//! - `do_door_tagged`, `do_lift_tagged`, `do_crusher_tagged`, `do_stairs_tagged`
//! - `update_environmental_damage`

use aetheris::simulation::{WorldCommand, WorldState};

/// Returns true if the linedef special type is a walk-trigger (W1/WR).
pub fn is_walk_trigger(special: u16) -> bool {
    unimplemented!("moved verbatim from doom.rs during B-exec")
}

pub fn activate_linedef_manual(
    world: &mut WorldState,
    line_idx: usize,
    override_back: Option<usize>,
    cmds: &mut Vec<WorldCommand>,
) {
    unimplemented!("moved verbatim from doom.rs during B-exec")
}

pub fn activate_linedef(
    world: &mut WorldState,
    special: u16,
    tag: u16,
    sector_back: Option<usize>,
    cmds: &mut Vec<WorldCommand>,
) {
    unimplemented!("moved verbatim from doom.rs during B-exec")
}

pub fn find_lowest_adjacent_ceiling(world: &WorldState, sector_idx: usize) -> f32 {
    unimplemented!("moved verbatim from doom.rs during B-exec")
}

pub fn trigger_door(world: &mut WorldState, sector_idx: usize, speed: f32, wait: f32) -> bool {
    unimplemented!("moved verbatim from doom.rs during B-exec")
}

pub fn do_door_tagged(world: &mut WorldState, tag: u16, speed: f32, wait: f32) -> bool {
    unimplemented!("moved verbatim from doom.rs during B-exec")
}

pub fn do_lift_tagged(world: &mut WorldState, tag: u16) {
    unimplemented!("moved verbatim from doom.rs during B-exec")
}

pub fn do_crusher_tagged(world: &mut WorldState, tag: u16, speed: f32, damage: f32) {
    unimplemented!("moved verbatim from doom.rs during B-exec")
}

pub fn do_stairs_tagged(world: &mut WorldState, tag: u16, step_height: f32) {
    unimplemented!("moved verbatim from doom.rs during B-exec")
}

pub fn update_environmental_damage(world: &mut WorldState) {
    unimplemented!("moved verbatim from doom.rs during B-exec")
}
