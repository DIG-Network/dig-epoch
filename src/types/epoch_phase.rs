//! # `types::epoch_phase` — `EpochPhase` enum and `PhaseTransition` struct
//!
//! **Introduced by:** `STR-002` — Module hierarchy (SPEC §13).
//!
//! **Future owner:**
//! [`TYP-001`](../../../docs/requirements/domains/epoch_types/specs/TYP-001.md)
//! — the `EpochPhase` enum (4 variants: `BlockProduction`,
//! `CheckpointBlockProduction`, `Finalization`, `Settlement`) together with
//! its inherent methods (`index`, `next`, `previous`, `name`, permission
//! helpers) and the `PhaseTransition` struct that records phase changes.
//!
//! **Spec reference:** [`SPEC.md` §4](../../../docs/resources/SPEC.md) —
//! phase state machine.
//!
//! ## Status at STR-002
//!
//! Empty aside from the [`STR_002_MODULE_PRESENT`] sentinel.
//! Free-function phase calculation lives in [`crate::phase`]; this module
//! is exclusively for the type definitions, per STR-002's types-vs-functions
//! split.

/// Sentinel marker proving the module exists and is reachable at
/// `dig_epoch::types::epoch_phase::STR_002_MODULE_PRESENT`.
///
/// Exercised by the STR-002 integration test — see
/// [`tests/crate_structure/str_002_test.rs`](../../../tests/crate_structure/str_002_test.rs).
#[doc(hidden)]
pub const STR_002_MODULE_PRESENT: () = ();
