//! # `types::reward` — `RewardDistribution` struct
//!
//! **Introduced by:** `STR-002` — Module hierarchy (SPEC §13).
//!
//! **Future owner:**
//! [`REW-004`](../../../docs/requirements/domains/reward_economics/specs/REW-004.md)
//! — the `RewardDistribution` struct, the data type returned by
//! `compute_reward_distribution()` in [`crate::rewards`]. Tracks the
//! 5-role split (proposer, farmers, storage providers, treasury, burn).
//!
//! **Spec reference:** [`SPEC.md` §5](../../../docs/resources/SPEC.md) —
//! reward economics.
//!
//! ## Status at STR-002
//!
//! Empty aside from the [`STR_002_MODULE_PRESENT`] sentinel.

/// Sentinel marker proving the module exists and is reachable at
/// `dig_epoch::types::reward::STR_002_MODULE_PRESENT`.
///
/// Exercised by the STR-002 integration test — see
/// [`tests/crate_structure/str_002_test.rs`](../../../tests/crate_structure/str_002_test.rs).
#[doc(hidden)]
pub const STR_002_MODULE_PRESENT: () = ();
