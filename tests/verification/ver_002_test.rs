/// VER-002 — epoch_block_inclusion_proof + verification.
///
/// Normative: docs/requirements/domains/verification/NORMATIVE.md §VER-002
/// Spec ref:  docs/resources/SPEC.md §7.2
use chia_protocol::Bytes32;
use dig_epoch::verification::{
    compute_epoch_block_root, epoch_block_inclusion_proof, verify_block_inclusion_proof,
};

fn h(x: u8) -> Bytes32 {
    Bytes32::new([x; 32])
}

#[test]
fn test_out_of_bounds_returns_none() {
    let hashes = vec![h(1), h(2)];
    assert!(epoch_block_inclusion_proof(&hashes, 2).is_none());
    assert!(epoch_block_inclusion_proof(&hashes, 999).is_none());
}

#[test]
fn test_empty_returns_none() {
    assert!(epoch_block_inclusion_proof(&[], 0).is_none());
}

#[test]
fn test_single_block_proof_verifies() {
    let hashes = vec![h(7)];
    let root = compute_epoch_block_root(&hashes);
    let proof = epoch_block_inclusion_proof(&hashes, 0).unwrap();
    assert!(verify_block_inclusion_proof(hashes[0], &proof, root));
}

#[test]
fn test_first_block_proof_verifies() {
    let hashes = vec![h(1), h(2), h(3), h(4), h(5)];
    let root = compute_epoch_block_root(&hashes);
    let proof = epoch_block_inclusion_proof(&hashes, 0).unwrap();
    assert!(verify_block_inclusion_proof(hashes[0], &proof, root));
}

#[test]
fn test_last_block_proof_verifies() {
    let hashes = vec![h(1), h(2), h(3), h(4), h(5)];
    let root = compute_epoch_block_root(&hashes);
    let proof = epoch_block_inclusion_proof(&hashes, 4).unwrap();
    assert!(verify_block_inclusion_proof(hashes[4], &proof, root));
}

#[test]
fn test_middle_block_proof_verifies() {
    let hashes = vec![h(1), h(2), h(3), h(4), h(5)];
    let root = compute_epoch_block_root(&hashes);
    let proof = epoch_block_inclusion_proof(&hashes, 2).unwrap();
    assert!(verify_block_inclusion_proof(hashes[2], &proof, root));
}

#[test]
fn test_all_indices_verify() {
    let hashes = vec![h(1), h(2), h(3), h(4)];
    let root = compute_epoch_block_root(&hashes);
    for i in 0..hashes.len() {
        let proof = epoch_block_inclusion_proof(&hashes, i).unwrap();
        assert!(
            verify_block_inclusion_proof(hashes[i], &proof, root),
            "failed at index {i}"
        );
    }
}

#[test]
fn test_wrong_leaf_fails() {
    let hashes = vec![h(1), h(2), h(3), h(4)];
    let root = compute_epoch_block_root(&hashes);
    let proof = epoch_block_inclusion_proof(&hashes, 1).unwrap();
    // Verifying with the wrong leaf value should fail.
    assert!(!verify_block_inclusion_proof(h(99), &proof, root));
}
