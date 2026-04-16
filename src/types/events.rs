//! # `types::events` — epoch event enum and stats struct
//!
//! **Introduced by:** `STR-002` — Module hierarchy (SPEC §13).
//!
//! **Future owner:**
//! [`TYP-005`](../../../docs/requirements/domains/epoch_types/specs/TYP-005.md)
//! — the `EpochEvent` enum (4 variants: `EpochAdvanced`, `PhaseTransitioned`,
//! `CheckpointCompetitionStarted`, `CheckpointFinalized`) together with
//! the `EpochStats` struct summarising cross-epoch counters.
//!
//! **Spec reference:** [`SPEC.md` §3.6](../../../docs/resources/SPEC.md) —
//! events; [`SPEC.md` §3.7](../../../docs/resources/SPEC.md) — stats.
//!
//! ## Status at STR-002
//!
//! Empty aside from the [`STR_002_MODULE_PRESENT`] sentinel.

/// Sentinel marker proving the module exists and is reachable at
/// `dig_epoch::types::events::STR_002_MODULE_PRESENT`.
///
/// Exercised by the STR-002 integration test — see
/// [`tests/crate_structure/test_module_hierarchy.rs`](../../../tests/crate_structure/test_module_hierarchy.rs).
#[doc(hidden)]
pub const STR_002_MODULE_PRESENT: () = ();
