//! # `dfsp` — root-level DFSP epoch-boundary processing
//!
//! **Introduced by:** `STR-002` — Module hierarchy (SPEC §13).
//!
//! **Future owners:** the `dfsp_processing` domain (DFS-*) — free functions
//! for DFSP (Distributed File Storage Proof) epoch-boundary work:
//!
//! - burn policy evaluation
//! - operations digest computation
//! - commitment digest computation
//! - namespace rollup
//! - tail-root derivation
//! - activation control
//!
//! See [`SPEC.md` §13](../../docs/resources/SPEC.md) for the canonical
//! function list and [`SPEC.md` §3.14](../../docs/resources/SPEC.md) for
//! the DFSP epoch-boundary semantics.
//!
//! ## Relationship to `types/dfsp.rs`
//!
//! STR-002's Implementation Notes make the split explicit:
//!
//! - **This module** (`src/dfsp.rs`) holds **processing free functions**
//!   that consume and produce DFSP snapshot types.
//! - [`crate::types::dfsp`] holds the **seven** DFSP data types enumerated
//!   in SPEC §3.14 (`DfspCloseSnapshot`, `DfspExecutionStageV1`,
//!   `DfspEpochBurnContextV1`, `DfspEpochBurnPolicyV1`,
//!   `DfspEpochBurnPolicyScheduleEntryV1`,
//!   `DfspEpochStorageProofEvaluationContextV1`,
//!   `DfspEpochIssuancePreviewV1`).
//!
//! Both modules must exist simultaneously with distinct responsibilities.
//!
//! ## Status at STR-002
//!
//! Empty aside from the [`STR_002_MODULE_PRESENT`] sentinel.

/// Sentinel marker proving the module exists and is reachable at
/// `dig_epoch::dfsp::STR_002_MODULE_PRESENT`.
///
/// Exercised by the STR-002 integration test — see
/// [`tests/crate_structure/test_module_hierarchy.rs`](../../tests/crate_structure/test_module_hierarchy.rs)
/// (row 9, `test_dfsp_module`).
#[doc(hidden)]
pub const STR_002_MODULE_PRESENT: () = ();
