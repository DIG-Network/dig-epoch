/// HEA-004 — l1_range_for_epoch
///
/// Normative: docs/requirements/domains/height_arithmetic/NORMATIVE.md §HEA-004
/// Spec ref:  docs/resources/SPEC.md §5.4
use dig_epoch::arithmetic::l1_range_for_epoch;
use dig_epoch::constants::EPOCH_L1_BLOCKS;

#[test]
fn test_l1_range_epoch_0() {
    assert_eq!(l1_range_for_epoch(100, 0), (100, 131));
}

#[test]
fn test_l1_range_epoch_1() {
    assert_eq!(l1_range_for_epoch(100, 1), (132, 163));
}

#[test]
fn test_l1_range_epoch_2() {
    assert_eq!(l1_range_for_epoch(100, 2), (164, 195));
}

/// Every epoch's L1 window is exactly EPOCH_L1_BLOCKS wide.
#[test]
fn test_l1_range_width() {
    for e in 0..10 {
        let (start, end) = l1_range_for_epoch(100, e);
        assert_eq!(end - start + 1, EPOCH_L1_BLOCKS, "e={e}");
    }
}

/// Consecutive epoch L1 windows are contiguous (no gaps or overlaps).
#[test]
fn test_l1_range_contiguous() {
    for e in 0..10 {
        let (_, end) = l1_range_for_epoch(100, e);
        let (next_start, _) = l1_range_for_epoch(100, e + 1);
        assert_eq!(end + 1, next_start, "e={e}");
    }
}

/// genesis_l1_height=0, epoch=0 starts at 0.
#[test]
fn test_l1_range_genesis_0() {
    assert_eq!(l1_range_for_epoch(0, 0), (0, 31));
}

/// Large epoch number.
#[test]
fn test_l1_range_large_epoch() {
    assert_eq!(l1_range_for_epoch(100, 1000), (32100, 32131));
}
