# Verification - Normative Requirements

> **Domain:** verification
> **Prefix:** VER
> **Spec reference:** [SPEC.md - Sections 7.1-7.6](../../../resources/SPEC.md)

## Requirements

### VER-001: Epoch Block Root

compute_epoch_block_root(block_hashes: &[Bytes32]) -> Bytes32 MUST compute a Merkle root via chia-sdk-types::MerkleTree over all block hashes in epoch order. MUST return EMPTY_ROOT for an empty slice. The block hashes MUST be ordered by their position within the epoch (block production order). Given the same ordered set of block hashes, the function MUST always produce the same Bytes32.

**Spec reference:** SPEC Section 7.1

### VER-002: Block Inclusion Proof

epoch_block_inclusion_proof(block_hashes: &[Bytes32], index: usize) -> Option<MerkleProof> MUST generate a MerkleProof for the block at the given index within the epoch's block hash list. MUST return None if index is out of bounds. The returned proof MUST be verifiable against the epoch block root computed by VER-001. The proof enables any party to confirm that a specific block was included in a given epoch without requiring the full block list.

**Spec reference:** SPEC Section 7.2

### VER-003: Epoch Withdrawals Root

compute_epoch_withdrawals_root(withdrawal_hashes: &[Bytes32]) -> Bytes32 MUST compute an order-independent Merkle set root via chia-consensus::compute_merkle_set_root(). MUST return EMPTY_ROOT for an empty slice. Uses tagged hashing (same as dig-block: leaf = SHA-256(0x01 || data), node = SHA-256(0x02 || left || right)). The result is independent of the input ordering of withdrawal hashes.

**Spec reference:** SPEC Section 7.3

### VER-004: Checkpoint Data and Sign Material

EpochCheckpointData MUST contain the following fields:
- `network_id: Bytes32` - the network identifier
- `epoch: u64` - the epoch number
- `block_root: Bytes32` - Merkle root over epoch block hashes (VER-001)
- `state_root: Bytes32` - CoinSet state root at epoch close
- `withdrawals_root: Bytes32` - Merkle set root over epoch withdrawals (VER-003)
- `checkpoint_hash: Bytes32` - the hash of the Checkpoint struct

EpochCheckpointSignMaterial MUST contain the following fields:
- `checkpoint: EpochCheckpointData` - the checkpoint data
- `score: u64` - computed checkpoint score
- `signing_digest: Bytes32` - the digest to be signed by validators

epoch_checkpoint_sign_material_from_l2_blocks() MUST construct EpochCheckpointSignMaterial from actual L2 blocks, computing block_root, state_root, withdrawals_root, checkpoint_hash, and signing_digest from the block data.

**Spec reference:** SPEC Sections 7.4, 7.5

### VER-005: Aggregate Signature Construction

stored_checkpoint_from_epoch_sign_material_with_aggregate_v1(material: &EpochCheckpointSignMaterial, validator_set: &[(u32, PublicKey)], per_validator: &[(u32, PublicKey, Signature)], submitter: u32) -> Result<CheckpointSubmission, EpochError> MUST: aggregate all provided per-validator signatures via chia-bls::aggregate(), aggregate the corresponding public keys from the validator set, and construct a CheckpointSubmission with the computed score. The aggregate signature MUST be valid under the aggregate public key for the signing digest. The validator_set provides the full set of (index, pubkey) pairs, while per_validator provides (index, pubkey, signature) tuples for each signing validator.

**Spec reference:** SPEC Section 7.6
