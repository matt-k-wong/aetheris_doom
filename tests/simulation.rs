//! Deterministic, headless simulation tests for `aetheris_doom`.
//!
//! Always-on unit coverage per ADR 0002 (`docs/adr/0002-test-harness.md`). Tests exercise
//! the public `aetheris_doom::doom` API without GPU, WAD, or window dependencies.
//!
//! RNG note: `p_random()` reads a **process-global** `static mut PRND_INDEX` in `src/doom.rs`
//! (not `thread_local`). Each test that depends on sequence position calls `reset_rng()` or
//! `reset_rng_to()` at the start; parallel `cargo test` workers can race on that global if
//! they interleave RNG calls, matching the engine's single-threaded determinism model.

use aetheris::simulation::{DoorState, LineDefinition, Sector, SectorAction, Thing, WorldState};
use aetheris_doom::doom::{
    self, DoomThingExt, DoomWorldExt, MONSTER_CACODEMON, MONSTER_IMP, MONSTER_ZOMBIEMAN, MobjState,
    MonsterAction, S_POSS_ATK, S_POSS_DIE, S_POSS_RUN, S_POSS_STND,
};

const PRND_PREFIX: [u8; 8] = [0, 8, 109, 220, 222, 241, 149, 107];

fn thing_def(kind: u16) -> Option<&'static doom::ThingDef> {
    doom::DEFAULT_THING_DEFS
        .iter()
        .find(|&&(k, _)| k == kind)
        .map(|(_, def)| def)
}

fn walk_next_states(states: &[MobjState], start: usize, max_steps: usize) -> Vec<usize> {
    let mut chain = vec![start];
    let mut current = start;
    for _ in 0..max_steps {
        assert!(
            current < states.len(),
            "state index {current} out of bounds (len {})",
            states.len()
        );
        current = states[current].next_state;
        chain.push(current);
    }
    chain
}

fn walk_until_terminal(states: &[MobjState], start: usize, max_steps: usize) -> usize {
    let mut current = start;
    for step in 0..max_steps {
        assert!(
            current < states.len(),
            "state index {current} out of bounds at step {step}"
        );
        if states[current].duration == -1 {
            return current;
        }
        current = states[current].next_state;
    }
    panic!("no terminal state (duration == -1) within {max_steps} steps from {start}");
}

/// RNG sequence determinism: table prefix, wrap-around, and seekable index.
#[test]
fn rng_sequence_is_deterministic() {
    doom::reset_rng();
    let prefix: Vec<u8> = (0..PRND_PREFIX.len()).map(|_| doom::p_random()).collect();
    assert_eq!(
        prefix, PRND_PREFIX,
        "first values must match the DOOM PRND_TABLE prefix"
    );

    doom::reset_rng();
    for _ in 0..256 {
        doom::p_random();
    }
    let wrapped = doom::p_random();
    doom::reset_rng();
    let first_again = doom::p_random();
    assert_eq!(
        wrapped, first_again,
        "after 256 draws the index must wrap and return the table start"
    );

    doom::reset_rng_to(5);
    assert_eq!(
        doom::p_random(),
        241,
        "reset_rng_to(5) must seek to PRND_TABLE[5]"
    );

    doom::reset_rng_to(0);
    let seq_a: Vec<u8> = (0..PRND_PREFIX.len()).map(|_| doom::p_random()).collect();
    doom::reset_rng_to(0);
    let seq_b: Vec<u8> = (0..PRND_PREFIX.len()).map(|_| doom::p_random()).collect();
    assert_eq!(
        seq_a, seq_b,
        "re-anchoring to the same index reproduces the subsequence"
    );
}

/// Zombieman (POSS) state table: stand/look, run-cycle loop, and attack chain.
#[test]
fn monster_state_progression_terminates() {
    let states = doom::STATES;

    assert_eq!(doom::get_start_state(MONSTER_ZOMBIEMAN), S_POSS_STND);

    let stand = &states[S_POSS_STND];
    assert_eq!(stand.sprite, "POSS");
    assert_eq!(stand.action, Some(MonsterAction::Look));

    let run_cycle = walk_next_states(states, S_POSS_RUN, 8);
    assert_eq!(
        run_cycle.last().copied(),
        Some(S_POSS_RUN),
        "POSS run frames must loop back to S_POSS_RUN"
    );
    assert!(
        run_cycle
            .windows(2)
            .all(|w| w[0] != w[1] || w[0] == S_POSS_RUN),
        "run cycle should advance through distinct frames before returning to S_POSS_RUN"
    );

    let atk_chain = walk_next_states(states, S_POSS_ATK, 4);
    let has_pos_attack = atk_chain
        .iter()
        .any(|&idx| states[idx].action == Some(MonsterAction::PosAttack));
    assert!(
        has_pos_attack,
        "S_POSS_ATK chain must include a PosAttack frame (indices: {atk_chain:?})"
    );

    let terminal = walk_until_terminal(states, S_POSS_DIE, 32);
    assert_eq!(
        states[terminal].duration, -1,
        "POSS death chain must reach a terminal corpse frame"
    );
    assert_eq!(
        doom::get_start_state(MONSTER_IMP),
        doom::S_TROO_STND,
        "imp kind maps to S_TROO_STND"
    );
}

/// `DEFAULT_THING_DEFS` and `DoomThingExt` helpers return known mobjinfo and fallbacks.
#[test]
fn thing_def_lookup_returns_known_and_fallback() {
    let imp = thing_def(MONSTER_IMP).expect("imp (3001) must be in DEFAULT_THING_DEFS");
    assert_eq!(imp.health, 60.0);
    assert_eq!(imp.radius, 20.0);

    let caco = thing_def(MONSTER_CACODEMON).expect("cacodemon (3005) must be present");
    assert_eq!(caco.health, 400.0);
    assert_eq!(caco.radius, 31.0);
    assert_eq!(caco.pain_chance, 128);

    let zombieman = thing_def(MONSTER_ZOMBIEMAN).expect("zombieman (3004) must be present");
    assert_eq!(zombieman.health, 20.0);
    assert_eq!(zombieman.pain_chance, 200);

    let world = WorldState::new();
    assert_eq!(Thing::initial_health(MONSTER_IMP, &world), 60.0);
    assert_eq!(Thing::pain_chance(MONSTER_CACODEMON, &world), 128);
    assert_eq!(Thing::initial_health(42_424, &world), 100.0);
    assert_eq!(Thing::pain_chance(42_424, &world), 0);
}

fn closed_door_sector() -> Sector {
    Sector {
        floor_height: 0.0,
        ceiling_height: 0.0,
        light_level: 255.0,
        texture_floor: "FLOOR4_8".into(),
        texture_ceiling: "CEIL3_5".into(),
        tag: 0,
        action: SectorAction::None,
        special_type: 0,
        secret_found: false,
    }
}

fn open_room_sector() -> Sector {
    Sector {
        floor_height: 0.0,
        ceiling_height: 128.0,
        light_level: 192.0,
        texture_floor: "FLOOR4_8".into(),
        texture_ceiling: "CEIL3_5".into(),
        tag: 0,
        action: SectorAction::None,
        special_type: 0,
        secret_found: false,
    }
}

/// DR open-door (special 1, tag 0) on a hand-built `WorldState` starts opening the back sector.
#[test]
fn linedef_door_trigger_emits_command() {
    const DOOR_SECTOR: usize = 1;

    let mut world = WorldState::new();
    world.vertices = vec![glam::Vec2::new(0.0, 0.0), glam::Vec2::new(64.0, 0.0)];
    world.sectors = vec![open_room_sector(), closed_door_sector()];
    world.linedefs.push(LineDefinition {
        start_idx: 0,
        end_idx: 1,
        front: None,
        back: None,
        sector_front: Some(0),
        sector_back: Some(DOOR_SECTOR),
        special_type: 1,
        sector_tag: 0,
        flags: 0,
        activated: false,
    });

    let mut cmds = Vec::new();
    world.activate_linedef_manual(0, Some(DOOR_SECTOR), &mut cmds);

    match &world.sectors[DOOR_SECTOR].action {
        SectorAction::Door {
            state: DoorState::Opening,
            speed,
            wait_timer,
            open_height,
            close_height,
            ..
        } => {
            assert_eq!(*speed, 4.0);
            assert_eq!(*wait_timer, 4.0);
            assert_eq!(*close_height, 0.0);
            assert!(
                *open_height >= 84.0,
                "closed door must target at least floor+84 (safe headroom), got {open_height}"
            );
        }
        other => panic!("expected Door::Opening on back sector, got {other:?}"),
    }
    assert!(world.linedefs[0].activated);
}

/// Save/load round-trip: not exposed through `aetheris_doom` yet (menu stubs in `main.rs` only).
#[test]
#[ignore = "save/load not implemented in public API (README roadmap); engine IO lives in main.rs"]
fn save_load_round_trip_preserves_world() {
    // When `aetheris_doom` exposes a headless save/load entry point, construct a minimal
    // WorldState, persist it, reload, and assert sector/thing/player fields match.
}

/// Golden render integration: WAD parse -> map load -> headless render -> compare against
/// `tests/goldens/*.png`.
///
/// Ignored: needs `headless_render_case` extracted from the `winit` loop (ADR 0002) plus a
/// `wgpu`/Mesa software adapter. Run with
/// `DISPLAY=:1 LIBGL_ALWAYS_SOFTWARE=1 cargo test -- --ignored`.
#[test]
#[ignore = "needs headless_render_case extraction + GPU/Mesa adapter (ADR 0002)"]
fn golden_render_matches_baseline() {
    // Iterate shared GOLDEN_CASES through headless_render_case(..., GoldenMode::Compare).
}
