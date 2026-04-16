//! # `types` — data-type definitions grouped by concern
//!
//! **Introduced by:** `STR-002` — Module hierarchy (SPEC §13).
//!
//! This module is the **single subdirectory** that STR-002 carves out of
//! the otherwise-flat dig-epoch tree. Its purpose is to separate the data
//! shapes the crate manipulates (structs / enums) from the free functions
//! that operate on them (which live at the crate root in `arithmetic.rs`,
//! `phase.rs`, `rewards.rs`, `verification.rs`, `dfsp.rs`, and `manager.rs`).
//!
//! ## Submodules
//!
//! Each submodule corresponds to a single data-type family and has exactly
//! one future-owner requirement family:
//!
//! | Submodule                  | Future owners                                   |
//! |----------------------------|-------------------------------------------------|
//! | [`epoch_phase`]            | TYP-001 — `EpochPhase`, `PhaseTransition`       |
//! | [`epoch_info`]             | TYP-002 — `EpochInfo`                           |
//! | [`epoch_summary`]          | TYP-003 — `EpochSummary`                        |
//! | [`dfsp`]                   | TYP-004 — 7 DFSP types (SPEC §3.14)             |
//! | [`events`]                 | TYP-005 — `EpochEvent`, `EpochStats`            |
//! | [`checkpoint_competition`] | CKP-001 — `CheckpointCompetition`, status enum  |
//! | [`reward`]                 | REW-004 — `RewardDistribution`                  |
//! | [`verification`]           | TYP-006 — `EpochBlockLink`, `EpochCheckpointData` |
//!
//! ## Re-export policy
//!
//! STR-002 only declares `pub mod <submodule>;` entries; the real
//! `pub use` re-exports (so consumers can write `dig_epoch::EpochPhase`)
//! land in [`STR-003`](../../docs/requirements/domains/crate_structure/specs/STR-003.md).
//! STR-002's integration test uses the fully-qualified path
//! `dig_epoch::types::<submodule>::STR_002_MODULE_PRESENT`, which is
//! exactly what STR-002 guarantees.
//!
//! ## Spec reference
//!
//! [`SPEC.md` §13](../../docs/resources/SPEC.md) — canonical module list.
//! [`STR-002.md`](../../docs/requirements/domains/crate_structure/specs/STR-002.md)
//! — per-submodule responsibility table.

pub mod checkpoint_competition;
pub mod dfsp;
pub mod epoch_info;
pub mod epoch_phase;
pub mod epoch_summary;
pub mod events;
pub mod reward;
pub mod verification;
