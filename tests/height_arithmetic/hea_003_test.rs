/// HEA-003 — Checkpoint block detection
///
/// Normative: docs/requirements/domains/height_arithmetic/NORMATIVE.md §HEA-003
/// Spec ref:  docs/resources/SPEC.md §5.2–5.3
use dig_epoch::arithmetic::{
    ensure_checkpoint_block_empty, is_checkpoint_class_block, is_epoch_checkpoint_block,
    is_genesis_checkpoint_block,
};

/// Height 1 (genesis) is a checkpoint-class block.
#[test]
fn test_genesis_is_checkpoint_class() {
    assert!(is_genesis_checkpoint_block(1));
    assert!(is_checkpoint_class_block(1));
}

/// Height 32 and 64 (epoch checkpoints) are checkpoint-class.
#[test]
fn test_epoch_checkpoint_is_checkpoint_class() {
    assert!(is_epoch_checkpoint_block(32));
    assert!(is_checkpoint_class_block(32));
    assert!(is_epoch_checkpoint_block(64));
    assert!(is_checkpoint_class_block(64));
}

/// is_genesis_checkpoint_block(32) is false; is_epoch_checkpoint_block(1) is false.
#[test]
fn test_detection_specificity() {
    assert!(!is_genesis_checkpoint_block(32));
    assert!(!is_epoch_checkpoint_block(1));
}

/// Normal blocks (not divisible by 32 and not 1) are not checkpoint-class.
#[test]
fn test_normal_blocks_not_checkpoint_class() {
    assert!(!is_checkpoint_class_block(2));
    assert!(!is_checkpoint_class_block(33));
    assert!(!is_checkpoint_class_block(65));
}

/// ensure_checkpoint_block_empty passes at checkpoint height with all zeros.
#[test]
fn test_ensure_empty_passes_at_checkpoint() {
    assert!(ensure_checkpoint_block_empty(32, 0, 0, 0).is_ok());
}

/// ensure_checkpoint_block_empty rejects non-zero bundle count at checkpoint.
#[test]
fn test_ensure_empty_rejects_bundles() {
    assert!(ensure_checkpoint_block_empty(32, 1, 0, 0).is_err());
}

/// ensure_checkpoint_block_empty rejects non-zero cost at checkpoint.
#[test]
fn test_ensure_empty_rejects_cost() {
    assert!(ensure_checkpoint_block_empty(32, 0, 100, 0).is_err());
}

/// ensure_checkpoint_block_empty rejects non-zero fees at checkpoint.
#[test]
fn test_ensure_empty_rejects_fees() {
    assert!(ensure_checkpoint_block_empty(32, 0, 0, 50).is_err());
}

/// ensure_checkpoint_block_empty passes at non-checkpoint height regardless of values.
#[test]
fn test_ensure_empty_passes_non_checkpoint() {
    assert!(ensure_checkpoint_block_empty(15, 5, 1000, 500).is_ok());
}

/// ensure_checkpoint_block_empty(1, 0, 0, 0) passes (genesis is checkpoint-class).
#[test]
fn test_ensure_empty_genesis_passes() {
    assert!(ensure_checkpoint_block_empty(1, 0, 0, 0).is_ok());
}

/// ensure_checkpoint_block_empty(1, 1, 0, 0) fails (genesis is checkpoint-class).
#[test]
fn test_ensure_empty_genesis_rejects() {
    assert!(ensure_checkpoint_block_empty(1, 1, 0, 0).is_err());
}
