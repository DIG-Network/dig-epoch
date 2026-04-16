//! # `error` — crate-wide error types
//!
//! **Introduced by:** `STR-002` — Module hierarchy (SPEC §13).
//!
//! **Future owners:** Phase 2 of `IMPLEMENTATION_ORDER.md`:
//!
//! - [`ERR-001`](../../docs/requirements/domains/error_types/specs/ERR-001.md)
//!   — `EpochError` enum with all variants
//! - [`ERR-002`](../../docs/requirements/domains/error_types/specs/ERR-002.md)
//!   — `CheckpointCompetitionError` enum
//! - [`ERR-003`](../../docs/requirements/domains/error_types/specs/ERR-003.md)
//!   — `From` conversions and `Display` messages
//!
//! **Spec reference:**
//! [`SPEC.md` §13](../../docs/resources/SPEC.md) — canonical module list;
//! [`SPEC.md` §11](../../docs/resources/SPEC.md) — error taxonomy.
//!
//! ## Content rule
//!
//! Both error enums derive their surface from the `thiserror` crate
//! (pinned in `Cargo.toml` by STR-001). Per SPEC §11, no error variant
//! may leak internal details of the `parking_lot::RwLock` (lock poisoning
//! does not apply — `parking_lot` does not poison), nor may any variant
//! wrap a panic-producing path.
//!
//! ## Status at STR-002
//!
//! Empty aside from the [`STR_002_MODULE_PRESENT`] sentinel.

/// Sentinel marker proving the module exists and is reachable at
/// `dig_epoch::error::STR_002_MODULE_PRESENT`.
///
/// Exercised by the STR-002 integration test — see
/// [`tests/crate_structure/str_002_test.rs`](../../tests/crate_structure/str_002_test.rs)
/// (row 10, `test_error_module`).
#[doc(hidden)]
pub const STR_002_MODULE_PRESENT: () = ();

use crate::types::epoch_phase::EpochPhase;

// -----------------------------------------------------------------------------
// ERR-002 — CheckpointCompetitionError
// -----------------------------------------------------------------------------

/// Errors within the checkpoint competition lifecycle.
///
/// Spec ref: SPEC §10.2 / ERR-002.
#[derive(Debug, Clone, thiserror::Error)]
pub enum CheckpointCompetitionError {
    /// Checkpoint data failed validation.
    #[error("Invalid checkpoint data: {0}")]
    InvalidData(String),

    /// No competition exists for the requested epoch.
    #[error("Checkpoint competition not found for epoch {0}")]
    NotFound(u64),

    /// Submitted checkpoint's score does not exceed the current leader.
    #[error("Score not higher: current {current}, submitted {submitted}")]
    ScoreNotHigher {
        /// Current leading score.
        current: u64,
        /// Score of the new submission.
        submitted: u64,
    },

    /// Submission's epoch field doesn't match the competition's epoch.
    #[error("Epoch mismatch: expected {expected}, got {got}")]
    EpochMismatch {
        /// Epoch the competition is running for.
        expected: u64,
        /// Epoch field in the submission.
        got: u64,
    },

    /// Competition has already been finalized.
    #[error("Competition already finalized")]
    AlreadyFinalized,

    /// Competition hasn't been started yet.
    #[error("Competition not started")]
    NotStarted,
}

// -----------------------------------------------------------------------------
// ERR-001 — EpochError
// -----------------------------------------------------------------------------

/// Primary error type for the dig-epoch crate.
///
/// Spec ref: SPEC §10.1 / ERR-001.
#[derive(Debug, Clone, thiserror::Error)]
pub enum EpochError {
    /// Attempted to advance an epoch that hasn't reached Complete phase.
    #[error("Cannot advance: epoch {0} is not complete")]
    EpochNotComplete(u64),

    /// Attempted to advance an epoch with no finalized checkpoint.
    #[error("Cannot advance: epoch {0} has no finalized checkpoint")]
    NoFinalizedCheckpoint(u64),

    /// Checkpoint-class block contains non-zero SpendBundles, cost, or fees.
    #[error("Checkpoint block at height {0} is not empty: {1} bundles, {2} cost, {3} fees")]
    CheckpointBlockNotEmpty(u64, u32, u64, u64),

    /// Operation requires a specific phase but the epoch is in a different one.
    #[error("Phase mismatch: expected {expected}, got {got}")]
    PhaseMismatch {
        /// Required phase.
        expected: EpochPhase,
        /// Actual current phase.
        got: EpochPhase,
    },

    /// Submission or query references the wrong epoch.
    #[error("Epoch mismatch: expected {expected}, got {got}")]
    EpochMismatch {
        /// Expected epoch number.
        expected: u64,
        /// Actual epoch number received.
        got: u64,
    },

    /// L2 height is below genesis (height 0 or underflow).
    #[error("Invalid height {0}: below genesis")]
    InvalidHeight(u64),

    /// DFSP operation attempted at a height before activation.
    #[error("DFSP not active at height {0}")]
    DfspNotActive(u64),

    /// DFSP epoch-boundary processing error.
    #[error("DFSP epoch-boundary error: {0}")]
    DfspBoundary(String),

    /// Checkpoint competition error (delegated via `#[from]`).
    #[error("Competition error: {0}")]
    Competition(#[from] CheckpointCompetitionError),
}
