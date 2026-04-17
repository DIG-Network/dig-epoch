/// VER-001 — compute_epoch_block_root (ordered Merkle root).
///
/// Normative: docs/requirements/domains/verification/NORMATIVE.md §VER-001
/// Spec ref:  docs/resources/SPEC.md §7.1
use chia_protocol::Bytes32;
use chia_sdk_types::MerkleTree;
use dig_epoch::constants::EMPTY_ROOT;
use dig_epoch::verification::compute_epoch_block_root;

fn h(x: u8) -> Bytes32 {
    Bytes32::new([x; 32])
}

#[test]
fn test_empty_returns_empty_root() {
    assert_eq!(compute_epoch_block_root(&[]), EMPTY_ROOT);
}

#[test]
fn test_single_matches_merkle_tree() {
    let leaves = vec![h(1)];
    let expected = MerkleTree::new(&leaves).root();
    assert_eq!(compute_epoch_block_root(&leaves), expected);
}

#[test]
fn test_multiple_matches_merkle_tree() {
    let leaves = vec![h(1), h(2), h(3), h(4)];
    let expected = MerkleTree::new(&leaves).root();
    assert_eq!(compute_epoch_block_root(&leaves), expected);
}

#[test]
fn test_deterministic() {
    let leaves = vec![h(1), h(2), h(3)];
    let a = compute_epoch_block_root(&leaves);
    let b = compute_epoch_block_root(&leaves);
    assert_eq!(a, b);
}

#[test]
fn test_order_sensitive() {
    let a = compute_epoch_block_root(&[h(1), h(2), h(3)]);
    let b = compute_epoch_block_root(&[h(3), h(2), h(1)]);
    assert_ne!(a, b);
}
