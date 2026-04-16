/// HEA-001 — epoch_for_block_height
///
/// Normative: docs/requirements/domains/height_arithmetic/NORMATIVE.md §HEA-001
/// Spec ref:  docs/resources/SPEC.md §5.1
use dig_epoch::arithmetic::epoch_for_block_height;
use dig_epoch::constants::BLOCKS_PER_EPOCH;

/// epoch_for_block_height(1) == 0 (genesis).
#[test]
fn test_genesis_height() {
    assert_eq!(epoch_for_block_height(1), 0);
}

/// Last block of epoch 0 (h=32) maps to epoch 0.
#[test]
fn test_last_block_epoch_0() {
    assert_eq!(epoch_for_block_height(32), 0);
}

/// First block of epoch 1 (h=33) maps to epoch 1.
#[test]
fn test_first_block_epoch_1() {
    assert_eq!(epoch_for_block_height(33), 1);
}

/// Last block of epoch 1 (h=64) maps to epoch 1.
#[test]
fn test_last_block_epoch_1() {
    assert_eq!(epoch_for_block_height(64), 1);
}

/// First block of epoch 2 (h=65) maps to epoch 2.
#[test]
fn test_first_block_epoch_2() {
    assert_eq!(epoch_for_block_height(65), 2);
}

/// Large height round-trip.
#[test]
fn test_large_height() {
    assert_eq!(
        epoch_for_block_height(1_000_000),
        (1_000_000 - 1) / BLOCKS_PER_EPOCH
    );
}

/// All heights 1..=32 map to epoch 0.
#[test]
fn test_all_heights_epoch_0() {
    for h in 1..=32 {
        assert_eq!(epoch_for_block_height(h), 0, "h={h}");
    }
}

/// All heights 33..=64 map to epoch 1.
#[test]
fn test_all_heights_epoch_1() {
    for h in 33..=64 {
        assert_eq!(epoch_for_block_height(h), 1, "h={h}");
    }
}
