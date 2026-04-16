/// HEA-007 — is_first_block_after_epoch_checkpoint
///
/// Normative: docs/requirements/domains/height_arithmetic/NORMATIVE.md §HEA-007
/// Spec ref:  docs/resources/SPEC.md §5.2
use dig_epoch::arithmetic::is_first_block_after_epoch_checkpoint;

#[test]
fn test_genesis_not_first_after() {
    assert!(!is_first_block_after_epoch_checkpoint(1));
}

#[test]
fn test_normal_block() {
    assert!(!is_first_block_after_epoch_checkpoint(2));
}

#[test]
fn test_checkpoint_block() {
    assert!(!is_first_block_after_epoch_checkpoint(32));
}

#[test]
fn test_first_of_epoch_1() {
    assert!(is_first_block_after_epoch_checkpoint(33));
}

#[test]
fn test_second_of_epoch_1() {
    assert!(!is_first_block_after_epoch_checkpoint(34));
}

#[test]
fn test_first_of_epoch_2() {
    assert!(is_first_block_after_epoch_checkpoint(65));
}

#[test]
fn test_first_of_epoch_3() {
    assert!(is_first_block_after_epoch_checkpoint(97));
}

#[test]
fn test_large_height() {
    assert!(is_first_block_after_epoch_checkpoint(3201));
}
