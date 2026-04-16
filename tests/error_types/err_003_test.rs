/// ERR-003 — Error Conversions and Display
///
/// Normative: docs/requirements/domains/error_types/NORMATIVE.md §ERR-003
/// Spec ref:  docs/resources/SPEC.md §10
use dig_epoch::error::{CheckpointCompetitionError, EpochError};
use std::error::Error;

/// From<CheckpointCompetitionError::NotStarted> produces Competition(NotStarted).
#[test]
fn test_from_not_started() {
    let e: EpochError = CheckpointCompetitionError::NotStarted.into();
    assert!(matches!(
        e,
        EpochError::Competition(CheckpointCompetitionError::NotStarted)
    ));
}

/// From<CheckpointCompetitionError::AlreadyFinalized> produces Competition(AlreadyFinalized).
#[test]
fn test_from_already_finalized() {
    let e: EpochError = CheckpointCompetitionError::AlreadyFinalized.into();
    assert!(matches!(
        e,
        EpochError::Competition(CheckpointCompetitionError::AlreadyFinalized)
    ));
}

/// From<ScoreNotHigher { .. }> produces Competition(ScoreNotHigher { .. }).
#[test]
fn test_from_score_not_higher() {
    let e: EpochError = CheckpointCompetitionError::ScoreNotHigher {
        current: 10,
        submitted: 5,
    }
    .into();
    assert!(matches!(
        e,
        EpochError::Competition(CheckpointCompetitionError::ScoreNotHigher { .. })
    ));
}

/// The ? operator propagates CheckpointCompetitionError into Result<_, EpochError>.
#[test]
fn test_question_mark_propagation() {
    fn inner() -> Result<(), CheckpointCompetitionError> {
        Err(CheckpointCompetitionError::NotStarted)
    }
    fn outer() -> Result<(), EpochError> {
        inner()?;
        Ok(())
    }
    let result = outer();
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        EpochError::Competition(CheckpointCompetitionError::NotStarted)
    ));
}

/// Competition(NotStarted) displays as "Competition error: Competition not started".
#[test]
fn test_competition_display_chain() {
    let e = EpochError::Competition(CheckpointCompetitionError::NotStarted);
    assert_eq!(e.to_string(), "Competition error: Competition not started");
}

/// EpochError implements std::error::Error.
#[test]
fn test_epoch_error_is_std_error() {
    let e: &dyn Error = &EpochError::InvalidHeight(0);
    let _ = e.to_string();
}

/// CheckpointCompetitionError implements std::error::Error.
#[test]
fn test_competition_error_is_std_error() {
    let e: &dyn Error = &CheckpointCompetitionError::NotStarted;
    let _ = e.to_string();
}
