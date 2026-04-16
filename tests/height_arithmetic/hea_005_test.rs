/// HEA-005 — Height-epoch round-trip identity property
///
/// Normative: docs/requirements/domains/height_arithmetic/NORMATIVE.md §HEA-005
/// Spec ref:  docs/resources/SPEC.md §14.10
use dig_epoch::arithmetic::{
    epoch_checkpoint_height, epoch_for_block_height, first_height_in_epoch,
};
use dig_epoch::constants::BLOCKS_PER_EPOCH;

/// Small heights (1..=320, 10 epochs): every height falls within its epoch's range.
#[test]
fn test_round_trip_small_heights() {
    for h in 1..=320 {
        let e = epoch_for_block_height(h);
        assert!(first_height_in_epoch(e) <= h, "h={h}");
        assert!(h <= epoch_checkpoint_height(e), "h={h}");
    }
}

/// Medium heights (3201..=3520, epochs 100-109).
#[test]
fn test_round_trip_medium_heights() {
    for h in 3201..=3520 {
        let e = epoch_for_block_height(h);
        assert!(first_height_in_epoch(e) <= h, "h={h}");
        assert!(h <= epoch_checkpoint_height(e), "h={h}");
    }
}

/// Large heights (320001..=320320, epochs 10000-10009).
#[test]
fn test_round_trip_large_heights() {
    for h in 320_001..=320_320 {
        let e = epoch_for_block_height(h);
        assert!(first_height_in_epoch(e) <= h, "h={h}");
        assert!(h <= epoch_checkpoint_height(e), "h={h}");
    }
}

/// Boundary: first_height_in_epoch(e) maps back to e.
#[test]
fn test_round_trip_boundary_first() {
    for e in 0..100 {
        let h = first_height_in_epoch(e);
        assert_eq!(epoch_for_block_height(h), e, "e={e}");
    }
}

/// Boundary: epoch_checkpoint_height(e) maps back to e.
#[test]
fn test_round_trip_boundary_checkpoint() {
    for e in 0..100 {
        let h = epoch_checkpoint_height(e);
        assert_eq!(epoch_for_block_height(h), e, "e={e}");
    }
}

/// epoch_for_block_height(first_height_in_epoch(e)) == e for large e.
#[test]
fn test_epoch_inverse_first() {
    for e in 0..10_000 {
        assert_eq!(epoch_for_block_height(first_height_in_epoch(e)), e, "e={e}");
    }
}

/// epoch_for_block_height(epoch_checkpoint_height(e)) == e for large e.
#[test]
fn test_epoch_inverse_checkpoint() {
    for e in 0..10_000 {
        assert_eq!(
            epoch_for_block_height(epoch_checkpoint_height(e)),
            e,
            "e={e}"
        );
    }
}

/// epoch_checkpoint_height(e) == first_height_in_epoch(e) + BLOCKS_PER_EPOCH - 1.
#[test]
fn test_checkpoint_equals_first_plus_blocks() {
    for e in 0..10_000 {
        assert_eq!(
            epoch_checkpoint_height(e),
            first_height_in_epoch(e) + BLOCKS_PER_EPOCH - 1,
            "e={e}"
        );
    }
}
