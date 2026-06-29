//! Skeleton: static actor data and kind-based classification.
//!
//! Real items to move here from `src/doom.rs`:
//! - thing-type id constants: `MONSTER_*`, `ITEM_*`, `KEY_*`, `EFFECT_*`
//! - `ThingDef` struct and the `DEFAULT_THING_DEFS` table
//! - the `DoomThingExt` trait and its `impl … for Thing`
//!
//! `DoomThingExt::initial_health` / `pain_chance` read `DEFAULT_THING_DEFS`, so
//! the table and the trait belong in the same module.

use aetheris::simulation::{Thing, WorldState};

/// Per-actor static stats (health/speed/radius/etc.).
#[derive(Clone, Copy)]
pub struct ThingDef {
    pub health: f32,
    pub speed: f32,
    pub radius: f32,
    pub height: f32,
    pub damage: i32,
    pub reaction_time: i32,
    pub pain_chance: u8,
    pub mass: i32,
}

// Representative subset of the thing-type id constants; the full set
// (`MONSTER_*`, `ITEM_*`, `KEY_*`, `EFFECT_*`) moves here during B-exec.
pub const MONSTER_ZOMBIEMAN: u16 = 3004;
pub const ITEM_SHOTGUN: u16 = 2001;
pub const KEY_BLUE: u16 = 5;
pub const EFFECT_BLOOD: u16 = 9999;

/// Actor table keyed by map thing-id. Placeholder; the real table is filled in
/// during B-exec.
pub const DEFAULT_THING_DEFS: &[(u16, ThingDef)] = &[];

/// Doom-specific extension methods on the engine's `Thing` type.
///
/// `is_monster` / `is_barrel` are consumed by `main.rs` and `bridge.rs`, so this
/// trait stays `pub` and is re-exported as `crate::doom::DoomThingExt`.
pub trait DoomThingExt {
    fn is_monster(&self) -> bool;
    fn is_flying(&self) -> bool;
    fn is_pickup(&self) -> bool;
    fn is_barrel(&self) -> bool;
    fn is_effect(&self) -> bool;
    fn initial_health(k: u16, world: &WorldState) -> f32;
    fn pain_chance(k: u16, world: &WorldState) -> u8;
    fn sprite_name<'a>(&self, world: &'a WorldState) -> &'a str;
    fn frame_char(&self, world: &WorldState) -> char;
}
