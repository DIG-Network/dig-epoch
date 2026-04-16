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
/// [`tests/crate_structure/test_module_hierarchy.rs`](../../../tests/crate_structure/test_module_hierarchy.rs).
#[doc(hidden)]
pub const STR_002_MODULE_PRESENT: () = ();
