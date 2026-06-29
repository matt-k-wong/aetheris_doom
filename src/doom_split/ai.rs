//! Skeleton: monster behavior (the `MonsterThinker` and its action dispatch).
//!
//! Real items to move here from `src/doom.rs`:
//! - `MonsterThinker` struct
//! - `impl MonsterThinker` (`new`, `is_in_death_sequence`, `set_state`,
//!   `execute_action`, `try_move`)
//! - `impl Thinker for MonsterThinker` (`on_pain`, `on_wake`, `update`)
//!
//! `MonsterThinker` + `MonsterThinker::new` are consumed by `main.rs`, so they
//! stay `pub` and are re-exported as `crate::doom::MonsterThinker`.
//!
//! AI emits combat via `WorldCommand`s (`FireHitscan`, `SpawnProjectile`,
//! `SplashDamage`, …) rather than calling `doom::combat` directly — keep that
//! decoupling during B-exec.

use aetheris::simulation::{Thinker, WorldCommand, WorldState};

use super::states::MonsterAction;

/// Per-monster AI thinker.
pub struct MonsterThinker {
    pub thing_idx: usize,
    pub state_idx: usize,
    pub tics: i32,
    pub target_thing_idx: Option<usize>,
    pub attack_cooldown: u32,
    pub just_entered_state: bool,
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
            just_entered_state: true,
        }
    }

    fn execute_action(
        &mut self,
        action: MonsterAction,
        world: &WorldState,
        cmds: &mut Vec<WorldCommand>,
    ) {
        unimplemented!("moved verbatim from doom.rs during B-exec")
    }

    fn try_move(&self, world: &WorldState, move_vec: glam::Vec2) -> bool {
        unimplemented!("moved verbatim from doom.rs during B-exec")
    }
}

impl Thinker for MonsterThinker {
    fn update(&mut self, world: &WorldState) -> (bool, Vec<WorldCommand>) {
        unimplemented!("moved verbatim from doom.rs during B-exec")
    }
    fn on_pain(
        &mut self,
        target_idx: usize,
        target_kind: u16,
        inflictor_idx: Option<usize>,
        inflictor_kind: Option<u16>,
    ) {
        unimplemented!("moved verbatim from doom.rs during B-exec")
    }
    fn on_wake(&mut self, thing_idx: usize) {
        unimplemented!("moved verbatim from doom.rs during B-exec")
    }
}
