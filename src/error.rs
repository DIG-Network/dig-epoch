//! # `error` — crate-wide error types
//!
//! **Introduced by:** `STR-002` — Module hierarchy (SPEC §13).
//!
//! **Future owners:** Phase 2 of `IMPLEMENTATION_ORDER.md`:
//!
//! - [`ERR-001`](../../docs/requirements/domains/error_types/specs/ERR-001.md)
//!   — `EpochError` enum with all variants
//! - [`ERR-002`](../../docs/requirements/domains/error_types/specs/ERR-002.md)
//!   — `CheckpointCompetitionError` enum
//! - [`ERR-003`](../../docs/requirements/domains/error_types/specs/ERR-003.md)
//!   — `From` conversions and `Display` messages
//!
//! **Spec reference:**
//! [`SPEC.md` §13](../../docs/resources/SPEC.md) — canonical module list;
//! [`SPEC.md` §11](../../docs/resources/SPEC.md) — error taxonomy.
//!
//! ## Content rule
//!
//! Both error enums derive their surface from the `thiserror` crate
//! (pinned in `Cargo.toml` by STR-001). Per SPEC §11, no error variant
//! may leak internal details of the `parking_lot::RwLock` (lock poisoning
//! does not apply — `parking_lot` does not poison), nor may any variant
//! wrap a panic-producing path.
//!
//! ## Status at STR-002
//!
//! Empty aside from the [`STR_002_MODULE_PRESENT`] sentinel.

/// Sentinel marker proving the module exists and is reachable at
/// `dig_epoch::error::STR_002_MODULE_PRESENT`.
///
/// Exercised by the STR-002 integration test — see
/// [`tests/crate_structure/str_002_test.rs`](../../tests/crate_structure/str_002_test.rs)
/// (row 10, `test_error_module`).
#[doc(hidden)]
pub const STR_002_MODULE_PRESENT: () = ();
