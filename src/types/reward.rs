//! # `types::reward` — `RewardDistribution` struct
//!
//! **Owner:** REW-007 / REW-004.
//!
//! **Spec reference:** [`SPEC.md` §3.12](../../../docs/resources/SPEC.md)

/// Sentinel marker proving the module exists and is reachable at
/// `dig_epoch::types::reward::STR_002_MODULE_PRESENT`.
#[doc(hidden)]
pub const STR_002_MODULE_PRESENT: () = ();

// -----------------------------------------------------------------------------
// REW-007 — RewardDistribution
// -----------------------------------------------------------------------------

use serde::{Deserialize, Serialize};

use crate::error::EpochError;

/// Per-epoch reward split across five roles plus the fee distribution.
///
/// Spec ref: SPEC §3.12 / REW-007.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RewardDistribution {
    /// Epoch number.
    pub epoch: u64,
    /// Block proposer reward share (10% of total).
    pub proposer_reward: u64,
    /// Attester reward share (80% of total, absorbs rounding).
    pub attester_reward: u64,
    /// Epoch-first spawner reward share (3% of total).
    pub ef_spawner_reward: u64,
    /// Checkpoint score submitter reward share (4% of total).
    pub score_submitter_reward: u64,
    /// Competition finalizer reward share (3% of total).
    pub finalizer_reward: u64,
    /// Proposer's share of transaction fees (50% of total fees).
    pub proposer_fee_share: u64,
    /// Burned portion of transaction fees.
    pub burned_fees: u64,
}

impl RewardDistribution {
    /// Serializes with bincode. Infallible for well-formed structs.
    pub fn to_bytes(&self) -> Vec<u8> {
        bincode::serialize(self).expect("RewardDistribution serialization should never fail")
    }

    /// Deserializes from bincode bytes, returning `EpochError::InvalidData` on failure.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, EpochError> {
        bincode::deserialize(bytes).map_err(|e| EpochError::InvalidData(e.to_string()))
    }
}
