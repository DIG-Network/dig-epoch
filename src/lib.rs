//! # dig-epoch
//!
//! Epoch geometry, phase machine, manager, and checkpoint-competition types
//! for the DIG L2.
//!
//! ## Status of this file
//!
//! The crate was first stubbed by
//! [`STR-001`](../docs/requirements/domains/crate_structure/specs/STR-001.md)
//! (Cargo.toml dependency set only). The module tree below is introduced
//! by [`STR-002`](../docs/requirements/domains/crate_structure/specs/STR-002.md)
//! per [`SPEC.md § 13`](../docs/resources/SPEC.md) — each module is a stub
//! whose real contents arrive in the per-domain requirements listed in
//! [`IMPLEMENTATION_ORDER.md`](../docs/requirements/IMPLEMENTATION_ORDER.md).
//!
//! ## Philosophy
//!
//! `dig-epoch` is intentionally thin: all primitive types (blocks,
//! checkpoints, hashes, BLS signatures, Merkle roots) come from the shared
//! DIG / Chia ecosystem crates listed in `Cargo.toml`. This crate
//! contributes the epoch-level *structure* (phases, arithmetic, reward
//! splits, the checkpoint competition) that sits above those primitives.
//!
//! See [`docs/resources/SPEC.md`](../docs/resources/SPEC.md) for the
//! authoritative crate specification.
//!
//! ## Module map (STR-002)
//!
//! The crate exposes a flat module tree with a single `types/` subdirectory:
//!
//! | Module              | Future owner        | Responsibility                               |
//! |---------------------|---------------------|----------------------------------------------|
//! | [`constants`]       | CON-001 … CON-006   | Compile-time epoch constants                 |
//! | [`arithmetic`]      | HEA-001 … HEA-005   | Pure height-epoch mapping functions          |
//! | [`phase`]           | PHS-001 … PHS-004   | L1-progress phase calculation                |
//! | [`rewards`]         | REW-001 … REW-006   | Block-reward / distribution computation      |
//! | [`manager`]         | STR-004, MGR-001…   | `EpochManager` struct + methods              |
//! | [`verification`]    | VER-*               | Root-level verification free functions       |
//! | [`dfsp`]            | DFS-*               | Root-level DFSP processing free functions    |
//! | [`error`]           | ERR-001 … ERR-003   | `EpochError`, `CheckpointCompetitionError`   |
//! | [`types`]           | TYP-*, CKP-*, REW-4 | Data types (see `types/mod.rs` for submods)  |
//!
//! Every module declared here is currently empty apart from a
//! `#[doc(hidden)]` `STR_002_MODULE_PRESENT` sentinel; real content arrives
//! one requirement at a time per `docs/requirements/IMPLEMENTATION_ORDER.md`.
//!
//! ## Re-exports
//!
//! None yet. Public re-exports arrive in
//! [`STR-003`](../docs/requirements/domains/crate_structure/specs/STR-003.md),
//! after the module hierarchy is in place. Consumers of STR-002 access
//! items via their full path (e.g. `dig_epoch::constants::FOO`); the
//! convenience `use dig_epoch::*` idiom becomes available only at STR-003.

// `deny(missing_docs)` would be ideal for a public API crate, but turning it
// on right now would force boilerplate docs on every subsequent requirement.
// We instead enable the softer `warn` so that CI surfaces missing docs without
// breaking the build while the module tree is still being fleshed out. This
// is consistent with the rest of the DIG/Chia ecosystem where crate-level
// docs are gated to the library target.
#![warn(missing_docs)]

// -----------------------------------------------------------------------------
// Module declarations — STR-002 (SPEC §13)
// -----------------------------------------------------------------------------
//
// Ordering rationale: the modules are declared in **responsibility order**
// rather than alphabetical order so that a reader scanning `lib.rs` sees
// the crate's data-flow narrative:
//
//   constants  → primitive compile-time values
//   types      → data shapes that operate over those values
//   arithmetic → pure height/epoch math on those shapes
//   phase      → state machine derived from arithmetic
//   rewards    → economic derivations (uses arithmetic + constants)
//   manager    → stateful orchestrator that wires everything together
//   verification, dfsp — cryptographic / protocol layers built on the
//                        stateful manager
//   error      — last so the error enum can reference the other modules
//                without forward references
//
// Per STR-002, these are declarations only — no `pub use` re-exports yet.
// Re-exports are the subject of STR-003, and adding them here would
// foreshadow that requirement.

/// Compile-time epoch constants. Filled by the `constants` domain (CON-*).
pub mod constants;

/// Data-type definitions for epochs, phases, events, rewards, and checkpoints.
/// Filled by the `epoch_types` domain (TYP-*) and adjacent type-owning
/// requirements (CKP-001 for the competition types, REW-004 for
/// `RewardDistribution`).
pub mod types;

/// Pure height-to-epoch arithmetic. Filled by the `height_epoch_arithmetic`
/// domain (HEA-*).
pub mod arithmetic;

/// L1-progress phase calculation. Filled by the `phase_state_machine`
/// domain (PHS-*).
pub mod phase;

/// Block-reward and distribution computation. Filled by the
/// `reward_economics` domain (REW-*).
pub mod rewards;

/// `EpochManager` struct and methods. Filled by STR-004 (constructor) and
/// the `epoch_manager` domain (MGR-*).
pub mod manager;

/// Root-level verification free functions (epoch block root, inclusion
/// proofs, checkpoint signing material). Filled by the `verification`
/// domain (VER-*). **Note:** distinct from `types::verification`, which
/// hosts the data types these functions produce/consume.
pub mod verification;

/// Root-level DFSP processing free functions (burn policy, digests,
/// rollup, tail roots, activation control). Filled by the `dfsp_processing`
/// domain (DFS-*). **Note:** distinct from `types::dfsp`, which hosts the
/// seven DFSP data types from SPEC §3.14.
pub mod dfsp;

/// Crate-wide error enums (`EpochError`, `CheckpointCompetitionError`).
/// Filled by the `error_types` domain (ERR-*).
pub mod error;

/// Reusable test fixtures (STR-005). Exposed as a public module so
/// integration tests under `tests/` can use them. Not for production use —
/// synthetic signatures and public keys do not verify cryptographically.
pub mod test_helpers;

// -----------------------------------------------------------------------------
// STR-003 — flat public re-exports
// -----------------------------------------------------------------------------
//
// Scope: every symbol reachable at `dig_epoch::<name>` corresponds to an
// implemented requirement. DFSP processing (DFS-*) is deferred per
// IMPLEMENTATION_ORDER.md, so its free functions are not re-exported here;
// the `DfspCloseSnapshot` type (TYP-004) is exposed because it's referenced
// by MGR-006 / the EpochInfo surface.

// -- Manager (MGR-001, STR-004) ----------------------------------------------
pub use manager::EpochManager;

// -- Data types (TYP-*, CKP-001, REW-007, VER-004) ---------------------------
pub use types::{
    CheckpointCompetition, CompetitionStatus, DfspCloseSnapshot, EpochBlockLink,
    EpochCheckpointData, EpochCheckpointSignMaterial, EpochEvent, EpochInfo, EpochPhase,
    EpochStats, EpochSummary, PhaseTransition, RewardDistribution,
};

// -- Constants (CON-001..006) -------------------------------------------------
pub use constants::*;

// -- Height-epoch arithmetic (HEA-001..007) ----------------------------------
pub use arithmetic::{
    ensure_checkpoint_block_empty, epoch_checkpoint_height, epoch_for_block_height,
    first_height_in_epoch, is_checkpoint_class_block, is_epoch_checkpoint_block,
    is_first_block_after_epoch_checkpoint, is_genesis_checkpoint_block, l1_range_for_epoch,
    last_committed_height_in_epoch,
};

// -- Phase machine (PHS-001) -------------------------------------------------
pub use phase::l1_progress_phase_for_network_epoch;

// -- Reward economics (REW-001..005) -----------------------------------------
pub use rewards::{
    block_reward_at_height, burned_fee_remainder, compute_reward_distribution,
    epoch_reward_with_floor, proposer_fee_share, total_block_reward,
};

// -- Verification (VER-001..005) ---------------------------------------------
pub use verification::{
    compute_epoch_block_root, compute_epoch_withdrawals_root, epoch_block_inclusion_proof,
    epoch_checkpoint_sign_material_from_l2_blocks,
    stored_checkpoint_from_epoch_sign_material_with_aggregate_v1, verify_block_inclusion_proof,
};

// -- Errors (ERR-001..003) ---------------------------------------------------
pub use error::{CheckpointCompetitionError, EpochError};
