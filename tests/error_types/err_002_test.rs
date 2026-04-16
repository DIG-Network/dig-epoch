/// ERR-002 — CheckpointCompetitionError Enum
///
/// Normative: docs/requirements/domains/error_types/NORMATIVE.md §ERR-002
/// Spec ref:  docs/resources/SPEC.md §10.2
use dig_epoch::error::CheckpointCompetitionError;
use std::error::Error;

/// InvalidData displays the correct message.
#[test]
fn test_invalid_data() {
    let e = CheckpointCompetitionError::InvalidData("bad hash".into());
    assert_eq!(e.to_string(), "Invalid checkpoint data: bad hash");
}

/// NotFound displays the correct message.
#[test]
fn test_not_found() {
    let e = CheckpointCompetitionError::NotFound(7);
    assert_eq!(
        e.to_string(),
        "Checkpoint competition not found for epoch 7"
    );
}

/// ScoreNotHigher displays the correct message.
#[test]
fn test_score_not_higher() {
    let e = CheckpointCompetitionError::ScoreNotHigher {
        current: 100,
        submitted: 50,
    };
    assert_eq!(e.to_string(), "Score not higher: current 100, submitted 50");
}

/// EpochMismatch displays the correct message.
#[test]
fn test_epoch_mismatch() {
    let e = CheckpointCompetitionError::EpochMismatch {
        expected: 5,
        got: 3,
    };
    assert_eq!(e.to_string(), "Epoch mismatch: expected 5, got 3");
}

/// AlreadyFinalized displays the correct message.
#[test]
fn test_already_finalized() {
    let e = CheckpointCompetitionError::AlreadyFinalized;
    assert_eq!(e.to_string(), "Competition already finalized");
}

/// NotStarted displays the correct message.
#[test]
fn test_not_started() {
    let e = CheckpointCompetitionError::NotStarted;
    assert_eq!(e.to_string(), "Competition not started");
}

/// All 6 variants produce valid Debug output.
#[test]
fn test_all_variants_debug() {
    let variants: &[CheckpointCompetitionError] = &[
        CheckpointCompetitionError::InvalidData("x".into()),
        CheckpointCompetitionError::NotFound(0),
        CheckpointCompetitionError::ScoreNotHigher {
            current: 0,
            submitted: 0,
        },
        CheckpointCompetitionError::EpochMismatch {
            expected: 0,
            got: 0,
        },
        CheckpointCompetitionError::AlreadyFinalized,
        CheckpointCompetitionError::NotStarted,
    ];
    for v in variants {
        let _ = format!("{v:?}");
    }
}

/// All 6 variants can be cloned.
#[test]
fn test_all_variants_clone() {
    let variants: Vec<CheckpointCompetitionError> = vec![
        CheckpointCompetitionError::InvalidData("x".into()),
        CheckpointCompetitionError::NotFound(0),
        CheckpointCompetitionError::ScoreNotHigher {
            current: 0,
            submitted: 0,
        },
        CheckpointCompetitionError::EpochMismatch {
            expected: 0,
            got: 0,
        },
        CheckpointCompetitionError::AlreadyFinalized,
        CheckpointCompetitionError::NotStarted,
    ];
    for v in &variants {
        let cloned = v.clone();
        assert_eq!(v.to_string(), cloned.to_string());
    }
}

/// CheckpointCompetitionError implements std::error::Error.
#[test]
fn test_competition_error_is_std_error() {
    let e: &dyn Error = &CheckpointCompetitionError::NotStarted;
    let _ = e.to_string();
}
