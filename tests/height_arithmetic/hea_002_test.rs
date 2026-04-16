/// HEA-002 — first_height_in_epoch and epoch_checkpoint_height
///
/// Normative: docs/requirements/domains/height_arithmetic/NORMATIVE.md §HEA-002
/// Spec ref:  docs/resources/SPEC.md §5.1
use dig_epoch::arithmetic::{
    epoch_checkpoint_height, epoch_for_block_height, first_height_in_epoch,
};
use dig_epoch::constants::BLOCKS_PER_EPOCH;

#[test]
fn test_first_height_epoch_0() {
    assert_eq!(first_height_in_epoch(0), 1);
}

#[test]
fn test_first_height_epoch_1() {
    assert_eq!(first_height_in_epoch(1), 33);
}

#[test]
fn test_first_height_epoch_2() {
    assert_eq!(first_height_in_epoch(2), 65);
}

#[test]
fn test_checkpoint_height_epoch_0() {
    assert_eq!(epoch_checkpoint_height(0), 32);
}

#[test]
fn test_checkpoint_height_epoch_1() {
    assert_eq!(epoch_checkpoint_height(1), 64);
}

#[test]
fn test_checkpoint_height_epoch_2() {
    assert_eq!(epoch_checkpoint_height(2), 96);
}

/// first_height maps back to its epoch.
#[test]
fn test_inverse_first_height() {
    for e in 0..100 {
        assert_eq!(epoch_for_block_height(first_height_in_epoch(e)), e, "e={e}");
    }
}

/// checkpoint height maps back to its epoch.
#[test]
fn test_inverse_checkpoint_height() {
    for e in 0..100 {
        assert_eq!(
            epoch_for_block_height(epoch_checkpoint_height(e)),
            e,
            "e={e}"
        );
    }
}

/// Each epoch spans exactly BLOCKS_PER_EPOCH heights.
#[test]
fn test_epoch_range_size() {
    for e in 0..10 {
        let size = epoch_checkpoint_height(e) - first_height_in_epoch(e) + 1;
        assert_eq!(size, BLOCKS_PER_EPOCH, "e={e}");
    }
}
