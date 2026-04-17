//! # `manager` — `EpochManager` struct and methods
//!
//! **Introduced by:** `STR-002` — Module hierarchy (SPEC §13).
//!
//! **Owners:** `MGR-001` (struct) / `MGR-002..MGR-008` (methods).
//! Phase tracking (PHS-002/003/004) is wired through the same struct.
//!
//! **Spec reference:** [`SPEC.md` §6](../../docs/resources/SPEC.md)

/// Sentinel marker proving the module exists and is reachable at
/// `dig_epoch::manager::STR_002_MODULE_PRESENT`.
#[doc(hidden)]
pub const STR_002_MODULE_PRESENT: () = ();

use std::collections::HashMap;

use chia_protocol::Bytes32;
use parking_lot::RwLock;

use crate::arithmetic::l1_range_for_epoch;
use crate::constants::{EPOCH_L1_BLOCKS, GENESIS_HEIGHT};
use crate::error::EpochError;
use crate::phase::l1_progress_phase_for_network_epoch;
use crate::types::checkpoint_competition::CheckpointCompetition;
use crate::types::dfsp::DfspCloseSnapshot;
use crate::types::epoch_info::EpochInfo;
use crate::types::epoch_phase::{EpochPhase, PhaseTransition};
use crate::types::epoch_summary::EpochSummary;
use crate::types::events::EpochStats;
use crate::types::reward::RewardDistribution;

// -----------------------------------------------------------------------------
// MGR-001 — EpochManagerInner
// -----------------------------------------------------------------------------

/// Private inner state for [`EpochManager`]. All access goes through
/// [`EpochManager`] methods, which acquire the outer `RwLock`.
struct EpochManagerInner {
    network_id: Bytes32,
    genesis_l1_height: u32,
    current_epoch: EpochInfo,
    competition: CheckpointCompetition,
    summaries: Vec<EpochSummary>,
    rewards: HashMap<u64, RewardDistribution>,
}

/// Primary state machine managing the current epoch's lifecycle and
/// archiving completed epochs.
///
/// Uses `parking_lot::RwLock` for interior mutability (start.md Hard
/// Requirement 12). Read operations allow concurrent access; write
/// operations block all other access.
pub struct EpochManager {
    inner: RwLock<EpochManagerInner>,
}

impl EpochManager {
    // -------------------------------------------------------------------------
    // MGR-001 / SPEC §6.2 — construction
    // -------------------------------------------------------------------------

    /// Creates an [`EpochManager`] at epoch 0 with empty history and a
    /// fresh `Pending` competition.
    ///
    /// `network_id` and `genesis_l1_height` are immutable for the lifetime
    /// of this manager. The initial `EpochInfo` starts at
    /// `GENESIS_HEIGHT` in `BlockProduction` phase.
    pub fn new(network_id: Bytes32, genesis_l1_height: u32, initial_state_root: Bytes32) -> Self {
        let current_epoch =
            EpochInfo::new(0, genesis_l1_height, GENESIS_HEIGHT, initial_state_root);
        Self {
            inner: RwLock::new(EpochManagerInner {
                network_id,
                genesis_l1_height,
                current_epoch,
                competition: CheckpointCompetition::new(0),
                summaries: Vec::new(),
                rewards: HashMap::new(),
            }),
        }
    }

    // -------------------------------------------------------------------------
    // MGR-008 / SPEC §6.3 — accessors
    // -------------------------------------------------------------------------

    /// Returns the current epoch number.
    pub fn current_epoch(&self) -> u64 {
        self.inner.read().current_epoch.epoch
    }

    /// Returns a clone of the current epoch's full state.
    pub fn current_epoch_info(&self) -> EpochInfo {
        self.inner.read().current_epoch.clone()
    }

    /// Returns the current phase of the current epoch.
    pub fn current_phase(&self) -> EpochPhase {
        self.inner.read().current_epoch.phase
    }

    /// Returns the network's genesis L1 height.
    pub fn genesis_l1_height(&self) -> u32 {
        self.inner.read().genesis_l1_height
    }

    /// Returns the network ID.
    pub fn network_id(&self) -> Bytes32 {
        self.inner.read().network_id
    }

    /// Maps an L1 height to its epoch number using `genesis_l1_height`.
    ///
    /// Heights before genesis map to epoch 0.
    pub fn epoch_for_l1_height(&self, l1_height: u32) -> u64 {
        let g = self.genesis_l1_height();
        if l1_height <= g {
            0
        } else {
            ((l1_height - g) / EPOCH_L1_BLOCKS) as u64
        }
    }

    /// Returns `(start_l1, end_l1)` for the given epoch.
    pub fn l1_range_for_epoch(&self, epoch: u64) -> (u32, u32) {
        l1_range_for_epoch(self.genesis_l1_height(), epoch)
    }

    // -------------------------------------------------------------------------
    // PHS-002 / MGR-008 — update_phase
    // -------------------------------------------------------------------------

    /// Recalculates the phase from `l1_height`. Returns `Some(PhaseTransition)`
    /// if the phase changed, `None` if unchanged.
    pub fn update_phase(&self, l1_height: u32) -> Option<PhaseTransition> {
        let mut inner = self.inner.write();
        let old_phase = inner.current_epoch.phase;
        let new_phase = l1_progress_phase_for_network_epoch(
            inner.genesis_l1_height,
            inner.current_epoch.epoch,
            l1_height,
        );
        if new_phase != old_phase {
            inner.current_epoch.phase = new_phase;
            Some(PhaseTransition {
                epoch: inner.current_epoch.epoch,
                from: old_phase,
                to: new_phase,
                l1_height,
            })
        } else {
            None
        }
    }

    /// Returns `true` when the current phase is `Complete`.
    pub fn should_advance(&self, _l1_height: u32) -> bool {
        self.current_phase() == EpochPhase::Complete
    }

    // -------------------------------------------------------------------------
    // MGR-002 — record_block (PHS-004 phase-gated)
    // -------------------------------------------------------------------------

    /// Records a block in the current epoch.
    ///
    /// Returns `Err(PhaseMismatch)` if not in `BlockProduction`.
    pub fn record_block(&self, fees: u64, tx_count: u64) -> Result<(), EpochError> {
        let mut inner = self.inner.write();
        if inner.current_epoch.phase != EpochPhase::BlockProduction {
            return Err(EpochError::PhaseMismatch {
                expected: EpochPhase::BlockProduction,
                got: inner.current_epoch.phase,
            });
        }
        inner.current_epoch.record_block(fees, tx_count);
        Ok(())
    }

    // -------------------------------------------------------------------------
    // MGR-003 — set_current_epoch_chain_totals
    // -------------------------------------------------------------------------

    /// Overwrites the current epoch's block production statistics.
    ///
    /// Used for resync / correction. No phase restriction; values are
    /// replaced, not incremented.
    pub fn set_current_epoch_chain_totals(&self, blocks: u32, fees: u64, txns: u64) {
        let mut inner = self.inner.write();
        inner.current_epoch.blocks_produced = blocks;
        inner.current_epoch.total_fees = fees;
        inner.current_epoch.total_transactions = txns;
    }

    // -------------------------------------------------------------------------
    // MGR-006 — set_current_epoch_dfsp_close_snapshot
    // -------------------------------------------------------------------------

    /// Applies DFSP close values to the current epoch before advance.
    ///
    /// Returns `Err(PhaseMismatch)` if not in `Finalization`.
    pub fn set_current_epoch_dfsp_close_snapshot(
        &self,
        snap: DfspCloseSnapshot,
    ) -> Result<(), EpochError> {
        let mut inner = self.inner.write();
        if inner.current_epoch.phase != EpochPhase::Finalization {
            return Err(EpochError::PhaseMismatch {
                expected: EpochPhase::Finalization,
                got: inner.current_epoch.phase,
            });
        }
        inner.current_epoch.collateral_registry_root = snap.collateral_registry_root;
        inner.current_epoch.cid_state_root = snap.cid_state_root;
        inner.current_epoch.node_registry_root = snap.node_registry_root;
        inner.current_epoch.namespace_epoch_root = snap.namespace_epoch_root;
        inner.current_epoch.dfsp_issuance_total = snap.dfsp_issuance_total;
        inner.current_epoch.active_cid_count = snap.active_cid_count;
        inner.current_epoch.active_node_count = snap.active_node_count;
        Ok(())
    }

    // -------------------------------------------------------------------------
    // MGR-004 — advance_epoch
    // -------------------------------------------------------------------------

    /// Archives the current epoch and transitions to `epoch + 1`.
    ///
    /// Preconditions:
    /// - Current phase is `Complete`.
    /// - Current competition is `Finalized`.
    ///
    /// Both preconditions are checked before any state mutation.
    pub fn advance_epoch(&self, _l1_height: u32, state_root: Bytes32) -> Result<u64, EpochError> {
        let mut inner = self.inner.write();
        let current_epoch_num = inner.current_epoch.epoch;
        if inner.current_epoch.phase != EpochPhase::Complete {
            return Err(EpochError::EpochNotComplete(current_epoch_num));
        }
        if !inner.competition.is_finalized() {
            return Err(EpochError::NoFinalizedCheckpoint(current_epoch_num));
        }

        let old_info = inner.current_epoch.clone();
        let next_epoch = current_epoch_num + 1;
        let next_start_l1 = inner.genesis_l1_height + (next_epoch as u32 * EPOCH_L1_BLOCKS);
        let next_start_l2 = old_info.start_l2_height + crate::constants::BLOCKS_PER_EPOCH;

        inner.summaries.push(EpochSummary::from(old_info));
        inner.current_epoch = EpochInfo::new(next_epoch, next_start_l1, next_start_l2, state_root);
        inner.competition = CheckpointCompetition::new(next_epoch);
        Ok(next_epoch)
    }

    // -------------------------------------------------------------------------
    // MGR-005 — query methods
    // -------------------------------------------------------------------------

    /// Returns a clone of the current `EpochInfo`.
    pub fn get_epoch_info(&self) -> EpochInfo {
        self.current_epoch_info()
    }

    /// Returns the `EpochSummary` for a specific completed epoch, or `None`.
    pub fn get_epoch_summary(&self, epoch: u64) -> Option<EpochSummary> {
        self.inner
            .read()
            .summaries
            .iter()
            .find(|s| s.epoch == epoch)
            .cloned()
    }

    /// Returns the last `n` summaries from the tail, preserving epoch order.
    pub fn recent_summaries(&self, n: usize) -> Vec<EpochSummary> {
        let inner = self.inner.read();
        let len = inner.summaries.len();
        let start = len.saturating_sub(n);
        inner.summaries[start..].to_vec()
    }

    /// Aggregate statistics across all completed epochs plus the current one.
    pub fn total_stats(&self) -> EpochStats {
        let inner = self.inner.read();
        let mut stats = EpochStats {
            total_epochs: inner.summaries.len() as u64 + 1,
            finalized_epochs: 0,
            total_blocks: 0,
            total_transactions: 0,
            total_fees: 0,
        };
        for s in &inner.summaries {
            if s.finalized {
                stats.finalized_epochs += 1;
            }
            stats.total_blocks += s.blocks as u64;
            stats.total_transactions += s.transactions;
            stats.total_fees += s.fees;
        }
        let cur = &inner.current_epoch;
        if cur.is_finalized() {
            stats.finalized_epochs += 1;
        }
        stats.total_blocks += cur.blocks_produced as u64;
        stats.total_transactions += cur.total_transactions;
        stats.total_fees += cur.total_fees;
        stats
    }

    /// Returns the [`RewardDistribution`] for `epoch`, or `None`.
    pub fn get_rewards(&self, epoch: u64) -> Option<RewardDistribution> {
        self.inner.read().rewards.get(&epoch).cloned()
    }

    // -------------------------------------------------------------------------
    // MGR-008 — store_rewards
    // -------------------------------------------------------------------------

    /// Archives a [`RewardDistribution`] keyed by its `epoch` field.
    pub fn store_rewards(&self, distribution: RewardDistribution) {
        let mut inner = self.inner.write();
        inner.rewards.insert(distribution.epoch, distribution);
    }

    // -------------------------------------------------------------------------
    // Internal accessors for checkpoint competition (used by MGR-004)
    // -------------------------------------------------------------------------

    /// Returns a clone of the current competition. Read-only.
    pub fn competition(&self) -> CheckpointCompetition {
        self.inner.read().competition.clone()
    }

    /// **Test / bootstrap helper** — directly overwrites the current competition.
    ///
    /// Used before CKP-002..005 provide full lifecycle methods. Not part of
    /// the stable SPEC §6.5 API.
    #[doc(hidden)]
    pub fn __set_competition_for_test(&self, competition: CheckpointCompetition) {
        self.inner.write().competition = competition;
    }

    /// **Test / bootstrap helper** — forces the current epoch into the given phase,
    /// bypassing the L1 progress calculation.
    ///
    /// Used to exercise phase-gated methods (MGR-003/004/006) before the
    /// phase machine is wired end-to-end. Not part of the stable SPEC API.
    #[doc(hidden)]
    pub fn __force_phase_for_test(&self, phase: EpochPhase) {
        self.inner.write().current_epoch.phase = phase;
    }

    // -------------------------------------------------------------------------
    // PHS-004 phase-gate stubs (superseded by CKP-003/004 in Phase 8)
    // -------------------------------------------------------------------------

    /// **Phase-check stub** for PHS-004.
    ///
    /// Returns `Err(PhaseMismatch)` if not in `Checkpoint`. CKP-003 will
    /// extend this into the full submission-accepting method.
    pub fn submit_checkpoint(&self) -> Result<(), EpochError> {
        let inner = self.inner.read();
        if inner.current_epoch.phase != EpochPhase::Checkpoint {
            return Err(EpochError::PhaseMismatch {
                expected: EpochPhase::Checkpoint,
                got: inner.current_epoch.phase,
            });
        }
        Ok(())
    }

    /// **Phase-check stub** for PHS-004.
    ///
    /// Returns `Err(PhaseMismatch)` if not in `Finalization`. CKP-004 will
    /// extend this into the full winner-selection method.
    pub fn finalize_competition(&self) -> Result<(), EpochError> {
        let inner = self.inner.read();
        if inner.current_epoch.phase != EpochPhase::Finalization {
            return Err(EpochError::PhaseMismatch {
                expected: EpochPhase::Finalization,
                got: inner.current_epoch.phase,
            });
        }
        Ok(())
    }
}
