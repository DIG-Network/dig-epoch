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
