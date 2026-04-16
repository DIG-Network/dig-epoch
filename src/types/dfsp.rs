//! # `types::dfsp` — DFSP epoch-boundary type definitions
//!
//! **Introduced by:** `STR-002` — Module hierarchy (SPEC §13).
//!
//! **Future owner:**
//! [`TYP-004`](../../../docs/requirements/domains/epoch_types/specs/TYP-004.md)
//! — the seven DFSP epoch-boundary types enumerated by SPEC §3.14:
//!
//! 1. `DfspCloseSnapshot`
//! 2. `DfspExecutionStageV1`
//! 3. `DfspEpochBurnContextV1`
//! 4. `DfspEpochBurnPolicyV1`
//! 5. `DfspEpochBurnPolicyScheduleEntryV1`
//! 6. `DfspEpochStorageProofEvaluationContextV1`
//! 7. `DfspEpochIssuancePreviewV1`
//!
//! **Spec reference:** [`SPEC.md` §3.14](../../../docs/resources/SPEC.md).
//!
//! ## Relationship to `src/dfsp.rs`
//!
//! This module owns the **type surface** of DFSP. The **processing
//! functions** (burn policy evaluation, operations digest, commitment
//! digest, namespace rollup, tail roots, activation control) live in
//! [`crate::dfsp`] per STR-002's Implementation Notes. Both modules must
//! exist simultaneously — the `types` variant is the one reachable at
//! `dig_epoch::types::dfsp::*`.
//!
//! ## Status at STR-002
//!
//! Empty aside from the [`STR_002_MODULE_PRESENT`] sentinel.

/// Sentinel marker proving the module exists and is reachable at
/// `dig_epoch::types::dfsp::STR_002_MODULE_PRESENT`.
///
/// Exercised by the STR-002 integration test — see
/// [`tests/crate_structure/str_002_test.rs`](../../../tests/crate_structure/str_002_test.rs).
#[doc(hidden)]
pub const STR_002_MODULE_PRESENT: () = ();
