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
use serde::{Deserialize, Serialize};

use crate::error::{CheckpointCompetitionError, EpochError};

// -----------------------------------------------------------------------------
// CKP-001 — CompetitionStatus
// -----------------------------------------------------------------------------

/// State machine for a checkpoint competition.
///
/// Spec ref: SPEC §3.10 / CKP-001.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
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

    // -------------------------------------------------------------------------
    // CKP-002 — start
    // -------------------------------------------------------------------------

    /// Transitions `Pending` → `Collecting`.
    ///
    /// Returns `Err(NotStarted)` if already past `Pending`.
    /// (Reusing `NotStarted` as the generic "wrong-lifecycle-state" error;
    /// specific variants may be refined in later tightening.)
    pub fn start(&mut self) -> Result<(), CheckpointCompetitionError> {
        if self.status != CompetitionStatus::Pending {
            return Err(CheckpointCompetitionError::AlreadyFinalized);
        }
        self.status = CompetitionStatus::Collecting;
        Ok(())
    }

    // -------------------------------------------------------------------------
    // CKP-003 — submit
    // -------------------------------------------------------------------------

    /// Records a checkpoint submission and, if its score beats the current leader,
    /// transitions status to `WinnerSelected` (or updates the existing one).
    ///
    /// Returns `true` when the submission became the new leader.
    ///
    /// Errors:
    /// - `NotStarted`            — status is `Pending`
    /// - `AlreadyFinalized`      — status is `Finalized` / `Failed`
    /// - `EpochMismatch`         — `submission.epoch` != `self.epoch`
    /// - `ScoreNotHigher`        — score does not exceed current leader
    pub fn submit(
        &mut self,
        submission: CheckpointSubmission,
    ) -> Result<bool, CheckpointCompetitionError> {
        match self.status {
            CompetitionStatus::Pending => {
                return Err(CheckpointCompetitionError::NotStarted);
            }
            CompetitionStatus::Finalized { .. } | CompetitionStatus::Failed => {
                return Err(CheckpointCompetitionError::AlreadyFinalized);
            }
            CompetitionStatus::Collecting | CompetitionStatus::WinnerSelected { .. } => {}
        }
        if submission.checkpoint.epoch != self.epoch {
            return Err(CheckpointCompetitionError::EpochMismatch {
                expected: self.epoch,
                got: submission.checkpoint.epoch,
            });
        }
        let new_score = submission.score;
        let current_score = match &self.status {
            CompetitionStatus::WinnerSelected { winner_score, .. } => *winner_score,
            _ => 0,
        };
        // Always record the submission for auditability.
        self.submissions.push(submission);
        let idx = self.submissions.len() - 1;
        let is_new_leader = match &self.status {
            CompetitionStatus::WinnerSelected { .. } => new_score > current_score,
            _ => new_score > 0,
        };
        if is_new_leader {
            let winner_hash = self.submissions[idx].checkpoint.hash();
            self.status = CompetitionStatus::WinnerSelected {
                winner_hash,
                winner_score: new_score,
            };
            self.current_winner = Some(idx);
            Ok(true)
        } else {
            Err(CheckpointCompetitionError::ScoreNotHigher {
                current: current_score,
                submitted: new_score,
            })
        }
    }

    // -------------------------------------------------------------------------
    // CKP-004 — finalize
    // -------------------------------------------------------------------------

    /// Transitions `WinnerSelected` → `Finalized { winner_hash, l1_height }`.
    ///
    /// Returns the winning checkpoint hash.
    pub fn finalize(&mut self, l1_height: u32) -> Result<Bytes32, CheckpointCompetitionError> {
        let winner_hash = match self.status {
            CompetitionStatus::WinnerSelected { winner_hash, .. } => winner_hash,
            CompetitionStatus::Finalized { .. } => {
                return Err(CheckpointCompetitionError::AlreadyFinalized);
            }
            _ => return Err(CheckpointCompetitionError::NotStarted),
        };
        self.status = CompetitionStatus::Finalized {
            winner_hash,
            l1_height,
        };
        Ok(winner_hash)
    }

    /// Transitions to `Failed` (terminal). Legal from `Collecting` or
    /// `WinnerSelected`.
    pub fn fail(&mut self) -> Result<(), CheckpointCompetitionError> {
        match self.status {
            CompetitionStatus::Collecting | CompetitionStatus::WinnerSelected { .. } => {
                self.status = CompetitionStatus::Failed;
                Ok(())
            }
            CompetitionStatus::Finalized { .. } | CompetitionStatus::Failed => {
                Err(CheckpointCompetitionError::AlreadyFinalized)
            }
            CompetitionStatus::Pending => Err(CheckpointCompetitionError::NotStarted),
        }
    }

    /// Returns the winning checkpoint submission, if one has been selected.
    pub fn winner(&self) -> Option<&CheckpointSubmission> {
        self.current_winner.and_then(|i| self.submissions.get(i))
    }

    /// Serializes with bincode. Infallible for well-formed structs.
    pub fn to_bytes(&self) -> Vec<u8> {
        bincode::serialize(self).expect("CheckpointCompetition serialization should never fail")
    }

    /// Deserializes from bincode bytes, returning `EpochError::InvalidData` on failure.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, EpochError> {
        bincode::deserialize(bytes).map_err(|e| EpochError::InvalidData(e.to_string()))
    }
}
