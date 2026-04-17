//! # `types::checkpoint_competition` — `CheckpointCompetition` struct and `CompetitionStatus` enum
//!
//! **Introduced by:** `STR-002` — Module hierarchy (SPEC §13).
//!
//! **Owner:** `CKP-001` — struct/enum surface. Lifecycle methods (start, submit,
//! finalize, lifecycle transitions) are added by `CKP-002`..`CKP-005`.
//!
//! **Spec reference:** [`SPEC.md` §3.9, §3.10](../../../docs/resources/SPEC.md).
//!
//! Per start.md Hard Requirement 1, this module MUST NOT redefine block types.
//! `Checkpoint` and `CheckpointSubmission` come from [`dig_block`].

/// Sentinel marker proving the module exists and is reachable at
/// `dig_epoch::types::checkpoint_competition::STR_002_MODULE_PRESENT`.
#[doc(hidden)]
pub const STR_002_MODULE_PRESENT: () = ();

use chia_protocol::Bytes32;
use dig_block::CheckpointSubmission;

// -----------------------------------------------------------------------------
// CKP-001 — CompetitionStatus
// -----------------------------------------------------------------------------

/// State machine for a checkpoint competition.
///
/// Spec ref: SPEC §3.10 / CKP-001.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompetitionStatus {
    /// Competition created but not yet accepting submissions.
    Pending,
    /// Actively accepting checkpoint submissions.
    Collecting,
    /// A winning submission has been identified by score.
    WinnerSelected {
        /// Hash of the winning checkpoint.
        winner_hash: Bytes32,
        /// Score of the winning submission.
        winner_score: u64,
    },
    /// Winner confirmed and anchored to an L1 height.
    Finalized {
        /// Hash of the winning checkpoint.
        winner_hash: Bytes32,
        /// L1 height at which the winner was anchored.
        l1_height: u32,
    },
    /// Competition ended due to timeout or error.
    Failed,
}

// -----------------------------------------------------------------------------
// CKP-001 — CheckpointCompetition
// -----------------------------------------------------------------------------

/// Per-epoch checkpoint competition, collecting submissions and selecting a winner.
///
/// Spec ref: SPEC §3.9 / CKP-001.
#[derive(Debug, Clone)]
pub struct CheckpointCompetition {
    /// Epoch this competition belongs to.
    pub epoch: u64,
    /// All checkpoint submissions received.
    pub submissions: Vec<CheckpointSubmission>,
    /// Current competition state.
    pub status: CompetitionStatus,
    /// Index into `submissions` of the current leader, if any.
    pub current_winner: Option<usize>,
}

impl CheckpointCompetition {
    /// Creates a new competition for `epoch` in `Pending` state with no submissions.
    pub fn new(epoch: u64) -> Self {
        Self {
            epoch,
            submissions: Vec::new(),
            status: CompetitionStatus::Pending,
            current_winner: None,
        }
    }

    /// True when the competition has reached the `Finalized` variant.
    pub fn is_finalized(&self) -> bool {
        matches!(self.status, CompetitionStatus::Finalized { .. })
    }
}
