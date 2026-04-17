/// CKP-002 — start_checkpoint_competition transitions Pending → Collecting.
///
/// Normative: docs/requirements/domains/checkpoint_competition/NORMATIVE.md §CKP-002
/// Spec ref:  docs/resources/SPEC.md §6.5
use chia_protocol::Bytes32;
use dig_epoch::error::{CheckpointCompetitionError, EpochError};
use dig_epoch::manager::EpochManager;
use dig_epoch::types::checkpoint_competition::{CheckpointCompetition, CompetitionStatus};
use dig_epoch::types::epoch_phase::EpochPhase;

fn mgr() -> EpochManager {
    EpochManager::new(Bytes32::new([0u8; 32]), 100, Bytes32::new([0u8; 32]))
}

/// Happy path: Pending → Collecting in Checkpoint phase.
#[test]
fn test_start_from_pending() {
    let m = mgr();
    m.update_phase(116); // → Checkpoint
    m.start_checkpoint_competition().unwrap();
    assert_eq!(m.competition().status, CompetitionStatus::Collecting);
}

/// PhaseMismatch when called outside Checkpoint.
#[test]
fn test_start_wrong_phase() {
    let m = mgr();
    let err = m.start_checkpoint_competition().unwrap_err();
    assert!(matches!(
        err,
        EpochError::PhaseMismatch {
            expected: EpochPhase::Checkpoint,
            got: EpochPhase::BlockProduction
        }
    ));
}

/// Second call from Collecting fails.
#[test]
fn test_start_already_collecting() {
    let m = mgr();
    m.update_phase(116);
    m.start_checkpoint_competition().unwrap();
    let err = m.start_checkpoint_competition().unwrap_err();
    assert!(matches!(err, EpochError::Competition(_)));
}

/// Starting when status is WinnerSelected fails.
#[test]
fn test_start_already_winner_selected() {
    let m = mgr();
    m.update_phase(116);
    let mut c = CheckpointCompetition::new(0);
    c.status = CompetitionStatus::WinnerSelected {
        winner_hash: Bytes32::new([1u8; 32]),
        winner_score: 100,
    };
    m.__set_competition_for_test(c);
    let err = m.start_checkpoint_competition().unwrap_err();
    assert!(matches!(err, EpochError::Competition(_)));
}

/// Starting when status is Finalized fails.
#[test]
fn test_start_already_finalized() {
    let m = mgr();
    m.update_phase(116);
    let mut c = CheckpointCompetition::new(0);
    c.status = CompetitionStatus::Finalized {
        winner_hash: Bytes32::new([1u8; 32]),
        l1_height: 132,
    };
    m.__set_competition_for_test(c);
    let err = m.start_checkpoint_competition().unwrap_err();
    assert!(matches!(
        err,
        EpochError::Competition(CheckpointCompetitionError::AlreadyFinalized)
    ));
}

/// Starting when status is Failed fails.
#[test]
fn test_start_already_failed() {
    let m = mgr();
    m.update_phase(116);
    let mut c = CheckpointCompetition::new(0);
    c.status = CompetitionStatus::Failed;
    m.__set_competition_for_test(c);
    let err = m.start_checkpoint_competition().unwrap_err();
    assert!(matches!(err, EpochError::Competition(_)));
}

/// Direct struct-level start(): Pending → Collecting.
#[test]
fn test_struct_level_start() {
    let mut c = CheckpointCompetition::new(7);
    c.start().unwrap();
    assert_eq!(c.status, CompetitionStatus::Collecting);
}
