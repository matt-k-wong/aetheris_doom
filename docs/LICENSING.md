# Licensing Decision: `aetheris_doom` is a GPLv2 Derivative Work

**Status:** Decision — action required
**Scope of this document:** Records the findings of a source audit of `src/doom.rs`,
establishes the correct licensing posture for this repository, and lists every
README claim that must change. It does **not** rewrite the README, change the
project license file, or modify code; those are follow-up actions tracked at the
end of this document.

---

## 1. Summary / TL;DR

`src/doom.rs` is **not** a clean-room implementation. It contains content that is
copied from, or directly transcribed from, id Software's DOOM C source code:
the `statenum_t` identifiers from `info.h`, the `states[]` state table from
`info.c`, the `mobjinfo[]` actor stats from `info.c`, the action-function names
from `p_enemy.c`, and the `rndtable` from `m_random.c`.

id's DOOM source is licensed under the **GNU General Public License, version 2
(GPLv2)** — it is **not** public domain and is **not** MIT-compatible. Because
this project incorporates and is a derivative work of that GPLv2 code, **the
entire combined program must be distributed under GPLv2** (or a GPLv2-compatible
copyleft license). The current README's claims of "clean-room," "MIT," "no
original copyrighted code," and "RNG table is the only exception" are factually
incorrect and create real legal exposure for downstream users.

**Decision:** Relicense the project under **GPLv2** (matching id's grant), add the
required notices, and correct the README. This document is the record of that
decision; the README rewrite and `LICENSE` file change are separate follow-up
tasks.

---

## 2. What was audited

- File: `src/doom.rs` (~6,400 lines), plus spot checks of `src/main.rs`,
  `src/bridge.rs`.
- Reference: id Software's DOOM source release (`info.h`, `info.c`, `p_enemy.c`,
  `m_random.c`).

The `aetheris` engine is an external git dependency (separate repository) and is
out of scope for this audit, except that its "clean-room" branding is referenced
by README claims (see §5).

---

## 3. Evidence: `src/doom.rs` derives from id's GPLv2 source

Each item below is independently sufficient to establish a derivative-work
relationship; together they are conclusive. Verbatim reuse of identifiers,
data tables, and numeric constants is the opposite of an independent clean-room
reimplementation.

### 3.1 `statenum_t` constants from `info.h` (verbatim identifiers)

`src/doom.rs` defines a large set of state constants whose names are the exact
`statenum_t` enumerators from id's `info.h`, including `S_NULL`, `S_POSS_STND`,
`S_POSS_RUN`, `S_POSS_ATK`, `S_POSS_PAIN`, `S_POSS_DIE`, and the analogous
`S_TROO_*`, `S_SPOS_*`, `S_SARG_*`, `S_HEAD_*`, `S_BOSS_*`, `S_SKULL_*`,
`S_CPOS_*`, `S_SKEL_*`, `S_FATT_*`, `S_BSPI_*`, `S_BOS2_*`, `S_PAIN_*`,
`S_VILE_*`, `S_SPID_*`, `S_CYBR_*`, `S_SSWV_*`, plus `S_BAR1` / `S_BEXP`
(barrel). See `src/doom.rs` lines ~486–597.

These naming conventions (`POSS` = "possessed"/zombieman, `TROO` = imp/trooper,
`SARG` = demon/sergeant, `BOS2` = Hell Knight, etc.) are id's internal sprite
mnemonics. Independently arriving at the identical identifier set is not
plausible.

### 3.2 `states[]` table from `info.c` (transcribed data table)

`DEFAULT_STATES` (`src/doom.rs` line ~601 onward) is a direct transcription of
id's `state_t states[NUMSTATES]` array. Each entry's fields map one-to-one:

| id `info.c` field | `src/doom.rs` `MobjState` field |
| --- | --- |
| `sprite` (`SPR_POSS`) | `sprite: "POSS"` |
| `frame` (`0`, `1`, …) | `frame: 'A'`, `'B'`, … |
| `tics` | `duration` |
| `action` (`{A_Look}`, `{A_Chase}`, …) | `action: Some(MonsterAction::Look)` … |
| `nextstate` | `next_state` |

The sprite mnemonics, the integer-frame → letter mapping (`0`→`A`, `1`→`B`, …),
the per-state tic durations, the action assignments, and the next-state wiring
all match the original table. The inline comments even preserve the original
state indices and names (`/* 1 S_POSS_STND */`, etc.).

### 3.3 `mobjinfo[]` actor stats from `info.c` (copied numeric constants)

`DEFAULT_THING_DEFS` (`src/doom.rs` line ~6174 onward) reproduces id's
`mobjinfo_t mobjinfo[]` values. Examples that match id's table exactly:

| Actor | `spawnhealth` | `radius` | `height` | `mass` | `painchance` |
| --- | --- | --- | --- | --- | --- |
| Zombieman (3004) | 20 | 20 | 56 | 100 | 200 |
| Imp (3001) | 60 | 20 | 56 | 100 | 200 |
| Demon (3002) | 150 | 30 | 56 | 400 | 180 |
| Baron (3003) | 1000 | 24 | 64 | 1000 | 50 |
| Cacodemon (3005) | 400 | 31 | 56 | 400 | 128 |
| Lost Soul (3006) | 100 | 16 | 56 | 50 | 255 |
| Spider Mastermind (7) | 3000 | 128 | 100 | 1000 | 40 |
| Cyberdemon (16) | 4000 | 40 | 110 | 1000 | 20 |
| Arch-Vile (64) | 700 | 20 | 56 | 500 | 10 |

(id stores `radius`/`height` as `16.16` fixed-point; the integer map-unit values
here are the same numbers with the fixed-point shift removed.) These are design
constants authored by id, not values derivable from "black-box observation" of an
already-correct magnitude.

### 3.4 `p_enemy.c` action functions → `MonsterAction` enum

The `MonsterAction` enum (`src/doom.rs` lines ~6–27) mirrors id's `p_enemy.c`
action-function set: `A_Look`, `A_Chase`, `A_FaceTarget`, `A_PosAttack`,
`A_SPosAttack`, `A_TroopAttack`, `A_SargAttack`, `A_HeadAttack`, `A_BruisAttack`,
`A_SkelMissile`, `A_FatAttack`, `A_VileChase`, `A_VileAttack`, `A_PainAttack`,
`A_Pain`, `A_Scream`, `A_Fall`, `A_Explode` (`A_BarrelDestroy`/`A_Explode`),
`A_VileChase` resurrection (`Raise`), and `A_SkullAttack`.

Beyond the names, the behavior reproduces original formulas. For example, the
demon melee in `MonsterAction::SargAttack` (`src/doom.rs` line ~3476) computes
`((p_random() % 10) + 1) * 4`, which is exactly id's `A_SargAttack`
`damage = ((P_Random()%10)+1)*4;`.

### 3.5 `rndtable` from `m_random.c`

`PRND_TABLE` (`src/doom.rs` lines ~350–368) is id's exact 256-byte `rndtable`,
and `p_random()` (line ~371) reproduces id's index-advancing `P_Random()`
algorithm. The README already concedes this one — but incorrectly frames it as
the *only* borrowed element.

---

## 4. Why the current posture is wrong

### 4.1 The source is GPLv2, not public domain and not MIT-compatible

- id Software released the DOOM source on **December 23, 1997** under the
  non-commercial, education-only **DOOM Source License (DSL)** — which by itself
  *forbade* commercial use entirely.
- John Carmack re-licensed the DOOM source under the **GNU GPL version 2** on
  **October 3, 1999**. GPLv2 was formally applied to id's public GitHub
  repository on **January 16, 2024**.
- The relevant grant available today is therefore **GPLv2**. There is no
  public-domain dedication, and at no point was the code MIT-licensed.

Under GPLv2 §2(b), a work that contains or is derived from GPLv2 code, when
distributed, "must be licensed as a whole at no charge to all third parties under
the terms of this License." MIT is **not** a valid choice for this combined work:
you cannot strip the copyleft, and MIT's "do anything, including closed-source
commercial use" grant directly contradicts GPLv2's obligations (source
availability, identical-license redistribution, no additional restrictions).

### 4.2 "Clean-room" / "convergent design" does not apply

A genuine clean-room reimplementation reproduces *behavior* without access to the
protected expression, and would not reproduce id's *identifiers, data-table
layout, and exact numeric constants*. Verbatim reuse of `statenum_t` names, the
`states[]` table contents, and `mobjinfo[]` magic numbers is copying of
expressive material, not independent convergence. The README's "AI generalized
training / convergent functional design" rationale does not change the
copyright analysis: the output is a derivative work regardless of how it was
produced.

### 4.3 The "RNG is a mathematical constant" theory is both wrong and moot

Even setting aside whether a 256-byte authored permutation table is a
"mathematical constant" (a dubious claim — it is an authored lookup table, not a
formula), it is **not the only** borrowed element. The state machine and actor
tables are far more expressive and are copied wholesale. The RNG carve-out
cannot rescue the MIT claim.

### 4.4 "100% memory-safe Rust" is also inaccurate

The README's headline claim of "100% memory-safe Rust" is contradicted by the
code: `src/doom.rs` uses `static mut PRND_INDEX` accessed through `unsafe` blocks
(the RNG state, ~lines 348–390), and `src/main.rs` uses a `static mut`
(`LAST_GOLDEN`, ~line 666) inside `unsafe`. This is a separate accuracy problem
from licensing but is listed because it appears in the same marketing copy that
must be corrected.

---

## 5. Required licensing posture (the decision)

1. **License the project under GPLv2.** Add a top-level `LICENSE` (or
   `COPYING`) file containing the full GNU GPL version 2 text. Set
   `license = "GPL-2.0-only"` in `Cargo.toml` (currently no `license` field is
   set).
2. **Add copyright/attribution notices.** Credit id Software for the
   DOOM-derived portions and retain the GPLv2 notice. Add a short header to
   `src/doom.rs` noting that the state tables, actor definitions, action
   functions, and RNG table are derived from id Software's GPLv2 DOOM source.
3. **Document the GPLv2 obligation prominently** in the README: anyone who
   distributes this program (modified or not, with or without the `opl_music`
   feature) must do so under GPLv2 and make complete corresponding source
   available. Closed-source commercial redistribution is **not** permitted.
4. **Stop describing the project as MIT / clean-room / unencumbered.**
5. **Keep the asset posture as-is.** Not shipping copyrighted IWAD assets and
   bundling Freedoom remains correct and unaffected by this decision. (Note that
   Freedoom is distributed under a permissive/BSD-style license, which is fine
   alongside a GPLv2 program.)

> Note: GPLv2 compliance is the minimum required to be lawful. It does **not**
> grant the originally-marketed freedoms (permissive MIT use, closed-source
> commercial products). If those freedoms are a hard product requirement, the
> only alternative is to remove and genuinely clean-room-replace every
> id-derived artifact in §3 — which is a substantial undertaking and is **not**
> what this decision assumes.

---

## 6. README claims that must change

Listed in document order. This is the deliverable inventory; the actual README
rewrite is a separate follow-up task (do **not** edit the README as part of this
decision doc).

| # | Location | Current claim | Why it's wrong | Required change |
| --- | --- | --- | --- | --- |
| 1 | Line 3 | "**100% memory-safe** Rust implementation" | Uses `unsafe` + `static mut` in `doom.rs` and `main.rs` (§4.4). | Drop "100% memory-safe", or qualify (e.g. "safe Rust except for a small `unsafe` RNG state cell"). |
| 2 | Line 8 | Aetheris is "a true 'cleanroom' graphics engine" | This DOOM crate is not clean-room (§3); the branding bleeds into the game's posture. | Remove the implication that the *game logic* is clean-room; if the engine itself is clean-room, scope the claim strictly to the engine. |
| 3 | Line 18 (heading) | "## 🧼 A True Clean-Room Recreation" | False for `doom.rs` (§3). | Remove/replace the entire clean-room section; replace with a GPLv2 derivation + attribution section. |
| 4 | Line 20 | "not a source port. It does not contain or derive from any of the original C source code released by Id Software in 1997." | Directly false: it contains id's `statenum_t`, `states[]`, `mobjinfo[]`, action functions, and `rndtable` (§3). | State plainly that the game logic is derived from id's GPLv2 DOOM source. |
| 5 | Line 22 | "100% ground-up, clean-room recreation written from scratch … through black-box observation" | False (§3, §4.2). | Remove. |
| 6 | Line 24 | "Because no original copyrighted code was used, this framework is legally unencumbered and free to be used as a foundation for your own commercial projects under the **MIT License**." | Wrong on every clause: copyrighted code *was* used; it is *not* unencumbered; *not* MIT; closed-source commercial use is *not* permitted (§4.1). | Replace with the GPLv2 obligation statement (§5.3). |
| 7 | Lines 26–29 | "The RNG Exception … **exactly one intentional exception** … functions as a mathematical constant rather than expressive logic" | The RNG is not the only borrowed element (§3), and the "mathematical constant" theory is dubious and moot (§4.3). | Remove the "only exception" framing; fold the RNG into the general GPLv2 derivation disclosure. |
| 8 | Line 31 | "AI Generation Disclosure … Any structural similarities … are the result of convergent functional design … not intentional copying or derivation of copyrighted material." | Verbatim identifiers and exact data tables are not "convergent design" (§4.2); the output is a derivative work regardless of authorship method. | Remove the disclaimer or rewrite it to acknowledge the derivation; keep an honest AI-assistance note if desired, but it cannot disclaim the copyright status. |
| 9 | Line 39 | "this repository strictly contains game logic and **adheres to copyright law** … does not include copyrighted DOOM game assets" | The *asset* statement is fine, but "adheres to copyright law" is currently false because the code is mislicensed (§4.1). | Keep the asset/Freedoom statement; remove or correct the broader compliance claim until the license is fixed. |
| 10 | Lines 77–78 | OPL3 section: enabling `opl_music` "legally infects your … binary, converting the entire executable into a GPL-licensed product … means you cannot sell a closed-source game compiled with this flag." | Technically correct about Chocolate Doom/`opl-emu`, but **misleading by omission**: it implies the base build is *not* GPL. The core game logic is already GPLv2-derived regardless of this flag. | Reframe: the whole project is GPLv2 in all configurations; `opl_music` adds another GPLv2 component but does not change the base license status. |
| 11 | (Implicit / missing) | No `LICENSE` file; `Cargo.toml` has no `license` field; README never states a concrete license beyond the inline "MIT" claim. | A GPLv2 work must ship its license text and declare it. | Add `LICENSE` (GPLv2 full text), set `license = "GPL-2.0-only"` in `Cargo.toml`, and add a clear "License: GPLv2" section to the README. |

---

## 7. Follow-up actions (not performed by this document)

- [ ] Rewrite `README.md` per the table in §6.
- [ ] Add a top-level `LICENSE`/`COPYING` file with the full GPLv2 text.
- [ ] Set `license = "GPL-2.0-only"` in `Cargo.toml`.
- [ ] Add GPLv2 + id Software attribution headers to `src/doom.rs`.
- [ ] (Optional, only if permissive licensing is a hard requirement) Plan a
      genuine clean-room replacement of every id-derived artifact in §3.

---

*This document is a good-faith engineering and licensing analysis, not formal
legal advice. For distribution decisions with commercial stakes, confirm with
qualified counsel.*
