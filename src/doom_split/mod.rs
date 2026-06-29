//! Compiling skeleton for the planned `src/doom/` decomposition.
//!
//! This module is a **non-authoritative scaffold** produced by the `B-design`
//! node and described in `docs/adr/0001-module-split.md`. It mirrors the target
//! module tree's public signatures so reviewers can see the shape of the split
//! before any code is moved.
//!
//! It is intentionally wired under `doom_split` (not `doom`) because the live
//! `src/doom.rs` still owns the real logic, and Rust cannot have both
//! `src/doom.rs` and `src/doom/mod.rs`. The `B-exec` node renames
//! `src/doom.rs` -> `src/doom/mod.rs`, moves the real items into these modules,
//! and deletes this scaffold.
//!
//! Nothing here changes runtime behavior: every function body is a placeholder
//! (`unimplemented!()` / trivial return) and the module is not referenced by the
//! game. The whole tree allows dead code so the skeleton compiles cleanly.
#![allow(dead_code, unused_variables, unused_imports, static_mut_refs)]

pub mod ai;
pub mod combat;
pub mod defs;
pub mod linedefs;
pub mod rng;
pub mod states;
pub mod world;

// In the real `src/doom/mod.rs`, these re-exports preserve the existing
// `crate::doom::*` public API consumed by `main.rs` / `bridge.rs` / `lib.rs`:
//
//   pub use ai::MonsterThinker;
//   pub use defs::DoomThingExt;
//   pub use states::{STATES, get_start_state};
//   pub use world::DoomWorldExt;
