//! # `rewards` — block reward and distribution computation
//!
//! **Introduced by:** `STR-002` — Module hierarchy (SPEC §13).
//!
//! **Future owners:** Phase 6 of `IMPLEMENTATION_ORDER.md`:
//!
//! - [`REW-001`](../../docs/requirements/domains/reward_economics/specs/REW-001.md)
//!   — `block_reward_at_height()` with halving schedule
//! - [`REW-002`](../../docs/requirements/domains/reward_economics/specs/REW-002.md)
//!   — `total_block_reward()` with epoch-first-block bonus
//! - [`REW-003`](../../docs/requirements/domains/reward_economics/specs/REW-003.md)
//!   — `proposer_fee_share()` / `burned_fee_remainder()`
//! - [`REW-004`](../../docs/requirements/domains/reward_economics/specs/REW-004.md)
//!   — `compute_reward_distribution()` (5-role split)
//! - [`REW-005`](../../docs/requirements/domains/reward_economics/specs/REW-005.md)
//!   — tail emission floor
//! - [`REW-006`](../../docs/requirements/domains/reward_economics/specs/REW-006.md)
//!   — halving-interval boundary verification
//!
//! **Spec reference:**
//! [`SPEC.md` §13](../../docs/resources/SPEC.md) — canonical module list;
//! [`SPEC.md` §5](../../docs/resources/SPEC.md) — reward economics.
//!
//! ## Content rule
//!
//! All reward computations are **pure functions** of L1/L2 height and
//! compile-time constants. Per start.md Hard Requirement 8, reward
//! constants are compile-time; this module may never read runtime
//! configuration. The matching data type
//! ([`crate::types::reward::RewardDistribution`]) lives in `types/` as
//! mandated by the STR-002 split between types and free functions.
//!
//! ## Status at STR-002
//!
//! Empty aside from the [`STR_002_MODULE_PRESENT`] sentinel.

/// Sentinel marker proving the module exists and is reachable at
/// `dig_epoch::rewards::STR_002_MODULE_PRESENT`.
///
/// Exercised by the STR-002 integration test — see
/// [`tests/crate_structure/test_module_hierarchy.rs`](../../tests/crate_structure/test_module_hierarchy.rs).
#[doc(hidden)]
pub const STR_002_MODULE_PRESENT: () = ();
