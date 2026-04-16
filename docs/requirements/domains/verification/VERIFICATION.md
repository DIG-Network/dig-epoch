# Verification - Verification Matrix

> **Domain:** verification
> **Prefix:** VER
> **Normative:** [NORMATIVE.md](./NORMATIVE.md)
> **Tracking:** [TRACKING.yaml](./TRACKING.yaml)

| ID | Status | Summary | Verification Approach |
|----|--------|---------|----------------------|
| VER-001 | gap | Epoch Block Root | Unit test: compute_epoch_block_root with known block hashes in epoch order, verify Merkle root matches chia-sdk-types::MerkleTree. Verify EMPTY_ROOT returned for empty slice. Verify determinism: same inputs always produce same output. Verify order matters: different orderings produce different roots. |
| VER-002 | gap | Block Inclusion Proof | Unit test: generate epoch_block_inclusion_proof for valid index, verify proof is verifiable against compute_epoch_block_root result. Verify None returned for out-of-bounds index. Verify proof for first, last, and middle blocks. |
| VER-003 | gap | Epoch Withdrawals Root | Unit test: compute_epoch_withdrawals_root with known withdrawal hashes, verify Merkle set root matches chia-consensus::compute_merkle_set_root(). Verify EMPTY_ROOT for empty slice. Verify order-independence: same hashes in different order produce identical root. Verify tagged hashing (0x01 leaf, 0x02 node). |
| VER-004 | gap | Checkpoint Data and Sign Material | Unit test: construct EpochCheckpointData with all 6 fields (network_id as Bytes32), verify accessors. Construct EpochCheckpointSignMaterial with checkpoint + score + signing_digest. Call epoch_checkpoint_sign_material_from_l2_blocks() with test blocks, verify block_root, state_root, withdrawals_root, checkpoint_hash, and signing_digest are correctly derived. |
| VER-005 | gap | Aggregate Signature Construction | Unit test: generate multiple BLS key pairs and signatures over a signing digest, call stored_checkpoint_from_epoch_sign_material_with_aggregate_v1(material, validator_set, per_validator, submitter). Verify aggregate signature via chia-bls::aggregate(). Verify aggregate public key. Verify CheckpointSubmission score. Verify signer bitmap computed from validator_set and per_validator. Verify error on empty per_validator. |
