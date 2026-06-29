//! Skeleton: Doom's deterministic lookup-table RNG (`P_Random`).
//!
//! Real items to move here from `src/doom.rs`:
//! - `static mut PRND_INDEX` (becomes module-private)
//! - `const PRND_TABLE: [u8; 256]` (becomes module-private)
//! - `p_random`, `reset_rng`, `reset_rng_to`
//!
//! The single global RNG cursor lives here so its single-threaded call-order
//! contract is auditable in one place (see ADR "Risks").

/// Module-private RNG cursor. Stays `static mut`; only this module touches it.
static mut PRND_INDEX: usize = 0;

/// Doom's `P_Random` — returns 0-255 from the lookup table and advances the cursor.
pub fn p_random() -> u8 {
    unimplemented!("moved verbatim from doom.rs during B-exec")
}

/// Reset the RNG to a known state for testing.
pub fn reset_rng() {
    unimplemented!("moved verbatim from doom.rs during B-exec")
}

/// Reset the RNG to a specific index for deterministic testing.
pub fn reset_rng_to(index: usize) {
    unimplemented!("moved verbatim from doom.rs during B-exec")
}
