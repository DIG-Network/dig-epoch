//! # `types::events` — epoch event enum and stats struct
//!
//! **Introduced by:** `STR-002` — Module hierarchy (SPEC §13).
//!
//! **Future owner:**
//! [`TYP-005`](../../../docs/requirements/domains/epoch_types/specs/TYP-005.md)
//! — the `EpochEvent` enum (4 variants: `EpochAdvanced`, `PhaseTransitioned`,
//! `CheckpointCompetitionStarted`, `CheckpointFinalized`) together with
//! the `EpochStats` struct summarising cross-epoch counters.
//!
//! **Spec reference:** [`SPEC.md` §3.6](../../../docs/resources/SPEC.md) —
//! events; [`SPEC.md` §3.7](../../../docs/resources/SPEC.md) — stats.
//!
//! ## Status at STR-002
//!
//! Empty aside from the [`STR_002_MODULE_PRESENT`] sentinel.

/// Sentinel marker proving the module exists and is reachable at
/// `dig_epoch::types::events::STR_002_MODULE_PRESENT`.
///
/// Exercised by the STR-002 integration test — see
/// [`tests/crate_structure/str_002_test.rs`](../../../tests/crate_structure/str_002_test.rs).
#[doc(hidden)]
pub const STR_002_MODULE_PRESENT: () = ();

use dig_block::Checkpoint;

use crate::types::epoch_phase::EpochPhase;

// -----------------------------------------------------------------------------
// TYP-005 — EpochEvent
// -----------------------------------------------------------------------------

/// Events emitted by EpochManager for telemetry and driver notification.
///
/// Spec ref: SPEC §3.7 / TYP-005.
#[derive(Debug, Clone)]
pub enum EpochEvent {
    /// A new epoch has begun.
    EpochStarted {
        /// Epoch number.
        epoch: u64,
        /// L1 height at which the epoch started.
        l1_height: u32,
    },
    /// The epoch's phase has advanced.
    PhaseChanged {
        /// Epoch number.
        epoch: u64,
        /// Phase before the transition.
        from: EpochPhase,
        /// Phase after the transition.
        to: EpochPhase,
        /// L1 height at which the transition was observed.
        l1_height: u32,
    },
    /// Epoch finalization completed successfully.
    EpochFinalized {
        /// Epoch number.
        epoch: u64,
        /// Winning checkpoint.
        checkpoint: Checkpoint,
    },
    /// Epoch finalization failed.
    EpochFailed {
        /// Epoch number.
        epoch: u64,
    },
}

// -----------------------------------------------------------------------------
// TYP-007 — EpochStats
// -----------------------------------------------------------------------------

/// Aggregate statistics across all epochs managed by EpochManager.
///
/// Spec ref: SPEC §3.8 / TYP-007.
#[derive(Debug, Clone, Default)]
pub struct EpochStats {
    /// Total number of epochs managed (finalized and not).
    pub total_epochs: u64,
    /// Number of epochs that completed with a checkpoint.
    pub finalized_epochs: u64,
    /// Total L2 blocks across all epochs.
    pub total_blocks: u64,
    /// Total transaction count across all epochs.
    pub total_transactions: u64,
    /// Total fees collected across all epochs (mojos).
    pub total_fees: u64,
}
