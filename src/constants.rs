//! # `constants` — compile-time epoch constants
//!
//! **Introduced by:** `STR-002` — Module hierarchy (SPEC §13).
//!
//! **Future owners:** This module is the landing zone for Phase 1 of
//! `IMPLEMENTATION_ORDER.md`:
//!
//! - [`CON-001`](../../docs/requirements/domains/constants/specs/CON-001.md)
//!   — epoch geometry (`BLOCKS_PER_EPOCH`, `EPOCH_L1_BLOCKS`, `GENESIS_HEIGHT`)
//! - [`CON-002`](../../docs/requirements/domains/constants/specs/CON-002.md)
//!   — phase boundary thresholds (50 % / 75 % / 100 %)
//! - [`CON-003`](../../docs/requirements/domains/constants/specs/CON-003.md)
//!   — reward economics (`MOJOS_PER_L2`, `INITIAL_BLOCK_REWARD`, halvings,
//!   tail emission, epoch-first-block bonus)
//! - [`CON-004`](../../docs/requirements/domains/constants/specs/CON-004.md)
//!   — 5-role fee / reward split percentages
//! - [`CON-005`](../../docs/requirements/domains/constants/specs/CON-005.md)
//!   — DFSP, consensus, slashing, and withdrawal constants
//! - [`CON-006`](../../docs/requirements/domains/constants/specs/CON-006.md)
//!   — sentinel constants (`EMPTY_ROOT`)
//!
//! **Spec reference:**
//! [`SPEC.md` §13](../../docs/resources/SPEC.md) — canonical module list.
//!
//! ## Content rule
//!
//! Per the STR-002 responsibility table, this module holds **only**
//! `pub const` declarations. It must never grow a function, enum, struct,
//! or `use` import that has runtime semantics; cross-cutting concerns such
//! as the genesis challenge come from `dig-constants` and are re-exported
//! through `lib.rs`, not re-defined here.
//!
//! ## Status at STR-002
//!
//! Intentionally empty aside from the [`STR_002_MODULE_PRESENT`] sentinel.
//! The sentinel exists so integration tests can prove the module is wired
//! into the crate without foreshadowing the real constants. It has zero
//! runtime footprint and is safe to keep once CON-* requirements land.

/// Sentinel marker proving the module exists, is declared in `lib.rs`, and
/// is reachable from an external crate at its canonical path.
///
/// See the STR-002 integration test at
/// [`tests/crate_structure/str_002_test.rs`](../../tests/crate_structure/str_002_test.rs).
///
/// The value is a zero-sized `()` constant; its only purpose is to let
/// `use dig_epoch::constants::STR_002_MODULE_PRESENT` succeed. No production
/// code should depend on it — later requirements are free to add real
/// `pub const` items alongside it without coordination.
#[doc(hidden)]
pub const STR_002_MODULE_PRESENT: () = ();
