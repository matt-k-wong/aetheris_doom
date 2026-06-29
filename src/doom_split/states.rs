//! Skeleton: the actor state machine.
//!
//! Real items to move here from `src/doom.rs`:
//! - `MonsterAction` enum and `MobjState` struct
//! - all `S_*` state-index constants (per-monster STND/RUN/ATK/PAIN/DIE)
//! - `STATES` (alias of `DEFAULT_STATES`) and the `DEFAULT_STATES` table
//! - `get_start_state(kind)`
//!
//! `STATES` is consumed by `main.rs` and `bridge.rs`, so it stays `pub` and is
//! re-exported as `crate::doom::STATES`.

/// Per-state action invoked on state entry (vanilla `P_SetMobjState` semantics).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MonsterAction {
    Look,
    Chase,
    FaceTarget,
    PosAttack,
    SPosAttack,
    TroopAttack,
    SargAttack,
    HeadAttack,
    BruisAttack,
    SkelMissile,
    FatAttack,
    VileChase,
    VileAttack,
    PainAttack,
    Pain,
    Scream,
    Fall,
    Explode,
    Raise,
    SkullAttack,
}

/// One row of the actor state table.
#[derive(Clone, Copy)]
pub struct MobjState {
    pub sprite: &'static str,
    pub frame: char,
    pub duration: i32,
    pub action: Option<MonsterAction>,
    pub next_state: usize,
}

// Representative subset of the `S_*` indices; the full set moves here during
// B-exec. Many of these are only read by `doom::ai`.
pub const S_NULL: usize = 0;
pub const S_POSS_STND: usize = 1;
pub const S_POSS_RUN: usize = 3;

/// The active state table. Placeholder; B-exec moves the real `DEFAULT_STATES`.
pub const DEFAULT_STATES: &[MobjState] = &[];

/// Public alias used across the crate (and re-exported as `crate::doom::STATES`).
pub const STATES: &[MobjState] = DEFAULT_STATES;

/// Map a map thing-id to its starting state index.
pub fn get_start_state(kind: u16) -> usize {
    unimplemented!("moved verbatim from doom.rs during B-exec")
}
