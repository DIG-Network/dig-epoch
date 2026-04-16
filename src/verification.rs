//! # `verification` — root-level verification free functions
//!
//! **Introduced by:** `STR-002` — Module hierarchy (SPEC §13).
//!
//! **Future owners:** the `verification` domain (VER-*) — functions for
//! computing per-epoch block Merkle roots, inclusion proofs, withdrawals
//! Merkle-set roots, and checkpoint signing material:
//!
//! - `compute_epoch_block_root()`
//! - `epoch_block_inclusion_proof()`
//! - `compute_epoch_withdrawals_root()`
//! - checkpoint signing material computation (uses `chia-sdk-signer`
//!   domain-separation bytes)
//!
//! See [`SPEC.md` §13](../../docs/resources/SPEC.md) for the canonical
//! function list and [`SPEC.md` §7](../../docs/resources/SPEC.md) for the
//! verification algorithms.
//!
//! ## Relationship to `types/verification.rs`
//!
//! STR-002's Implementation Notes make the split explicit:
//!
//! - **This module** (`src/verification.rs`) holds **free functions** that
//!   depend on the Chia crates (`chia-consensus::compute_merkle_set_root`,
//!   `chia-sdk-types::MerkleTree`, `chia-sha2`, etc.). Start.md Hard
//!   Requirements 5 and 6 forbid us from re-implementing those algorithms.
//! - [`crate::types::verification`] holds the **data types** those
//!   functions produce / consume (`EpochCheckpointData`, `EpochBlockLink`).
//!
//! Both modules must exist simultaneously with distinct responsibilities;
//! the STR-002 integration test exercises each path separately.
//!
//! ## Status at STR-002
//!
//! Empty aside from the [`STR_002_MODULE_PRESENT`] sentinel.

/// Sentinel marker proving the module exists and is reachable at
/// `dig_epoch::verification::STR_002_MODULE_PRESENT`.
///
/// Exercised by the STR-002 integration test — see
/// [`tests/crate_structure/str_002_test.rs`](../../tests/crate_structure/str_002_test.rs)
/// (row 8, `test_verification_module`).
#[doc(hidden)]
pub const STR_002_MODULE_PRESENT: () = ();
