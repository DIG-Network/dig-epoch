//! # `verification` — root-level verification free functions
//!
//! **Introduced by:** `STR-002` — Module hierarchy (SPEC §13).
//!
//! **Owners:** `VER-001..VER-005` — epoch block root, inclusion proofs,
//! withdrawals set root, checkpoint sign material, and aggregate-signature
//! construction.
//!
//! **Spec reference:** [`SPEC.md` §7](../../docs/resources/SPEC.md).
//!
//! All Merkle work is delegated to the ecosystem crates:
//! - Per-block ordered tree: [`chia_sdk_types::MerkleTree`]
//! - Per-withdrawal order-independent set: [`chia_consensus::merkle_set::compute_merkle_set_root`]
//! - Hashing: [`chia_sha2::Sha256`]
//! - BLS aggregation: [`chia_bls::aggregate`] / [`chia_bls::aggregate_verify`]

/// Sentinel marker proving the module exists and is reachable at
/// `dig_epoch::verification::STR_002_MODULE_PRESENT`.
#[doc(hidden)]
pub const STR_002_MODULE_PRESENT: () = ();

use chia_bls::{aggregate, PublicKey, Signature};
use chia_consensus::merkle_set::compute_merkle_set_root;
use chia_protocol::Bytes32;
use chia_sdk_types::{MerkleProof, MerkleTree};
use chia_sha2::Sha256;
use dig_block::{Checkpoint, CheckpointSubmission, SignerBitmap};

use crate::constants::EMPTY_ROOT;
use crate::error::EpochError;
use crate::types::verification::{EpochCheckpointData, EpochCheckpointSignMaterial};

// -----------------------------------------------------------------------------
// VER-001 — compute_epoch_block_root
// -----------------------------------------------------------------------------

/// Canonical ordered Merkle root over an epoch's block hashes.
///
/// Returns [`EMPTY_ROOT`] when the slice is empty. Otherwise delegates to
/// [`MerkleTree::new`] and returns its root. The hashing scheme is the
/// tagged leaf/node scheme from `chia-sdk-types` (see module docs).
pub fn compute_epoch_block_root(block_hashes: &[Bytes32]) -> Bytes32 {
    if block_hashes.is_empty() {
        return EMPTY_ROOT;
    }
    MerkleTree::new(block_hashes).root()
}

// -----------------------------------------------------------------------------
// VER-002 — epoch_block_inclusion_proof
// -----------------------------------------------------------------------------

/// Merkle inclusion proof for the block at `index` within `block_hashes`.
///
/// Returns `None` when `index` is out of bounds or the slice is empty.
pub fn epoch_block_inclusion_proof(block_hashes: &[Bytes32], index: usize) -> Option<MerkleProof> {
    if index >= block_hashes.len() {
        return None;
    }
    let tree = MerkleTree::new(block_hashes);
    tree.proof(block_hashes[index])
}

/// Recomputes the Merkle root by walking a leaf-keyed proof against `leaf`.
///
/// Uses the same tagged hashing as [`MerkleTree`]: leaf = `SHA-256(0x01 || leaf)`,
/// node = `SHA-256(0x02 || left || right)`. The `path` is the bit-indexed
/// sibling position (bit 0 == sibling on the right at level 0, etc).
pub fn verify_block_inclusion_proof(leaf: Bytes32, proof: &MerkleProof, root: Bytes32) -> bool {
    let mut current = {
        let mut h = Sha256::new();
        h.update([0x01u8]);
        h.update(leaf.as_ref());
        Bytes32::new(h.finalize())
    };
    for (level, sibling) in proof.proof.iter().enumerate() {
        let bit_is_right = (proof.path >> level) & 1 == 1;
        let mut h = Sha256::new();
        h.update([0x02u8]);
        if bit_is_right {
            // `self` is on the right, sibling on the left.
            h.update(sibling.as_ref());
            h.update(current.as_ref());
        } else {
            h.update(current.as_ref());
            h.update(sibling.as_ref());
        }
        current = Bytes32::new(h.finalize());
    }
    current == root
}

// -----------------------------------------------------------------------------
// VER-003 — compute_epoch_withdrawals_root
// -----------------------------------------------------------------------------

/// Order-independent Merkle set root over an epoch's withdrawal hashes.
///
/// Returns [`EMPTY_ROOT`] when the slice is empty. Otherwise delegates to
/// [`compute_merkle_set_root`] from `chia-consensus`.
pub fn compute_epoch_withdrawals_root(withdrawal_hashes: &[Bytes32]) -> Bytes32 {
    if withdrawal_hashes.is_empty() {
        return EMPTY_ROOT;
    }
    let mut leaves: Vec<[u8; 32]> = withdrawal_hashes.iter().map(|h| h.to_bytes()).collect();
    Bytes32::new(compute_merkle_set_root(&mut leaves))
}

// -----------------------------------------------------------------------------
// VER-004 — checkpoint sign material helpers
// -----------------------------------------------------------------------------

/// Builds [`EpochCheckpointSignMaterial`] from an L2-epoch worth of inputs.
///
/// - `block_root` is computed via [`compute_epoch_block_root`] over `block_hashes`.
/// - `withdrawals_root` is computed via [`compute_epoch_withdrawals_root`].
/// - `checkpoint_hash` is derived from a synthesized `Checkpoint` struct.
/// - `signing_digest` is the hash of `(network_id || epoch_le || block_root ||
///   state_root || withdrawals_root || checkpoint_hash)`.
/// - `score` is `stake_percentage * block_count` per CKP-004.
#[allow(clippy::too_many_arguments)]
pub fn epoch_checkpoint_sign_material_from_l2_blocks(
    network_id: Bytes32,
    epoch: u64,
    block_hashes: &[Bytes32],
    state_root: Bytes32,
    withdrawal_hashes: &[Bytes32],
    prev_checkpoint: Bytes32,
    total_fees: u64,
    tx_count: u64,
    stake_percentage: u64,
) -> EpochCheckpointSignMaterial {
    let block_root = compute_epoch_block_root(block_hashes);
    let withdrawals_root = compute_epoch_withdrawals_root(withdrawal_hashes);

    let mut checkpoint = Checkpoint::new();
    checkpoint.epoch = epoch;
    checkpoint.state_root = state_root;
    checkpoint.block_root = block_root;
    checkpoint.block_count = block_hashes.len() as u32;
    checkpoint.tx_count = tx_count;
    checkpoint.total_fees = total_fees;
    checkpoint.prev_checkpoint = prev_checkpoint;
    checkpoint.withdrawals_root = withdrawals_root;
    checkpoint.withdrawal_count = withdrawal_hashes.len() as u32;

    let checkpoint_hash = checkpoint.hash();
    let score = checkpoint.compute_score(stake_percentage);

    let data = EpochCheckpointData {
        network_id,
        epoch,
        block_root,
        state_root,
        withdrawals_root,
        checkpoint_hash,
    };
    let signing_digest = data.signing_digest();

    EpochCheckpointSignMaterial {
        checkpoint: data,
        score,
        signing_digest,
    }
}

// -----------------------------------------------------------------------------
// VER-005 — aggregate signature construction
// -----------------------------------------------------------------------------

/// Constructs a [`CheckpointSubmission`] with an aggregated BLS signature.
///
/// - `validator_set` is the complete active validator set `(index, pubkey)`.
/// - `per_validator` is `(index, pubkey, signature)` for each signer.
/// - Signer bitmap is derived from comparing the two slices.
/// - Aggregate signature uses [`chia_bls::aggregate`].
pub fn stored_checkpoint_from_epoch_sign_material_with_aggregate_v1(
    material: &EpochCheckpointSignMaterial,
    validator_set: &[(u32, PublicKey)],
    per_validator: &[(u32, PublicKey, Signature)],
    submitter: u32,
) -> Result<CheckpointSubmission, EpochError> {
    if per_validator.is_empty() {
        return Err(EpochError::DfspBoundary(
            "aggregate signature requires at least one signer".to_string(),
        ));
    }

    // Aggregate signatures.
    let sigs: Vec<Signature> = per_validator.iter().map(|(_, _, s)| s.clone()).collect();
    let agg_sig = aggregate(&sigs);

    // Aggregate public keys by summing (chia-bls PublicKey supports addition).
    let mut agg_pk = PublicKey::default();
    for (_, pk, _) in per_validator {
        agg_pk += pk;
    }

    // Build the signer bitmap: one bit per validator in `validator_set`,
    // set if that validator's index appears in `per_validator`.
    let mut bitmap = SignerBitmap::new(validator_set.len() as u32);
    for (vi, (idx, _)) in validator_set.iter().enumerate() {
        if per_validator.iter().any(|(i, _, _)| i == idx) {
            bitmap
                .set_signed(vi as u32)
                .map_err(|e| EpochError::DfspBoundary(format!("set_signed: {e}")))?;
        }
    }

    // Reconstruct a minimal Checkpoint for the submission (block_root/state_root/
    // withdrawals_root are the only fields required for hash-equivalence with
    // the sign material).
    let mut cp = Checkpoint::new();
    cp.epoch = material.checkpoint.epoch;
    cp.state_root = material.checkpoint.state_root;
    cp.block_root = material.checkpoint.block_root;
    cp.withdrawals_root = material.checkpoint.withdrawals_root;

    Ok(CheckpointSubmission::new(
        cp,
        bitmap,
        agg_sig,
        agg_pk,
        material.score,
        submitter,
    ))
}
