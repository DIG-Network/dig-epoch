//! # `manager` — `EpochManager` struct and methods
//!
//! **Introduced by:** `STR-002` — Module hierarchy (SPEC §13).
//!
//! **Future owners:**
//!
//! - [`STR-004`](../../docs/requirements/domains/crate_structure/specs/STR-004.md)
//!   — `EpochManager::new(...)` constructor
//! - Phase 7 of `IMPLEMENTATION_ORDER.md` — the `epoch_manager` domain:
//!   - [`MGR-001`](../../docs/requirements/domains/epoch_manager/specs/MGR-001.md)
//!     — struct definition with interior mutability (`parking_lot::RwLock`)
//!   - [`MGR-002`](../../docs/requirements/domains/epoch_manager/specs/MGR-002.md)
//!     — `record_block(fees, tx_count)`
//!   - [`MGR-003`](../../docs/requirements/domains/epoch_manager/specs/MGR-003.md)
//!     — `set_current_epoch_chain_totals()`
//!   - [`MGR-004`](../../docs/requirements/domains/epoch_manager/specs/MGR-004.md)
//!     — `advance_epoch(l1_height, state_root)`
//!   - [`MGR-005`](../../docs/requirements/domains/epoch_manager/specs/MGR-005.md)
//!     — query methods (`get_epoch_info`, `get_epoch_summary`, …)
//!   - [`MGR-006`](../../docs/requirements/domains/epoch_manager/specs/MGR-006.md)
//!     — `set_current_epoch_dfsp_close_snapshot()`
//!   - [`MGR-007`](../../docs/requirements/domains/epoch_manager/specs/MGR-007.md)
//!     — epoch history / summaries storage
//!
//! **Spec reference:**
//! [`SPEC.md` §13](../../docs/resources/SPEC.md) — canonical module list;
//! [`SPEC.md` §6](../../docs/resources/SPEC.md) — EpochManager surface.
//!
//! ## Content rule
//!
//! Per start.md Hard Requirement 12, `EpochManager` MUST use interior
//! mutability via `parking_lot::RwLock` (not `std::sync::RwLock`). The
//! struct itself will be added by MGR-001; STR-002 only guarantees the
//! module is present so that STR-004 (constructor) has a home.
//!
//! All method implementations live here, but the *data types* they
//! operate on (`EpochInfo`, `EpochSummary`, `CheckpointCompetition`, …)
//! live under [`crate::types`] per STR-002's types-vs-functions split.
//!
//! ## Status at STR-002
//!
//! Empty aside from the [`STR_002_MODULE_PRESENT`] sentinel.

/// Sentinel marker proving the module exists and is reachable at
/// `dig_epoch::manager::STR_002_MODULE_PRESENT`.
///
/// Exercised by the STR-002 integration test — see
/// [`tests/crate_structure/test_module_hierarchy.rs`](../../tests/crate_structure/test_module_hierarchy.rs).
#[doc(hidden)]
pub const STR_002_MODULE_PRESENT: () = ();
