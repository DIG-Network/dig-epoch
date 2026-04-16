//! # `phase` — L1-progress phase calculation
//!
//! **Introduced by:** `STR-002` — Module hierarchy (SPEC §13).
//!
//! **Owner:** PHS-001 (Phase 5). EpochManager phase methods live in `manager`.
//!
//! **Spec reference:** [`SPEC.md` §4`](../../docs/resources/SPEC.md)

/// Sentinel marker proving the module exists and is reachable at
/// `dig_epoch::phase::STR_002_MODULE_PRESENT`.
#[doc(hidden)]
pub const STR_002_MODULE_PRESENT: () = ();

use crate::constants::{
    EPOCH_L1_BLOCKS, PHASE_BLOCK_PRODUCTION_END_PCT, PHASE_CHECKPOINT_END_PCT,
    PHASE_FINALIZATION_END_PCT,
};
use crate::types::epoch_phase::EpochPhase;

// -----------------------------------------------------------------------------
// PHS-001 — l1_progress_phase_for_network_epoch
// -----------------------------------------------------------------------------

/// Maps L1 block height progress to an `EpochPhase`.
///
/// Progress percentage = `(current - epoch_l1_start) * 100 / EPOCH_L1_BLOCKS`.
/// Phase boundaries are driven by the CON-002 constants (50/75/100%).
/// Pure and deterministic — no side effects, no wall-clock time.
pub fn l1_progress_phase_for_network_epoch(
    genesis_l1_height: u32,
    epoch: u64,
    current_l1_height: u32,
) -> EpochPhase {
    let epoch_l1_start = genesis_l1_height + (epoch as u32 * EPOCH_L1_BLOCKS);
    let pct = if current_l1_height <= epoch_l1_start {
        0u64
    } else {
        let elapsed = (current_l1_height - epoch_l1_start) as u64;
        (elapsed * 100 / EPOCH_L1_BLOCKS as u64).min(100)
    };
    if pct < PHASE_BLOCK_PRODUCTION_END_PCT as u64 {
        EpochPhase::BlockProduction
    } else if pct < PHASE_CHECKPOINT_END_PCT as u64 {
        EpochPhase::Checkpoint
    } else if pct < PHASE_FINALIZATION_END_PCT as u64 {
        EpochPhase::Finalization
    } else {
        EpochPhase::Complete
    }
}
