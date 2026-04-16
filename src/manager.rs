//! # `manager` — `EpochManager` struct and methods
//!
//! **Introduced by:** `STR-002` — Module hierarchy (SPEC §13).
//!
//! **Core struct (MGR-001/STR-004):** Phase tracking (PHS-002/003/004).
//!
//! **Spec reference:** [`SPEC.md` §6](../../docs/resources/SPEC.md)

/// Sentinel marker proving the module exists and is reachable at
/// `dig_epoch::manager::STR_002_MODULE_PRESENT`.
#[doc(hidden)]
pub const STR_002_MODULE_PRESENT: () = ();

use chia_protocol::Bytes32;
use parking_lot::RwLock;

use crate::error::EpochError;
use crate::phase::l1_progress_phase_for_network_epoch;
use crate::types::epoch_info::EpochInfo;
use crate::types::epoch_phase::EpochPhase;
use crate::types::epoch_phase::PhaseTransition;

// -----------------------------------------------------------------------------
// MGR-001 / STR-004 — EpochManager struct
// -----------------------------------------------------------------------------

struct EpochManagerInner {
    current_epoch: EpochInfo,
    genesis_l1_height: u32,
}

/// Primary state machine for managing an epoch's lifecycle.
///
/// Uses `parking_lot::RwLock` for interior mutability (Hard Requirement 12).
pub struct EpochManager {
    inner: RwLock<EpochManagerInner>,
}

impl EpochManager {
    /// Creates a new `EpochManager` starting at the given epoch.
    pub fn new(
        genesis_l1_height: u32,
        epoch: u64,
        start_l2_height: u64,
        start_state_root: Bytes32,
    ) -> Self {
        let start_l1_height =
            genesis_l1_height + (epoch as u32 * crate::constants::EPOCH_L1_BLOCKS);
        let current_epoch =
            EpochInfo::new(epoch, start_l1_height, start_l2_height, start_state_root);
        Self {
            inner: RwLock::new(EpochManagerInner {
                current_epoch,
                genesis_l1_height,
            }),
        }
    }

    // -------------------------------------------------------------------------
    // PHS-002 — phase tracking
    // -------------------------------------------------------------------------

    /// Returns the current epoch phase.
    pub fn current_phase(&self) -> EpochPhase {
        self.inner.read().current_epoch.phase
    }

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

    // -------------------------------------------------------------------------
    // PHS-003 — should_advance
    // -------------------------------------------------------------------------

    /// Returns `true` when the current phase is `Complete`.
    pub fn should_advance(&self, _l1_height: u32) -> bool {
        self.current_phase() == EpochPhase::Complete
    }

    // -------------------------------------------------------------------------
    // PHS-004 — phase-gated operations
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

    /// Stub: submit a checkpoint. Phase-gated to `Checkpoint`.
    ///
    /// Returns `Err(PhaseMismatch)` if not in `Checkpoint`.
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

    /// Stub: finalize the checkpoint competition. Phase-gated to `Finalization`.
    ///
    /// Returns `Err(PhaseMismatch)` if not in `Finalization`.
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
