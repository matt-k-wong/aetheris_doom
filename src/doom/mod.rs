//! Doom game logic module tree (ADR 0001).
#![allow(static_mut_refs)]

pub mod ai;
pub mod combat;
pub mod defs;
pub mod linedefs;
pub mod rng;
pub mod states;
pub mod world;

pub use ai::MonsterThinker;
pub use defs::DoomThingExt;
pub use states::{STATES, get_start_state};
pub use world::DoomWorldExt;
