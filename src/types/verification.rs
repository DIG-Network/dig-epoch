//! # `types::verification` — verification-related data types
//!
//! **Introduced by:** `STR-002` — Module hierarchy (SPEC §13).
//!
//! **Future owner:**
//! [`TYP-006`](../../../docs/requirements/domains/epoch_types/specs/TYP-006.md)
//! — verification-related data structures:
//!
//! - `EpochCheckpointData` with `hash()` and `test_coin_id()`
//! - `EpochBlockLink`
//!
//! **Spec reference:** [`SPEC.md` §7](../../../docs/resources/SPEC.md) —
//! verification.
//!
//! ## Relationship to `src/verification.rs`
//!
//! This module owns the **type surface** of verification. The **free
//! functions** that compute verification artefacts (epoch block Merkle
//! root, inclusion proofs, withdrawals Merkle-set root, checkpoint signing
//! material) live in [`crate::verification`] per STR-002's Implementation
//! Notes. Both modules must exist simultaneously with distinct
//! responsibilities.
//!
//! ## Status at STR-002
//!
//! Empty aside from the [`STR_002_MODULE_PRESENT`] sentinel.

/// Sentinel marker proving the module exists and is reachable at
/// `dig_epoch::types::verification::STR_002_MODULE_PRESENT`.
///
/// Exercised by the STR-002 integration test — see
/// [`tests/crate_structure/str_002_test.rs`](../../../tests/crate_structure/str_002_test.rs).
#[doc(hidden)]
pub const STR_002_MODULE_PRESENT: () = ();

use chia_protocol::Bytes32;
use chia_sha2::Sha256;
use serde::{Deserialize, Serialize};

// -----------------------------------------------------------------------------
// TYP-006 — EpochBlockLink
// -----------------------------------------------------------------------------

/// Parent-to-child relationship between consecutive blocks within an epoch.
///
/// Spec ref: SPEC §3.13 / TYP-006.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpochBlockLink {
    /// Hash of the parent block.
    pub parent_hash: Bytes32,
    /// Hash of the current block.
    pub block_hash: Bytes32,
}

// -----------------------------------------------------------------------------
// VER-004 — EpochCheckpointData
// -----------------------------------------------------------------------------

/// Minimum data needed to identify and verify an epoch checkpoint.
///
/// Binds `network_id` to the other fields to prevent cross-network replay.
///
/// Spec ref: SPEC §7.4 / VER-004.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EpochCheckpointData {
    /// Network identifier (prevents cross-network replay).
    pub network_id: Bytes32,
    /// Epoch number this checkpoint summarizes.
    pub epoch: u64,
    /// Ordered Merkle root over epoch block hashes (VER-001).
    pub block_root: Bytes32,
    /// Post-epoch L2 state root.
    pub state_root: Bytes32,
    /// Order-independent Merkle set root over epoch withdrawals (VER-003).
    pub withdrawals_root: Bytes32,
    /// Hash of the underlying [`dig_block::Checkpoint`] struct.
    pub checkpoint_hash: Bytes32,
}

impl EpochCheckpointData {
    /// SHA-256 over `network_id || epoch_le || block_root || state_root ||
    /// withdrawals_root || checkpoint_hash`.
    pub fn signing_digest(&self) -> Bytes32 {
        let mut h = Sha256::new();
        h.update(self.network_id.as_ref());
        h.update(self.epoch.to_le_bytes());
        h.update(self.block_root.as_ref());
        h.update(self.state_root.as_ref());
        h.update(self.withdrawals_root.as_ref());
        h.update(self.checkpoint_hash.as_ref());
        Bytes32::new(h.finalize())
    }
}

/// Validator-facing signing material: [`EpochCheckpointData`] plus the
/// competition score and the exact bytes to sign.
///
/// Spec ref: SPEC §7.5 / VER-004.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EpochCheckpointSignMaterial {
    /// Core checkpoint data (network-bound).
    pub checkpoint: EpochCheckpointData,
    /// Competition score = `stake_percentage * block_count` (CKP-004).
    pub score: u64,
    /// BLS signing digest (hash of `checkpoint`).
    pub signing_digest: Bytes32,
}
