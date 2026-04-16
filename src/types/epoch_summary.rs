//! # `types::epoch_summary` — `EpochSummary` immutable archive
//!
//! **Introduced by:** `STR-002` — Module hierarchy (SPEC §13).
//!
//! **Owner:** `TYP-003` — immutable archive produced from `EpochInfo` at epoch close.
//!
//! **Spec reference:** [`SPEC.md` §3.5](../../../docs/resources/SPEC.md)

/// Sentinel marker proving the module exists and is reachable at
/// `dig_epoch::types::epoch_summary::STR_002_MODULE_PRESENT`.
#[doc(hidden)]
pub const STR_002_MODULE_PRESENT: () = ();

use chia_protocol::Bytes32;
use serde::{Deserialize, Serialize};

use crate::types::epoch_info::EpochInfo;

/// Immutable archive of a completed epoch.
///
/// Spec ref: SPEC §3.5 / TYP-003.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpochSummary {
    /// Epoch number.
    pub epoch: u64,
    /// L2 blocks produced.
    pub blocks: u32,
    /// Total transactions.
    pub transactions: u64,
    /// Total fees (mojos).
    pub fees: u64,
    /// True when a winning checkpoint was recorded.
    pub finalized: bool,
    /// Hash of the winning checkpoint, if any.
    pub checkpoint_hash: Option<Bytes32>,

    // -- DFSP state at close --
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

impl From<EpochInfo> for EpochSummary {
    fn from(info: EpochInfo) -> Self {
        let finalized = info.checkpoint.is_some();
        let checkpoint_hash = info.checkpoint.map(|c| c.hash());
        Self {
            epoch: info.epoch,
            blocks: info.blocks_produced,
            transactions: info.total_transactions,
            fees: info.total_fees,
            finalized,
            checkpoint_hash,
            collateral_registry_root: info.collateral_registry_root,
            cid_state_root: info.cid_state_root,
            node_registry_root: info.node_registry_root,
            namespace_epoch_root: info.namespace_epoch_root,
            dfsp_issuance_total: info.dfsp_issuance_total,
            active_cid_count: info.active_cid_count,
            active_node_count: info.active_node_count,
        }
    }
}
