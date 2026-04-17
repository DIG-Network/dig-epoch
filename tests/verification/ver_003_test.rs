/// VER-003 — compute_epoch_withdrawals_root (order-independent).
///
/// Normative: docs/requirements/domains/verification/NORMATIVE.md §VER-003
/// Spec ref:  docs/resources/SPEC.md §7.3
use chia_protocol::Bytes32;
use dig_epoch::constants::EMPTY_ROOT;
use dig_epoch::verification::compute_epoch_withdrawals_root;

fn h(x: u8) -> Bytes32 {
    Bytes32::new([x; 32])
}

#[test]
fn test_empty_returns_empty_root() {
    assert_eq!(compute_epoch_withdrawals_root(&[]), EMPTY_ROOT);
}

#[test]
fn test_single_is_non_empty_root() {
    let r = compute_epoch_withdrawals_root(&[h(1)]);
    assert_ne!(r, EMPTY_ROOT);
}

#[test]
fn test_multiple_is_non_empty_root() {
    let r = compute_epoch_withdrawals_root(&[h(1), h(2), h(3)]);
    assert_ne!(r, EMPTY_ROOT);
}

/// Order-independence: same set of hashes in different orders yields same root.
#[test]
fn test_order_independent() {
    let a = compute_epoch_withdrawals_root(&[h(1), h(2), h(3)]);
    let b = compute_epoch_withdrawals_root(&[h(3), h(1), h(2)]);
    let c = compute_epoch_withdrawals_root(&[h(2), h(3), h(1)]);
    assert_eq!(a, b);
    assert_eq!(a, c);
}

#[test]
fn test_deterministic() {
    let a = compute_epoch_withdrawals_root(&[h(1), h(2)]);
    let b = compute_epoch_withdrawals_root(&[h(1), h(2)]);
    assert_eq!(a, b);
}

#[test]
fn test_distinct_sets_differ() {
    let a = compute_epoch_withdrawals_root(&[h(1), h(2)]);
    let b = compute_epoch_withdrawals_root(&[h(3), h(4)]);
    assert_ne!(a, b);
}
