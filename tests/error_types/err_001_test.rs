/// ERR-001 — EpochError Enum
///
/// Normative: docs/requirements/domains/error_types/NORMATIVE.md §ERR-001
/// Spec ref:  docs/resources/SPEC.md §10.1
use dig_epoch::error::{CheckpointCompetitionError, EpochError};
use dig_epoch::types::epoch_phase::EpochPhase;
use std::error::Error;

/// EpochNotComplete displays the correct message.
#[test]
fn test_epoch_not_complete() {
    let e = EpochError::EpochNotComplete(5);
    assert_eq!(e.to_string(), "Cannot advance: epoch 5 is not complete");
}

/// NoFinalizedCheckpoint displays the correct message.
#[test]
fn test_no_finalized_checkpoint() {
    let e = EpochError::NoFinalizedCheckpoint(3);
    assert_eq!(
        e.to_string(),
        "Cannot advance: epoch 3 has no finalized checkpoint"
    );
}

/// CheckpointBlockNotEmpty displays the correct message.
#[test]
fn test_checkpoint_block_not_empty() {
    let e = EpochError::CheckpointBlockNotEmpty(32, 1, 100, 50);
    assert_eq!(
        e.to_string(),
        "Checkpoint block at height 32 is not empty: 1 bundles, 100 cost, 50 fees"
    );
}

/// PhaseMismatch display includes both phase names.
#[test]
fn test_phase_mismatch() {
    let e = EpochError::PhaseMismatch {
        expected: EpochPhase::BlockProduction,
        got: EpochPhase::Checkpoint,
    };
    let msg = e.to_string();
    assert!(
        msg.contains("BlockProduction") && msg.contains("Checkpoint"),
        "PhaseMismatch message must contain both phase names, got: {msg}"
    );
}

/// EpochMismatch displays the correct message.
#[test]
fn test_epoch_mismatch() {
    let e = EpochError::EpochMismatch {
        expected: 5,
        got: 3,
    };
    assert_eq!(e.to_string(), "Epoch mismatch: expected 5, got 3");
}

/// InvalidHeight displays the correct message.
#[test]
fn test_invalid_height() {
    let e = EpochError::InvalidHeight(0);
    assert_eq!(e.to_string(), "Invalid height 0: below genesis");
}

/// DfspNotActive displays the correct message.
#[test]
fn test_dfsp_not_active() {
    let e = EpochError::DfspNotActive(100);
    assert_eq!(e.to_string(), "DFSP not active at height 100");
}

/// DfspBoundary displays the correct message.
#[test]
fn test_dfsp_boundary() {
    let e = EpochError::DfspBoundary("burn failed".into());
    assert_eq!(e.to_string(), "DFSP epoch-boundary error: burn failed");
}

/// Competition variant wraps CheckpointCompetitionError via #[from].
#[test]
fn test_competition_from() {
    let inner = CheckpointCompetitionError::NotStarted;
    let e: EpochError = inner.into();
    assert!(
        matches!(
            e,
            EpochError::Competition(CheckpointCompetitionError::NotStarted)
        ),
        "Expected Competition(NotStarted)"
    );
}

/// All 9 EpochError variants produce valid Debug output.
#[test]
fn test_all_variants_debug() {
    let variants: &[EpochError] = &[
        EpochError::EpochNotComplete(0),
        EpochError::NoFinalizedCheckpoint(0),
        EpochError::CheckpointBlockNotEmpty(0, 0, 0, 0),
        EpochError::PhaseMismatch {
            expected: EpochPhase::BlockProduction,
            got: EpochPhase::Complete,
        },
        EpochError::EpochMismatch {
            expected: 0,
            got: 0,
        },
        EpochError::InvalidHeight(0),
        EpochError::DfspNotActive(0),
        EpochError::DfspBoundary("x".into()),
        EpochError::Competition(CheckpointCompetitionError::NotStarted),
    ];
    for v in variants {
        let _ = format!("{v:?}");
    }
}

/// All 9 EpochError variants can be cloned.
#[test]
fn test_all_variants_clone() {
    let variants: Vec<EpochError> = vec![
        EpochError::EpochNotComplete(0),
        EpochError::NoFinalizedCheckpoint(0),
        EpochError::CheckpointBlockNotEmpty(0, 0, 0, 0),
        EpochError::PhaseMismatch {
            expected: EpochPhase::BlockProduction,
            got: EpochPhase::Complete,
        },
        EpochError::EpochMismatch {
            expected: 0,
            got: 0,
        },
        EpochError::InvalidHeight(0),
        EpochError::DfspNotActive(0),
        EpochError::DfspBoundary("x".into()),
        EpochError::Competition(CheckpointCompetitionError::NotStarted),
    ];
    for v in &variants {
        let cloned = v.clone();
        assert_eq!(v.to_string(), cloned.to_string());
    }
}

/// EpochError implements std::error::Error.
#[test]
fn test_epoch_error_is_std_error() {
    let e: &dyn Error = &EpochError::InvalidHeight(0);
    let _ = e.to_string();
}
