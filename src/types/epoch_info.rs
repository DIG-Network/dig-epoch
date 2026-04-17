//! # `types::epoch_info` — `EpochInfo` mutable epoch state
//!
//! **Introduced by:** `STR-002` — Module hierarchy (SPEC §13).
//!
//! **Owner:** `TYP-002` — mutable accumulator for the current epoch lifecycle.
//!
//! **Spec reference:** [`SPEC.md` §3.4](../../../docs/resources/SPEC.md)

/// Sentinel marker proving the module exists and is reachable at
/// `dig_epoch::types::epoch_info::STR_002_MODULE_PRESENT`.
#[doc(hidden)]
pub const STR_002_MODULE_PRESENT: () = ();

use chia_protocol::Bytes32;
use dig_block::Checkpoint;
use serde::{Deserialize, Serialize};

use crate::constants::{
    BLOCKS_PER_EPOCH, EMPTY_ROOT, EPOCH_L1_BLOCKS, PHASE_BLOCK_PRODUCTION_END_PCT,
    PHASE_CHECKPOINT_END_PCT, PHASE_FINALIZATION_END_PCT,
};
use crate::error::EpochError;
use crate::types::epoch_phase::EpochPhase;

/// Mutable state container for the current epoch.
///
/// Spec ref: SPEC §3.4 / TYP-002.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpochInfo {
    // -- Identity --
    /// Epoch number (0-indexed).
    pub epoch: u64,
    /// First L1 height in this epoch's window.
    pub start_l1_height: u32,
    /// Last L1 height in this epoch's window.
    pub end_l1_height: u32,
    /// First L2 block height in this epoch.
    pub start_l2_height: u64,

    // -- Mutable counters --
    /// L2 blocks recorded so far.
    pub blocks_produced: u32,
    /// Current phase.
    pub phase: EpochPhase,
    /// Accumulated fees (mojos).
    pub total_fees: u64,
    /// Accumulated transaction count.
    pub total_transactions: u64,

    // -- State --
    /// Winning checkpoint (set after finalization).
    pub checkpoint: Option<Checkpoint>,
    /// CoinSet state root at epoch start.
    pub start_state_root: Bytes32,

    // -- DFSP close snapshot --
    /// Collateral registry SMT root at close.
    pub collateral_registry_root: Bytes32,
    /// CID lifecycle state root at close.
    pub cid_state_root: Bytes32,
    /// Node registry SMT root at close.
    pub node_registry_root: Bytes32,
    /// Cumulative namespace root at close.
    pub namespace_epoch_root: Bytes32,
    /// Total DFSP issuance this epoch (mojos).
    pub dfsp_issuance_total: u64,
    /// Active CIDs at close.
    pub active_cid_count: u32,
    /// Active storage nodes at close.
    pub active_node_count: u32,
}

impl EpochInfo {
    /// Creates a new epoch with `BlockProduction` phase and zeroed counters.
    ///
    /// `end_l1_height = start_l1_height + EPOCH_L1_BLOCKS`. All DFSP roots
    /// default to `EMPTY_ROOT`; numeric DFSP fields default to 0.
    pub fn new(
        epoch: u64,
        start_l1_height: u32,
        start_l2_height: u64,
        start_state_root: Bytes32,
    ) -> Self {
        Self {
            epoch,
            start_l1_height,
            end_l1_height: start_l1_height + EPOCH_L1_BLOCKS,
            start_l2_height,
            blocks_produced: 0,
            phase: EpochPhase::BlockProduction,
            total_fees: 0,
            total_transactions: 0,
            checkpoint: None,
            start_state_root,
            collateral_registry_root: EMPTY_ROOT,
            cid_state_root: EMPTY_ROOT,
            node_registry_root: EMPTY_ROOT,
            namespace_epoch_root: EMPTY_ROOT,
            dfsp_issuance_total: 0,
            active_cid_count: 0,
            active_node_count: 0,
        }
    }

    /// Returns `BLOCKS_PER_EPOCH`.
    pub fn target_blocks(&self) -> u64 {
        BLOCKS_PER_EPOCH
    }

    /// True when phase is `BlockProduction`.
    pub fn can_produce_blocks(&self) -> bool {
        self.phase == EpochPhase::BlockProduction
    }

    /// True when phase is `Checkpoint`.
    pub fn can_submit_checkpoint(&self) -> bool {
        self.phase == EpochPhase::Checkpoint
    }

    /// True when phase is `Complete`.
    pub fn is_complete(&self) -> bool {
        self.phase == EpochPhase::Complete
    }

    /// True when a winning checkpoint has been recorded.
    pub fn is_finalized(&self) -> bool {
        self.checkpoint.is_some()
    }

    /// Increments `blocks_produced`, `total_fees`, and `total_transactions`.
    pub fn record_block(&mut self, fees: u64, tx_count: u64) {
        self.blocks_produced += 1;
        self.total_fees += fees;
        self.total_transactions += tx_count;
    }

    /// Records the winning checkpoint.
    pub fn set_checkpoint(&mut self, checkpoint: Checkpoint) {
        self.checkpoint = Some(checkpoint);
    }

    /// Returns 0–100 based on how far the epoch has progressed through its L1 window.
    ///
    /// Clamped at 100 — returns 100 once `current_l1_height >= end_l1_height`.
    pub fn progress_percentage(&self, current_l1_height: u32) -> u32 {
        if current_l1_height <= self.start_l1_height {
            return 0;
        }
        let elapsed = (current_l1_height - self.start_l1_height) as u64;
        let pct = elapsed * 100 / EPOCH_L1_BLOCKS as u64;
        pct.min(100) as u32
    }

    /// Deterministically computes the epoch phase from L1 progress percentage.
    pub fn calculate_phase(&self, current_l1_height: u32) -> EpochPhase {
        let pct = self.progress_percentage(current_l1_height);
        if pct < PHASE_BLOCK_PRODUCTION_END_PCT {
            EpochPhase::BlockProduction
        } else if pct < PHASE_CHECKPOINT_END_PCT {
            EpochPhase::Checkpoint
        } else if pct < PHASE_FINALIZATION_END_PCT {
            EpochPhase::Finalization
        } else {
            EpochPhase::Complete
        }
    }

    /// Serializes with bincode. Infallible for well-formed structs.
    pub fn to_bytes(&self) -> Vec<u8> {
        bincode::serialize(self).expect("EpochInfo serialization should never fail")
    }

    /// Deserializes from bincode bytes, returning `EpochError::InvalidData` on failure.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, EpochError> {
        bincode::deserialize(bytes).map_err(|e| EpochError::InvalidData(e.to_string()))
    }
}
