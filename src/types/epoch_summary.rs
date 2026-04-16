//! # `types::epoch_summary` — `EpochSummary` immutable archive
//!
//! **Introduced by:** `STR-002` — Module hierarchy (SPEC §13).
//!
//! **Future owner:**
//! [`TYP-003`](../../../docs/requirements/domains/epoch_types/specs/TYP-003.md)
//! — the `EpochSummary` struct, produced from an
//! [`crate::types::epoch_info::EpochInfo`] at the moment an epoch closes.
//! Unlike `EpochInfo`, `EpochSummary` is immutable: once archived by the
//! [`crate::manager::EpochManager`] it is never mutated again.
//!
//! **Spec reference:** [`SPEC.md` §3.3](../../../docs/resources/SPEC.md) —
//! `EpochSummary` shape.
//!
//! ## Status at STR-002
//!
//! Empty aside from the [`STR_002_MODULE_PRESENT`] sentinel.

/// Sentinel marker proving the module exists and is reachable at
/// `dig_epoch::types::epoch_summary::STR_002_MODULE_PRESENT`.
///
/// Exercised by the STR-002 integration test — see
/// [`tests/crate_structure/test_module_hierarchy.rs`](../../../tests/crate_structure/test_module_hierarchy.rs).
#[doc(hidden)]
pub const STR_002_MODULE_PRESENT: () = ();
