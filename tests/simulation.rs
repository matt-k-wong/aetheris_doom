//! Deterministic, headless simulation tests for `aetheris_doom`.
//!
//! Scaffolding produced by ADR 0002 (`docs/adr/0002-test-harness.md`). These are
//! COMPILING STUBS: the C-exec node fills in the real assertions. Every non-ignored stub
//! must compile and pass today (no `todo!()` in a running test), so the bodies are
//! deliberately trivial placeholders that still touch the public API surface they will
//! eventually exercise.
//!
//! Determinism rule (see ADR 0002 §Determinism): any test that can reach `p_random()`
//! must anchor the global RNG with `reset_rng_to(seed)` before acting.

use aetheris_doom::doom;

/// RNG sequence determinism: anchoring with `reset_rng_to(seed)` must reproduce the same
/// `p_random()` subsequence every run, and re-anchoring to the same seed must repeat it.
#[test]
fn rng_sequence_is_deterministic() {
    // TODO(C-exec): assert that `reset_rng_to(seed)` + N `p_random()` calls yields a
    // fixed sequence anchored in `PRND_TABLE`, and that re-seeding reproduces it exactly
    // (and that the index wraps mod 256).
    doom::reset_rng_to(0);
    let _first = doom::p_random();
    doom::reset_rng();
    let _second = doom::p_random();
    assert!(true);
}

/// Monster state-machine progression: walking `STATES` via `next_state` from each
/// monster's start state must terminate (terminal frame has `duration == -1`) without
/// indexing out of bounds.
#[test]
fn monster_state_progression_terminates() {
    // TODO(C-exec): for each known monster kind, start at `get_start_state(kind)` and
    // walk `next_state` (with a cycle guard) asserting indices stay in-bounds and a
    // terminal state is reachable; also assert `MonsterThinker::set_state` sets `tics`
    // from the entered state's `duration`.
    let start = doom::get_start_state(3001); // Imp
    assert!(start < doom::STATES.len());
    assert!(true);
}

/// Linedef / door trigger: applying a triggering action against a minimal `WorldState`
/// fixture emits the expected `WorldCommand`(s).
///
/// Ignored until C-exec adds the `WorldState` fixture builder described in ADR 0002
/// (the engine `WorldState` has no cheap public constructor available to the scaffold).
#[test]
#[ignore = "C-exec: needs minimal WorldState fixture builder (ADR 0002)"]
fn linedef_door_trigger_emits_command() {
    // TODO(C-exec): build a tiny WorldState with a switch/door linedef, run the relevant
    // thinker `update`, and assert the emitted WorldCommand opens/raises the door.
}

/// Thing-def lookup: `DEFAULT_THING_DEFS` returns the documented stats for known kinds
/// and a sane fallback for unknown kinds.
#[test]
fn thing_def_lookup_returns_known_and_fallback() {
    // TODO(C-exec): assert specific health/pain_chance/speed for a known kind (e.g. the
    // Zombieman, 3004) and the fallback for an unknown kind.
    let known = doom::DEFAULT_THING_DEFS.iter().any(|&(k, _)| k == 3004);
    assert!(known);
    assert!(true);
}

/// Golden render integration: WAD parse -> map load -> headless render -> compare against
/// `tests/goldens/*.png`.
///
/// Ignored by default: it needs a `wgpu`/Mesa software adapter that is not present in a
/// headless CI worker, and it depends on C-exec extracting `headless_render_case` out of
/// the `winit` loop (ADR 0002 §Harness Architecture). Run explicitly with
/// `DISPLAY=:1 LIBGL_ALWAYS_SOFTWARE=1 cargo test -- --ignored`.
#[test]
#[ignore = "C-exec: needs headless_render_case extraction + GPU/Mesa adapter (ADR 0002)"]
fn golden_render_matches_baseline() {
    // TODO(C-exec): construct world + software renderer, iterate the shared GOLDEN_CASES,
    // call headless_render_case(..., GoldenMode::Compare), and assert the diff score is
    // below threshold for each case.
}
