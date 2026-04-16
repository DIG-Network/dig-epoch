/// HEA-006 — last_committed_height_in_epoch
///
/// Normative: docs/requirements/domains/height_arithmetic/NORMATIVE.md §HEA-006
/// Spec ref:  docs/resources/SPEC.md §5.5
use dig_epoch::arithmetic::{epoch_checkpoint_height, last_committed_height_in_epoch};

#[test]
fn test_tip_below_checkpoint() {
    assert_eq!(last_committed_height_in_epoch(0, 20), 20);
}

#[test]
fn test_tip_at_checkpoint() {
    assert_eq!(last_committed_height_in_epoch(0, 32), 32);
}

#[test]
fn test_tip_above_checkpoint() {
    assert_eq!(last_committed_height_in_epoch(0, 50), 32);
}

#[test]
fn test_epoch_1_capped() {
    assert_eq!(last_committed_height_in_epoch(1, 100), 64);
}

#[test]
fn test_epoch_1_below() {
    assert_eq!(last_committed_height_in_epoch(1, 40), 40);
}

#[test]
fn test_large_epoch() {
    let ckp = epoch_checkpoint_height(100);
    assert_eq!(last_committed_height_in_epoch(100, u64::MAX), ckp);
}
