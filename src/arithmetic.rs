//! # `arithmetic` — pure height-to-epoch mapping functions
//!
//! **Introduced by:** `STR-002` — Module hierarchy (SPEC §13).
//!
//! **Future owners:** Phase 4 of `IMPLEMENTATION_ORDER.md` — the
//! height-epoch arithmetic domain:
//!
//! - [`HEA-001`](../../docs/requirements/domains/height_epoch_arithmetic/specs/HEA-001.md)
//!   — `epoch_for_block_height(h) -> u64`
//! - [`HEA-002`](../../docs/requirements/domains/height_epoch_arithmetic/specs/HEA-002.md)
//!   — `first_height_in_epoch(e)`, `epoch_checkpoint_height(e)`
//! - [`HEA-003`](../../docs/requirements/domains/height_epoch_arithmetic/specs/HEA-003.md)
//!   — checkpoint-class predicates and `ensure_checkpoint_block_empty`
//! - [`HEA-004`](../../docs/requirements/domains/height_epoch_arithmetic/specs/HEA-004.md)
//!   — `l1_range_for_epoch(genesis_l1_height, epoch)`
//! - [`HEA-005`](../../docs/requirements/domains/height_epoch_arithmetic/specs/HEA-005.md)
//!   — round-trip identity property tests
//!
//! **Spec reference:**
//! [`SPEC.md` §13](../../docs/resources/SPEC.md) — canonical module list;
//! [`SPEC.md` §3.1 / §4](../../docs/resources/SPEC.md) — epoch geometry.
//!
//! ## Content rule
//!
//! Per STR-002's Implementation Notes, the functions in this module are
//! **pure** — they read compile-time constants from [`crate::constants`]
//! and return a derived value. They MUST NOT depend on
//! [`crate::manager::EpochManager`] or any mutable state, because light
//! clients rely on them to reconstruct epoch geometry from an L1 height
//! alone.
//!
//! ## Status at STR-002
//!
//! Empty aside from the [`STR_002_MODULE_PRESENT`] sentinel — see the
//! matching doc comment in [`crate::constants`] for rationale.

/// Sentinel marker proving the module exists and is reachable at
/// `dig_epoch::arithmetic::STR_002_MODULE_PRESENT`.
///
/// Exercised by the STR-002 integration test — see
/// [`tests/crate_structure/test_module_hierarchy.rs`](../../tests/crate_structure/test_module_hierarchy.rs).
#[doc(hidden)]
pub const STR_002_MODULE_PRESENT: () = ();
