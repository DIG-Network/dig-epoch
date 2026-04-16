//! # `types::checkpoint_competition` — checkpoint-competition types
//!
//! **Introduced by:** `STR-002` — Module hierarchy (SPEC §13).
//!
//! **Future owners:** Phase 8 of `IMPLEMENTATION_ORDER.md` — the
//! `checkpoint_competition` domain:
//!
//! - [`CKP-001`](../../../docs/requirements/domains/checkpoint_competition/specs/CKP-001.md)
//!   — `CheckpointCompetition` struct and `CompetitionStatus` enum
//! - CKP-002 … CKP-N — lifecycle methods (`start_checkpoint_competition`,
//!   `submit_checkpoint`, scoring comparisons, finalisation)
//!
//! **Spec reference:**
//! [`SPEC.md` §8](../../../docs/resources/SPEC.md) — checkpoint competition.
//!
//! ## Content rule
//!
//! Per start.md Hard Requirement 1, this module MUST NOT redefine block
//! types — the canonical `Checkpoint` and `CheckpointSubmission` types
//! come from [`dig_block`]. This module hosts the dig-epoch-level
//! *competition* wrapper that selects a winning submission and tracks
//! its status through the epoch lifecycle.
//!
//! ## Status at STR-002
//!
//! Empty aside from the [`STR_002_MODULE_PRESENT`] sentinel.

/// Sentinel marker proving the module exists and is reachable at
/// `dig_epoch::types::checkpoint_competition::STR_002_MODULE_PRESENT`.
///
/// Exercised by the STR-002 integration test — see
/// [`tests/crate_structure/test_module_hierarchy.rs`](../../../tests/crate_structure/test_module_hierarchy.rs).
#[doc(hidden)]
pub const STR_002_MODULE_PRESENT: () = ();
