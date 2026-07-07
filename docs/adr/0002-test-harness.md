# ADR 0002 — Deterministic, Headless Test Harness

- Status: Proposed (design + scaffolding only; bodies implemented by the C-exec node)
- Date: 2026-06-30
- Supersedes / relates to: the ad-hoc `--golden-test` path in `src/main.rs`

## Context & Problem

`aetheris_doom` currently has effectively **zero automated tests**. `cargo test`
compiles the crate but exercises no behavior (see `AGENTS.md`: "`cargo test` currently
contains 0 tests"). The only existing regression mechanism is golden-image testing, and
it suffers from three structural problems:

1. **Golden testing is trapped in the `winit` event loop.** The golden logic lives
   inside `run(...)` in `src/main.rs` (flags parsed at lines ~94–95, weapon/ammo setup at
   ~190, and the case driver inside the `Event::RedrawRequested` arm at ~659–712). It
   only runs by launching a real GUI window, which requires `DISPLAY=:1` and
   `LIBGL_ALWAYS_SOFTWARE=1`, plus a working `wgpu`/Mesa software adapter. It cannot be
   invoked from `cargo test`, cannot run in a headless CI worker without a display, and
   mixes simulation setup, rendering, screenshotting, and image comparison in one block.
   It also uses `static mut LAST_GOLDEN` and frame-count polling, which is awkward to
   reason about and impossible to unit test.

2. **Global mutable RNG state.** Doom's `p_random()` reads a module-level
   `PRND_TABLE`/`PRND_INDEX` (`src/doom.rs` ~371). Any test that touches monster pain,
   attacks, or other randomized logic is non-deterministic unless the index is reset
   first. The crate already exposes the hooks needed to fix this: `reset_rng()` (~380)
   and `reset_rng_to(index)` (~387).

3. **No layering between pure simulation and GPU-bound rendering.** Much of the
   interesting game logic (the `STATES`/`DEFAULT_STATES` table, the `MonsterThinker`
   state machine, `DEFAULT_THING_DEFS` lookups, `WorldCommand` emission) is pure data /
   pure functions that need no GPU at all, but there is currently no test entry point
   that runs them in isolation.

This ADR defines how to split tests into a fast, always-on **unit** layer and a slower,
opt-in **integration** layer, how to make both deterministic, and the minimal refactor
needed to lift golden rendering out of the event loop.

## The Split: Unit vs. Integration vs. Manual

### Unit-testable (pure simulation — no GPU, no window, no WAD)

These run on every `cargo test` and must stay fast and deterministic. They are reachable
today through the library crate (`use aetheris_doom::doom::...`), because `src/lib.rs`
re-exports `pub mod doom;` and the relevant items are already `pub`.

- **RNG sequence determinism.** `reset_rng_to(seed)` then a fixed sequence of
  `p_random()` calls must return a fixed, table-anchored sequence; resetting to the same
  seed reproduces it exactly, and the index wraps mod 256.
- **State-machine table integrity & progression.** Walking `STATES` via `next_state`
  from each monster's start state (`get_start_state(kind)`) terminates (terminal frame
  has `duration == -1`) and never indexes out of bounds. `MonsterThinker::set_state`
  semantics (sets `tics` from the state's `duration`, marks `just_entered_state`) and the
  tic-countdown → `next_state` advance are pure given a state index.
- **Thing-def lookups.** `DEFAULT_THING_DEFS` lookups used by `Thing::initial_health` /
  `Thing::pain_chance` return the documented health/pain-chance/speed for known kinds and
  the fallback for unknown kinds.
- **`WorldCommand` application / linedef & door triggers.** The *decision* logic that
  emits `WorldCommand`s (e.g. linedef scanning in the projectile/thinker `update`
  methods) is testable against a small hand-built `WorldState` fixture. NOTE: `WorldState`
  is owned by the external `aetheris` engine, so this category needs a tiny fixture
  builder (a near-empty `WorldState` with a couple of `linedefs`/`sectors`/`things`). If
  the engine does not expose a cheap constructor, C-exec should add a `#[cfg(test)]`
  fixture helper rather than reaching into private engine state. Until that fixture
  exists these tests may be authored `#[ignore]`d.

### Integration-testable (WAD parse → map load → headless render → golden compare)

These live in `tests/` and are **`#[ignore]`d by default** because they need a `wgpu`
adapter (GPU or Mesa software GL) that is not guaranteed in CI. They are run explicitly
with `cargo test -- --ignored` (plus `DISPLAY=:1 LIBGL_ALWAYS_SOFTWARE=1` in this VM).

- WAD parse + `load_map` of `freedoom1.wad` succeeds and yields a non-empty world.
- Driving the player to each fixed golden case, rendering scene + HUD, screenshotting,
  and comparing against `tests/goldens/*.png` via the engine's
  `VisualRegressionEngine::compare_images`.

### Stays manual / GUI

Interactive input (movement, firing, weapon switching), audio, frame pacing / perf, and
anything requiring a human to judge "looks right" remain manual GUI testing as described
in `AGENTS.md`.

## Determinism Strategy

1. **Anchor the RNG.** Every test (unit or integration) that can hit `p_random()` must
   call `reset_rng_to(seed)` (or `reset_rng()` for seed 0) in its first lines. Because
   the RNG is process-global, tests must not assume isolation across the global table;
   each test re-anchors before acting. Where two tests in the same binary could
   interleave, prefer `reset_rng_to` with a per-test seed and assert only on the
   subsequence that test produced.
2. **Fixed tick counts.** Simulation tests advance the world a fixed number of ticks
   (e.g. "tick the thinker N times, assert state == X"), never wall-clock durations.
3. **No wall-clock / threading nondeterminism.** Tests must not sleep, read
   `Instant::now()`, or depend on the event-loop accumulator. The headless render entry
   point (below) takes an explicit case spec instead of polling `frame_count`.
4. **Fixed player positions = reuse the golden cases.** The four golden cases already
   encode deterministic `(frame, name, position, angle, weapon)` tuples in `main.rs`
   (~661–664). These move into a shared `const` table so both the `--golden-test` CLI
   path and the integration test consume the *same* fixtures.

## Harness Architecture

The core refactor (C-exec) is to **extract the golden-drive logic out of the `winit`
loop** into a single headless-callable function, shared by the CLI flag and the
integration test:

```rust
// proposed, in src/lib.rs / a new src/harness.rs (engine-facing, GPU-capable)
pub struct GoldenCase {
    pub name: &'static str,
    pub position: glam::Vec2,
    pub angle: f32,
    pub weapon: aetheris::simulation::WeaponType,
}

pub const GOLDEN_CASES: &[GoldenCase] = /* the four cases lifted from main.rs */;

/// Place the player per `case`, render scene + HUD with `renderer`, write a screenshot,
/// and (optionally) compare against the committed golden. Returns the diff score.
pub fn headless_render_case(
    world: &mut aetheris::simulation::WorldState,
    renderer: &mut dyn aetheris::presentation::VisualBridge,
    case: &GoldenCase,
    mode: GoldenMode, // Compare | Update
) -> anyhow::Result<f32> { /* C-exec */ }
```

- `src/main.rs`'s `--golden-test` / `--update-goldens` arms become thin callers that
  iterate `GOLDEN_CASES` and call `headless_render_case(...)`; **no behavior change** to
  the game, just relocation of logic.
- The integration test constructs the same `world` + a software renderer and calls
  `headless_render_case(...)`. Because renderer construction (`ClassicSoftwareEngine`)
  still needs a window/`wgpu` adapter, the integration test is `#[ignore]`d so that a
  GPU-less `cargo test` stays green. CI gating options (pick one in C-exec):
  - `#[ignore]` + run explicitly with `--ignored` on a display-capable runner (default,
    chosen here — simplest, no new deps), or
  - a Cargo feature (e.g. `gpu_tests`) gating `#[cfg_attr(not(feature = "gpu_tests"), ignore)]`, or
  - an env guard (`if std::env::var("DISPLAY").is_err() { return; }`) for a soft skip.

This keeps **unit tests always-on** and **integration/golden tests opt-in**, exactly the
property CI needs (`cargo test` green without a display; goldens runnable on demand).

## Fixtures

- **Golden PNGs** live in `tests/goldens/<name>.png` (already the path used by
  `main.rs`). They are *not committed yet* (per `AGENTS.md`), so comparison is skipped
  until seeded. Seed them by running the existing
  `./target/release/aetheris_doom --update-goldens` (with `DISPLAY=:1
  LIBGL_ALWAYS_SOFTWARE=1`) which copies `temp_<name>.png` → `tests/goldens/<name>.png`.
- **WADs.** `freedoom1.wad` is committed at the repo root and loaded by default. Large
  binary fixtures (alternate WADs, baseline PNG sets) should **not** bloat git history;
  follow the same download-on-demand approach the engine uses for assets (fetch into a
  gitignored location in a setup step) rather than committing multi-MB blobs. Commit only
  the small golden PNGs needed for regression, and keep them lean (the four current
  cases).
- Transient artifacts (`temp_*.png`, `diff_*.png`) are build outputs and should be
  gitignored, not committed.

## Test Layout

- **Unit tests:** `#[cfg(test)] mod tests { ... }` co-located at the bottom of the module
  under test — primarily `src/doom.rs` (RNG, state table, thing-defs, `MonsterThinker`).
  Co-location keeps them next to the code and lets them reach private helpers if needed.
- **Integration tests:** one file per concern in `tests/`:
  - `tests/simulation.rs` — pure-simulation integration coverage reachable via the public
    `aetheris_doom::doom` API (RNG determinism, monster state progression, linedef/door
    trigger, thing-def lookup). This is the file scaffolded by this ADR's node.
  - `tests/golden_render.rs` (future, C-exec) — the `#[ignore]`d WAD→render→compare flow,
    or the golden stub may live in `tests/simulation.rs` initially (it does in the
    scaffold).

### Reachability / visibility

No `lib.rs` change was required for the scaffold: `src/lib.rs` already exposes
`pub mod doom;`, and the items the stubs reference (`reset_rng`, `reset_rng_to`,
`p_random`, `STATES`, `MobjState`, `MonsterThinker`, `DEFAULT_THING_DEFS`,
`get_start_state`) are already `pub`. The dead-code warnings for some of these (e.g.
`reset_rng_to`) come from the **bin** crate (`main.rs` has its own `mod doom;`) and are
pre-existing/non-blocking; they do not appear in the **lib** crate that tests link
against.

The one piece that is **not** yet reachable is the golden render path: it lives only in
`src/main.rs` (the bin). C-exec must perform the extraction described under "Harness
Architecture" (move `GOLDEN_CASES` + `headless_render_case` into the lib, e.g.
`src/harness.rs` re-exported from `lib.rs`) before the golden integration test can be
un-`#[ignore]`d and call shared code instead of duplicating it.
