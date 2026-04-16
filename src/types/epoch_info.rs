//! # `types::epoch_info` — `EpochInfo` mutable epoch state
//!
//! **Introduced by:** `STR-002` — Module hierarchy (SPEC §13).
//!
//! **Future owner:**
//! [`TYP-002`](../../../docs/requirements/domains/epoch_types/specs/TYP-002.md)
//! — the `EpochInfo` struct with all fields (chain totals, DFSP snapshot
//! slot, checkpoint slot, phase-calculation helpers such as
//! `calculate_phase`, `record_block`, `set_checkpoint`, and construction
//! via `EpochInfo::new(epoch, start_l1_height, start_height,
//! initial_state_root)`).
//!
//! **Spec reference:** [`SPEC.md` §3.2](../../../docs/resources/SPEC.md)
//! — `EpochInfo` shape.
//!
//! ## Relationship to `EpochSummary`
//!
//! `EpochInfo` is the **mutable** accumulator for an in-progress epoch;
//! when an epoch closes it is converted into an immutable
//! [`crate::types::epoch_summary::EpochSummary`] and archived in
//! [`crate::manager::EpochManager`].
//!
//! ## Status at STR-002
//!
//! Empty aside from the [`STR_002_MODULE_PRESENT`] sentinel.

/// Sentinel marker proving the module exists and is reachable at
/// `dig_epoch::types::epoch_info::STR_002_MODULE_PRESENT`.
///
/// Exercised by the STR-002 integration test — see
/// [`tests/crate_structure/str_002_test.rs`](../../../tests/crate_structure/str_002_test.rs).
#[doc(hidden)]
pub const STR_002_MODULE_PRESENT: () = ();
