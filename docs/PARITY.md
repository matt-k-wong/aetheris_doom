# Monster AI — Vanilla DOOM (v1.9) Parity Checklist

Audit of monster AI in `src/doom.rs` (`MonsterThinker::execute_action`, `try_move`,
`set_state`, and `Thinker for MonsterThinker`) against id Software's `p_enemy.c`
semantics from the DOOM v1.9 source release.

References to vanilla code below use the canonical `p_enemy.c` function names
(`A_Look`/`P_LookForPlayers`, `A_Chase`, `A_FaceTarget`, `A_PosAttack`, …). The current
implementation lives in the `MonsterAction` match inside
`MonsterThinker::execute_action` (`src/doom.rs`).

Severity legend:

- **demo-desync**: changes the RNG-call sequence or movement integers; would break
  vanilla demo playback (this port is not bit-exact anyway, but these widen the gap).
- **gameplay-visible**: a player would notice the behavioral difference in normal play.
- **cosmetic**: audio/visual only, no gameplay or determinism impact.

Each section lists: *Vanilla behavior* · *Current implementation* · *Divergence /
severity* · *Recommendation*. A summary of what was fixed in this PR vs. deferred is at
the end.

---

## `A_Look` (`MonsterThinker::execute_action`, `MonsterAction::Look`)

- **Vanilla:** `P_LookForPlayers` only wakes on *sight* when the target is within the
  monster's **forward 180-degree field of view** (`an > ANG90 && an < ANG270` ⇒ behind),
  with a "react anyway if within `MELEERANGE`" exception. A target heard via sound
  propagation (`soundtarget`) wakes the monster regardless of facing. Then `P_CheckSight`
  (LOS) must pass. There is no fixed sight-distance cap (LOS + REJECT do the gating). No
  `P_Random` is consumed by the look decision.
- **Current (before):** Plain distance gate `within_sight_range = target_dist < 3000.0`
  (comment "increased to 3000 for better aggro") OR `within_noise` (noise radius), then a
  LOS check — **no FOV cone**, so monsters effectively had 360-degree sight.
- **Divergence:** gameplay-visible. Monsters woke when the player was directly behind
  them, removing the classic "sneak up from behind" behavior.
- **Recommendation / status:** **FIXED in this PR.** Added the forward 180-degree FOV
  gate using `monster.angle` (spawn/map facing) vs. the bearing to the target, with the
  `MELEERANGE` (64u) close-range exception. The noise branch still bypasses FOV, matching
  vanilla's `soundtarget`. The `3000.0` distance cap is kept (this port has no REJECT
  table, so a cap is the pragmatic LOS-cost limiter); it is now ANDed with the FOV/melee
  test. No RNG is consumed, so determinism is unaffected.

## `A_Chase` (`MonsterAction::Chase`)

- **Vanilla:** moves in one of **8 discrete `movedir` directions** via `P_NewChaseDir` /
  `P_TryMove`/`P_Move`; decrements `reactiontime` and `movecount` (re-picks direction when
  `movecount` expires or move blocked); turns toward `movedir`; calls melee/missile
  attacks only when `P_CheckMeleeRange`/`P_CheckMissileRange` succeed; missile attacks gate
  on a `P_Random`-based refire probability that scales with distance
  (`P_CheckMissileRange`: `if (dist < 1) … P_Random() < prob`). Plays `seesound` randomly
  via `A_Chase`'s `if (P_Random() < 3) S_StartSound(actor, actor->info->activesound)`.
- **Current:** moves along a **free normalized vector** `dir = (target - pos).normalize()`
  toward the target at `def.speed`, with a single 45-degree left/right wall-slide fallback
  in `try_move`. Attack is gated by a per-frame `p_random() < 32` (~12.5%) plus a
  fixed `attack_cooldown` countdown and `target_dist < 2048.0` + LOS. No `movedir`,
  `movecount`, or `reactiontime`; no distance-scaled refire; no active sound.
- **Divergence:** gameplay-visible (movement is smoother/straighter than vanilla's blocky
  8-direction pathing; monsters home directly instead of the vanilla "wander toward"
  pattern) **and** demo-desync (RNG cadence for the attack decision differs from vanilla's
  `P_CheckMissileRange`). Attack cadence is roughly comparable but not vanilla-exact.
- **Recommendation / status:** **DEFERRED — needs design.** A faithful `movedir` +
  `movecount`/`reactiontime` + `P_NewChaseDir` rewrite is a large behavioral change and is
  explicitly out of scope for this audit. Documented here as the single biggest structural
  divergence. Per-frame attack-roll vs. `P_CheckMissileRange` is part of the same rewrite.

## `A_FaceTarget` (`MonsterAction::FaceTarget`)

- **Vanilla:** snaps `actor->angle` to face the target and clears `MF_SHADOW`-based aim
  fuzz; consumes no `P_Random` itself.
- **Current:** issues `ModifyThing { angle: atan2(dir) }`, no movement. Matches the core
  facing behavior.
- **Divergence:** none functionally significant (no shadow/aim-fuzz model in this port).
- **Recommendation:** **document-only.** Correct.

## `A_PosAttack` (`MonsterAction::PosAttack`) — Zombieman (3004)

- **Vanilla:** `A_FaceTarget`; `angle += (P_Random()-P_Random())<<20`;
  `damage = ((P_Random()%5)+1)*3` (3–15); single hitscan `P_LineAttack`. Three
  `P_Random()` calls, in that order. Plays `sfx_pistol`.
- **Current (before):** `spread = (p_random()-128)/256 * 0.1` (one `P_Random`, ~±2.9-deg)
  and **hardcoded `damage: 10.0`**.
- **Divergence:** gameplay-visible (constant 10 vs. 3–15 random; far narrower spread) and
  demo-desync (1 RNG call vs. 3).
- **Recommendation / status:** **FIXED in this PR.** Now consumes three `p_random()` in
  vanilla order: `r1`, `r2` for the spread `(r1-r2) * TAU/4096` (the exact BAM `<<20`
  conversion) and a third for `damage = ((p_random()%5)+1)*3`. RNG-call count now matches
  vanilla, so determinism is *approached*, not degraded.

## `A_SPosAttack` (`MonsterAction::SPosAttack`) — Shotgun Guy / Sergeant (9)

- **Vanilla:** plays `sfx_shotgn`, `A_FaceTarget`, then a loop of **3 pellets**, each:
  `angle = bangle + ((P_Random()-P_Random())<<20)`; `damage = ((P_Random()%5)+1)*3`;
  `P_LineAttack`. Nine `P_Random()` calls total, in order.
- **Current (before):** the `SPosAttack` enum variant existed and was referenced by the
  shotgun-guy attack states (`S_SPOS_ATK`), but `execute_action` had **no match arm** for
  it — it fell through to `_ => {}`, so **shotgun guys fired nothing and dealt no damage.**
- **Divergence:** gameplay-visible (an entire enemy's attack was a no-op) and demo-desync
  (0 RNG calls vs. 9).
- **Recommendation / status:** **FIXED in this PR.** Implemented to match vanilla: 3
  pellets, each consuming `r1`, `r2` (spread) + 1 damage roll `((p_random()%5)+1)*3`, in
  order (nine `p_random()` calls), via the existing `FireHitscan` path used by `PosAttack`.
  Plays `DSSHOTGN`.

## `A_TroopAttack` (`MonsterAction::TroopAttack`) — Imp (3001) [also reused by 3003/3005]

- **Vanilla:** `A_FaceTarget`; if `P_CheckMeleeRange`: `sfx_claw`,
  `damage = (P_Random()%8+1)*3` (3–24), `P_DamageMobj`, return; else
  `P_SpawnMissile(MT_TROOPSHOT)` (Imp fireball; missile damage rolled on impact as
  `(P_Random()%8+1)*info->damage`, `info->damage = 3`). The melee branch uses one
  `P_Random`; the missile branch uses none in the action.
- **Current (before):** melee branch (`target_dist < 72.0`) used **hardcoded `10.0`**;
  missile branch spawns Imp fireball (kind `10031`) with fixed `damage: 10.0`.
- **Divergence:** gameplay-visible (constant 10 vs. 3–24 melee) and demo-desync (melee
  used 0 RNG calls vs. vanilla's 1).
- **Recommendation / status:** **FIXED in this PR (melee only).** Melee now
  `damage = ((p_random()%8)+1)*3`, consuming one `p_random()` like vanilla; missile branch
  left as-is (the projectile damage model is computed at spawn here, not on impact —
  changing it touches the projectile subsystem and is out of scope). See "Shared
  TroopAttack" note below.

### Shared `TroopAttack` note — Cacodemon (3005) & Baron (3003)

- `S_HEAD_ATK` (Cacodemon) and `S_BOSS_ATK` (Baron) states both reference
  `MonsterAction::TroopAttack`, so they reuse the Imp's handler. The dedicated
  `MonsterAction::HeadAttack`, `BruisAttack`, `SkelMissile`, `FatAttack`, `VileChase`,
  `VileAttack`, `PainAttack`, and `Raise` variants are declared but **never assigned to any
  state** (dead variants).
- **Vanilla:** Cacodemon `A_HeadAttack` — melee `(P_Random()%6+1)*10`, else
  `MT_HEADSHOT`; Baron `A_BruisAttack` — melee `(P_Random()%8+1)*10`, else
  `MT_BRUISERSHOT`/BAL7. **Neither has a melee in normal range gating like the Imp** in the
  sense that their projectiles differ; both fire distinct missile types, not the Imp ball.
- **Divergence:** gameplay-visible (wrong projectile sprite/type and wrong melee damage
  for Caco/Baron; they also gain an Imp-style melee they should fire differently).
- **Recommendation:** **DEFERRED — needs design.** Correct handling requires per-monster
  attack handlers (and distinct projectile kinds), plus wiring the dead `HeadAttack` /
  `BruisAttack` variants into their states. Out of scope for low-risk formula fixes. My
  `TroopAttack` melee fix incidentally also affects Caco/Baron melee (now `*3` instead of a
  flat `10`); this is an approximation either way and is noted for the follow-up.

## `A_SargAttack` (`MonsterAction::SargAttack`) — Demon/Spectre (3002)

- **Vanilla:** `A_FaceTarget`; if `P_CheckMeleeRange`: `damage = ((P_Random()%10)+1)*4`
  (4–40), `P_DamageMobj`. **No sound** in `A_SargAttack`. One `P_Random` in melee range,
  none otherwise.
- **Current:** `target_dist < 72.0` ⇒ `damage = ((p_random()%10)+1)*4` — **matches the
  vanilla formula and RNG usage.** Additionally plays `DSBGSITE`, which vanilla does not.
- **Divergence:** cosmetic only — the extra `DSBGSITE` (sarge *sight* sound) on attack.
- **Recommendation:** **document-only.** Damage/RNG already correct; the spurious sound is
  cosmetic and left untouched to keep the change set minimal.

## `A_SkullAttack` (`MonsterAction::SkullAttack`) — Lost Soul (3006) / Pain Elemental frames

- **Vanilla:** `A_SkullAttack` plays `attacksound`, sets `MF_SKULLFLY`, and launches the
  skull at `SKULLSPEED` toward the target; the **damage is dealt on collision** in
  `PIT_CheckThing` as `damage = ((P_Random()%8)+1)*actor->info->damage` (`info->damage=3`,
  so 3–24), after which `MF_SKULLFLY` is cleared.
- **Current:** moves the skull at `speed*4` toward the target each tick; on
  `target_dist < 48.0` deals a **fixed `10.0`** and plays `DSATK`; no `MF_SKULLFLY` state
  machine, no random damage roll.
- **Divergence:** gameplay-visible (constant 10 vs. 3–24) and structural (continuous homing
  vs. ballistic charge that can overshoot and bounce off walls). Damage is not RNG-rolled.
- **Recommendation:** **DEFERRED.** A correct charge needs the `MF_SKULLFLY` flag +
  collision-time damage roll, which is a state/flag change rather than a pure formula tweak.
  Left as-is to avoid touching the movement/collision model.

## `A_Pain` / pain chance (`Thinker::on_pain`)

- **Vanilla:** on damage, `if (P_Random() < info->painchance)` enter the pain state and
  play `painsound`; also (separately) `P_DamageMobj` sets `target` to the attacker.
- **Current:** `on_pain` rolls `if p_random() < pain_chance` using per-thing
  `pain_chance`, enters the matching pain state, and sets `target_thing_idx` to the
  inflictor — **matches vanilla's painchance roll and target-acquire.** Barrels (2035)
  force `S_BEXP` immediately (correct — barrels have no pain state).
- **Divergence:** does not play a `painsound` (cosmetic); otherwise faithful.
- **Recommendation:** **document-only.** Core behavior + RNG correct.

## `A_Scream` (`MonsterAction::Scream`)

- **Vanilla:** plays the monster's `deathsound` (per-type; certain bosses
  `S_StartSound(NULL, …)` at full volume so it is map-wide).
- **Current:** always plays `DSPDIE` (the **player** death sound) positioned at the
  monster, for every monster type.
- **Divergence:** cosmetic (wrong, uniform death sound; no boss full-volume case).
- **Recommendation:** **document-only.** No gameplay/RNG impact; per-type death-sound
  table is a cosmetic follow-up.

## `A_Fall` (`MonsterAction::Fall`)

- **Vanilla:** clears `MF_SOLID` so the corpse stops blocking movement/hitscans.
- **Current:** no-op (comment only). However, `try_move` already skips things with
  `health <= 0.0`, and `fire_hitscan` skips `health <= 0.0`, so dead monsters are
  effectively non-blocking through other code paths.
- **Divergence:** none observable in practice (the solid-clear is achieved implicitly by
  the `health <= 0.0` guards elsewhere).
- **Recommendation:** **document-only.**

## `A_Explode` (`MonsterAction::Explode`) — Barrel / radius damage

- **Vanilla:** `P_RadiusAttack(thing, thing->target, 128)` — 128 max damage, falloff with
  distance, radius 128.
- **Current:** `SplashDamage { damage: 128.0, radius: 128.0 }` with linear falloff to
  player and monsters; plays `DSBAREXP`.
- **Divergence:** minor — vanilla falloff is `dist`-based per blockmap, this is a simple
  linear `(radius-dist)/radius`; close enough.
- **Recommendation:** **document-only.**

---

## `try_move` / movement & collision (supporting)

- **Vanilla:** `P_TryMove` checks blockmap lines + things, step-up `MAXSTEPSIZE` (24),
  drop-off rules, and `opentop-openbottom >= height` clearance.
- **Current:** brute-force loop over all linedefs/things; portal passable when
  `gap >= 56` and `step_up <= STEP_HEIGHT`; "only block if moving geometrically closer"
  heuristic to allow sliding.
- **Divergence:** structural (no blockmap, different slide rule) — part of the deferred
  `A_Chase`/movement rewrite.
- **Recommendation:** **DEFERRED — needs design** (tied to the `movedir` rewrite).

---

## Summary — applied in this PR vs. deferred

**Applied (clear, low-risk correctness fixes; RNG-call structure preserved/aligned to
vanilla):**

1. **`A_Look` FOV gate** restored — front 180-degree cone + `MELEERANGE` exception; noise
   path still bypasses FOV. (no RNG change)
2. **`A_PosAttack`** — vanilla damage `((P_Random()%5)+1)*3` and `(P_Random()-P_Random())`
   BAM spread; 1→3 `p_random()` calls in vanilla order.
3. **`A_SPosAttack`** — implemented (was a silent no-op): 3 pellets, 9 `p_random()` calls
   matching vanilla order/formula.
4. **`A_TroopAttack` melee** — vanilla `(P_Random()%8+1)*3`; melee now consumes 1
   `p_random()` like vanilla.

**Deferred (larger behavior changes / subsystem work — documented, not changed):**

- `A_Chase` 8-direction `movedir` + `movecount`/`reactiontime` + `P_CheckMissileRange`
  refire (biggest structural gap) and the related `try_move`/blockmap movement model.
- Per-monster missile attacks: wire up the dead `HeadAttack`/`BruisAttack`/`SkelMissile`/
  `FatAttack`/`VileAttack`/`PainAttack`/`Raise`/`VileChase` variants and distinct
  projectile kinds; correct Caco/Baron melee (`*10`) and missile types (Caco/Baron
  currently reuse the Imp `TroopAttack` handler + fireball).
- `A_SkullAttack` `MF_SKULLFLY` ballistic charge + collision-time damage roll.
- Missile/projectile damage rolled on impact (Imp/Caco/Baron fireballs) rather than fixed
  at spawn.

**Cosmetic only (no gameplay/RNG impact; left as-is):** `A_Scream` uses `DSPDIE` for all
monsters; `A_SargAttack` plays an extra `DSBGSITE`; pain has no `painsound`.

**Not touched (per constraints):** `PRND_TABLE`, `DEFAULT_STATES`, `DEFAULT_THING_DEFS`.
