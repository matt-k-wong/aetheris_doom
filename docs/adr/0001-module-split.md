# ADR 0001 — Decompose `src/doom.rs` into a `doom/` module tree

- **Status:** Proposed (design only — no behavior change)
- **Date:** 2026-06-29
- **Scope:** `src/doom.rs` (6,416 lines)
- **Related work:**
  - **B-design** (this node): produce this ADR + compiling skeleton modules.
  - **B-exec** (separate node): perform the behavior-preserving code move.

---

## Context

`src/doom.rs` is a single 6,416-line file that holds essentially the entire
game-logic layer that sits on top of the external `aetheris` engine. In one file
it currently defines, in source order:

| Lines (approx) | Contents |
| --- | --- |
| 1–48 | `MonsterAction` enum, `MobjState` struct, `ThingDef` struct |
| 50–296 | `PuffThinker` + `ProjectileThinker` structs and their `Thinker` impls, `PROJECTILE_RADIUS` |
| 298–344 | Thing-type id constants: `MONSTER_*`, `ITEM_*`, `KEY_*`, `EFFECT_*` |
| 346–409 | `static mut PRND_INDEX`, `PRND_TABLE`, `p_random()`, `reset_rng()`, `reset_rng_to()` (+ a commented-out `pain_chance_for_kind`) |
| 412–472 | `DoomThingExt` trait + `impl … for Thing` (`is_monster`, `is_flying`, `is_pickup`, `is_barrel`, `is_effect`, `initial_health`, `pain_chance`, `sprite_name`, `frame_char`) |
| 477–484 | `MonsterThinker` struct |
| 486–597 | `S_*` state-index constants (per-monster STND/RUN/ATK/PAIN/DIE) |
| 599–3177 | `STATES` alias + the `DEFAULT_STATES` state table |
| 3178–3200 | `get_start_state(kind)` |
| 3201–3597 | `impl MonsterThinker` (`new`, `is_in_death_sequence`, `set_state`, `execute_action`, `try_move`) |
| 3599–3788 | `impl Thinker for MonsterThinker` (`on_pain`, `on_wake`, `update`) |
| 3791–3824 | `DoomWorldExt` trait |
| 3826–6172 | `impl DoomWorldExt for WorldState` — `is_walk_trigger`, `spawn_effect_thing`, `fire_hitscan`, `spread_noise`, `update`, `apply_commands`, `activate_linedef_manual`, `activate_linedef`, `find_lowest_adjacent_ceiling`, `trigger_door`, `do_door_tagged`, `do_lift_tagged`, `do_crusher_tagged`, `do_stairs_tagged`, `update_environmental_damage` |
| 6174–6414 | `DEFAULT_THING_DEFS` actor table |
| 6416 | `init_world()` (no-op) |

Problems caused by the single-file layout:

- **Cross-cutting state is invisible.** The `static mut PRND_INDEX` RNG, the
  `STATES` table, and `DEFAULT_THING_DEFS` are referenced from nearly every other
  region of the file. A reader cannot tell what touches global RNG state without
  scanning all 6.4k lines.
- **Testability.** Pure, table-driven logic (RNG, state lookups, def lookups,
  thing classification) is interleaved with huge stateful methods
  (`update`, `apply_commands`, `fire_hitscan`). There is no seam to unit-test the
  pure pieces. `cargo test` currently contains 0 tests.
- **Review friction.** `DoomWorldExt::update` (~720 lines) and
  `DoomWorldExt::apply_commands` (~740 lines) are single functions. Any diff to AI,
  weapons, or linedef logic forces a reviewer to page through unrelated code, and
  merge conflicts are likely whenever two changes touch the file.
- **Discoverability.** New contributors cannot map "monster AI" or "doors" to a
  file; everything is `doom.rs`.

## Decision

Replace the single file with a `src/doom/` module tree. Because Rust forbids
having both `src/doom.rs` and `src/doom/mod.rs`, the **B-exec** node will rename
`src/doom.rs` → `src/doom/mod.rs` (or delete it after moving its items) as part of
the move. The public path `crate::doom::…` is preserved, so `main.rs`,
`bridge.rs`, and `lib.rs` need no import changes.

> This **B-design** node does **not** create `src/doom/` (that would collide with
> the live `src/doom.rs`). Instead it ships a compiling, non-authoritative
> skeleton under **`src/doom_split/`** (wired via `mod doom_split;` in `lib.rs`)
> that mirrors the target tree's signatures. `src/doom_split/` is a scaffold for
> review only; B-exec migrates the real code into `src/doom/` and removes the
> scaffold.

### Target module tree

```
src/doom/
├── mod.rs        // module wiring + `pub use` re-exports that preserve crate::doom::* API
├── rng.rs        // Doom P_Random
├── defs.rs       // thing-type ids, ThingDef, DEFAULT_THING_DEFS, DoomThingExt
├── states.rs     // MonsterAction, MobjState, S_* consts, STATES/DEFAULT_STATES, get_start_state
├── ai.rs         // MonsterThinker + action dispatch
├── combat.rs     // PuffThinker, ProjectileThinker, hitscan/splash/weapon firing
├── world.rs      // DoomWorldExt: update loop, apply_commands, spawn/noise helpers
└── linedefs.rs   // door/lift/crusher/stairs/sector + environmental damage
```

### Module responsibilities, contents, and intended API

#### `doom::rng`
- **Responsibility:** Doom's deterministic lookup-table RNG. Sole owner of the
  global RNG cursor.
- **Moves here:** `static mut PRND_INDEX`, `const PRND_TABLE: [u8; 256]`,
  `p_random()`, `reset_rng()`, `reset_rng_to()`.
- **Public API:** `pub fn p_random() -> u8`, `pub fn reset_rng()`,
  `pub fn reset_rng_to(index: usize)`. `PRND_INDEX`/`PRND_TABLE` become
  module-private (`PRND_INDEX` stays `static mut`, accessed only inside this
  module — see Risks).

#### `doom::defs`
- **Responsibility:** Static actor data and kind-based classification.
- **Moves here:** `ThingDef`; `DEFAULT_THING_DEFS`; the id constants
  `MONSTER_*`, `ITEM_*`, `KEY_*`, `EFFECT_*`; the `DoomThingExt` trait and its
  `impl … for Thing`.
- **Public API:** `pub struct ThingDef`, `pub const DEFAULT_THING_DEFS`,
  the id consts, and `pub trait DoomThingExt` (consumed by `main.rs`/`bridge.rs`).
  Note `DoomThingExt::initial_health`/`pain_chance` already read
  `DEFAULT_THING_DEFS`, so the table and the trait belong together.

#### `doom::states`
- **Responsibility:** The actor state machine: the frame/action/next-state table
  and the symbolic indices into it.
- **Moves here:** `MonsterAction`, `MobjState`, all `S_*` constants, `STATES`
  (alias of `DEFAULT_STATES`), `DEFAULT_STATES`, `get_start_state()`.
- **Public API:** `pub enum MonsterAction`, `pub struct MobjState`,
  `pub const STATES`, `pub const DEFAULT_STATES`, `pub fn get_start_state`, and the
  `S_*` consts (kept `pub` for cross-module AI use; many are only read by `ai`).
  `STATES` is consumed by `main.rs` and `bridge.rs`, so it stays `pub`.

#### `doom::ai`
- **Responsibility:** Monster behavior — the `MonsterThinker` and its per-action
  dispatch (look/chase/attack/pain/death/etc.).
- **Moves here:** `MonsterThinker` struct; `impl MonsterThinker`
  (`new`, `is_in_death_sequence`, `set_state`, `execute_action`, `try_move`);
  `impl Thinker for MonsterThinker`.
- **Public API:** `pub struct MonsterThinker` + `pub fn MonsterThinker::new(...)`
  (consumed by `main.rs`). The helper methods stay private. `execute_action`
  emits combat via `WorldCommand`s (e.g. `FireHitscan`, `SpawnProjectile`,
  `SplashDamage`), so it depends on `states`, `defs`, and `rng` but **not** on the
  `combat` module directly — the coupling is via commands.

#### `doom::combat`
- **Responsibility:** Damage delivery and projectile/puff lifetime —
  hitscan tracing, splash damage, BFG tracers, and the player weapon-firing
  branch currently inlined in `update`.
- **Moves here:** `PuffThinker`, `ProjectileThinker`, `PROJECTILE_RADIUS`, their
  `Thinker` impls, and `DoomWorldExt::fire_hitscan`. The large per-weapon `match`
  inside `update` (`Pistol`/`Shotgun`/`Chaingun`/`RocketLauncher`/`PlasmaRifle`/
  `BFG9000`/`Chainsaw`/`Fist`) should be extracted into a `fire_weapon(...)`
  helper here.
- **Public API:** `pub struct PuffThinker`, `pub struct ProjectileThinker`
  (their fields are read by `world`/`apply_commands` when spawning), plus
  crate-internal `fire_hitscan`/`fire_weapon` entry points. `PROJECTILE_RADIUS`
  becomes module-private.

#### `doom::world`
- **Responsibility:** The per-tick simulation driver and the `WorldCommand`
  interpreter — the heart of the game loop.
- **Moves here:** `DoomWorldExt` trait; `impl DoomWorldExt for WorldState`'s
  `update`, `apply_commands`, `spawn_effect_thing`, `spread_noise`; `init_world`.
- **Public API:** `pub trait DoomWorldExt` with `update` and `apply_commands`
  (both consumed by `main.rs`). The remaining trait methods are crate-internal.
  See "Trait-splitting" below for how the trait's methods are physically grouped.

#### `doom::linedefs`
- **Responsibility:** Map-geometry interaction — line specials, doors, lifts,
  crushers, stairs, and sector-based environmental damage.
- **Moves here:** `is_walk_trigger`, `activate_linedef_manual`,
  `activate_linedef`, `find_lowest_adjacent_ceiling`, `trigger_door`,
  `do_door_tagged`, `do_lift_tagged`, `do_crusher_tagged`, `do_stairs_tagged`,
  `update_environmental_damage`.
- **Public API:** crate-internal — none of these are referenced outside `doom`
  (verified by grep). They are called from `update`/`apply_commands` in `world`.

### Trait-splitting note (`DoomWorldExt` / `DoomThingExt`)

`DoomWorldExt` and `DoomThingExt` are traits implemented on **external** types
(`aetheris::simulation::WorldState` / `Thing`), so all of one trait's methods
must be implemented in a single `impl` block in one module — Rust does not allow
splitting a single trait `impl` across files. Two viable strategies for B-exec:

1. **Keep the trait whole in `world`,** and have its methods delegate to free
   functions that live in the topical modules (e.g. `combat::fire_hitscan(self, …)`,
   `linedefs::activate_linedef(self, …)`). This keeps `crate::doom::DoomWorldExt`
   stable while moving the bulk of the logic out of `world.rs`.
2. **Split into multiple traits** (e.g. `DoomWorldExt` for the loop,
   `DoomLinedefExt` for geometry, `DoomCombatExt` for firing), each impl'd in its
   own module.

**Recommended:** Strategy 1 — it preserves the exact public surface
(`crate::doom::DoomWorldExt`) that `main.rs` imports, minimizing risk. The
skeleton encodes this by defining `DoomWorldExt` in `world` and free-function
signatures in `combat`/`linedefs`.

## Public API boundaries

Grepping the rest of the crate (`src/main.rs`, `src/bridge.rs`, `src/lib.rs`) for
`doom::`, `DoomThingExt`, and `DoomWorldExt` shows the **only** externally
consumed items are:

| Item | Consumed by | Target module |
| --- | --- | --- |
| `DoomThingExt` (trait; `is_monster`, `is_barrel`) | `main.rs`, `bridge.rs` | `defs` |
| `DoomWorldExt` (trait; `update`, `apply_commands`) | `main.rs` | `world` |
| `STATES` | `main.rs`, `bridge.rs` | `states` |
| `get_start_state` | `main.rs` | `states` |
| `MonsterThinker` (+ `::new`) | `main.rs` | `ai` |

`doom/mod.rs` will `pub use` these so the `crate::doom::*` paths stay identical:

```rust
pub use ai::MonsterThinker;
pub use defs::DoomThingExt;
pub use states::{STATES, get_start_state};
pub use world::DoomWorldExt;
```

Everything else (id constants, `ThingDef`, `DEFAULT_THING_DEFS`, `MobjState`,
`MonsterAction`, `S_*`, `p_random`/`reset_rng*`, `PuffThinker`,
`ProjectileThinker`, `fire_hitscan`, all linedef/door helpers, `init_world`) is
**not** referenced outside `doom` today and can become `pub(crate)` (or stay
`pub` within `doom` and be re-exported only where convenient). Demoting them is
optional and out of scope for the move; the safe default for B-exec is to keep
their current visibility and only guarantee the five items above remain reachable
at `crate::doom::*`.

## Migration plan

1. **B-design (this node):** land this ADR and the `src/doom_split/` compiling
   skeleton. No behavior change.
2. **B-exec (separate node, behavior-preserving):**
   - Rename `src/doom.rs` → `src/doom/mod.rs`.
   - Move each region into its module per the tables above, adding intra-crate
     `use` lines. Resolve the duplicate `use aetheris::simulation::*;` (it appears
     at both line 1 and line 298 today).
   - Apply the trait-splitting strategy (delegating free functions) so
     `crate::doom::DoomWorldExt` / `DoomThingExt` keep their exact signatures.
   - Add `pub use` re-exports in `mod.rs` to preserve `crate::doom::*`.
   - Delete the `src/doom_split/` scaffold and its `mod doom_split;` line.
   - Verify with `cargo check --all-targets`, `cargo fmt --all -- --check`, and a
     `--golden-test` render run to confirm the WAD→render pipeline is unchanged.
   - The move must be **mechanical**: no logic edits beyond visibility/`use`
     adjustments. Behavior, RNG call order, and `WorldCommand` emission order must
     be byte-for-byte identical.

## Consequences

### Benefits
- Each concern (RNG, defs, states, AI, combat, world loop, linedefs) is in a file
  of a reviewable size; diffs become localized and merge conflicts rarer.
- The pure modules (`rng`, `defs`, `states`) gain a natural seam for the
  currently-absent unit tests.
- The global RNG state is encapsulated in one module, making its single-threaded
  contract explicit and auditable.

### Trade-offs / costs
- One large mechanical diff during B-exec; risk of accidental behavior change is
  the main hazard and is mitigated by golden-image verification.
- Splitting a trait whose `impl` must stay monolithic forces either delegation
  boilerplate (Strategy 1) or a public-surface change (Strategy 2). We pick
  delegation to keep the surface stable.

### Risks
- **`static mut PRND_INDEX` RNG.** Moving `p_random` must preserve *call order*
  across modules, because the result of every monster action, pain check, and
  weapon spread depends on the shared cursor. Reordering module initialization or
  function calls would silently change gameplay. The skeleton isolates this in
  `rng` so reviewers can confirm no extra/removed `p_random()` calls. (`static mut`
  access also emits the usual `unsafe` and, on recent toolchains,
  `static_mut_refs` lints — unchanged from today.)
- **Cross-module references.** `ai::execute_action`, `combat::fire_hitscan`,
  `defs::DoomThingExt`, and `world`/`linedefs` all read `STATES`,
  `DEFAULT_THING_DEFS`, and `p_random()`. The dependency graph is acyclic
  (`rng`/`defs`/`states` are leaves; `ai`/`combat` depend on them; `world`/
  `linedefs` depend on everything), but care is needed so the `WorldCommand`-based
  decoupling between `ai` and `combat` is not turned into a hard call dependency.
- **Trait coherence.** `DoomWorldExt`/`DoomThingExt` are impl'd on external types;
  their `impl` blocks cannot be physically split, constraining how much code can
  literally leave `world.rs`/`defs.rs` (hence the delegation strategy).
- **`--all-targets` parity.** `main.rs` declares its own `mod doom;` rather than
  using the library crate, so the tree must compile identically under both the bin
  and lib targets after the rename.
