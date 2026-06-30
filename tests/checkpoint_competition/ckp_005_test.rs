/// CKP-005 — Competition lifecycle state machine (Pending → Collecting →
/// WinnerSelected → Finalized / Failed).
///
/// Normative: docs/requirements/domains/checkpoint_competition/NORMATIVE.md §CKP-005
/// Spec ref:  docs/resources/SPEC.md §3.10
use chia_protocol::Bytes32;
use dig_block::{Checkpoint, CheckpointSubmission, PublicKey, Signature, SignerBitmap};
use dig_epoch::types::checkpoint_competition::{CheckpointCompetition, CompetitionStatus};

fn submission(epoch: u64, score: u64) -> CheckpointSubmission {
    let mut cp = Checkpoint::new();
    cp.epoch = epoch;
    cp.state_root = Bytes32::new([score as u8; 32]);
    CheckpointSubmission::new(
        cp,
        SignerBitmap::new(0),
        Signature::default(),
        PublicKey::default(),
        score,
        0,
    )
}

/// Creation invariants: Pending, epoch set, empty submissions, no winner.
#[test]
fn test_creation_invariants() {
    let c = CheckpointCompetition::new(42);
    assert_eq!(c.epoch, 42);
    assert_eq!(c.status, CompetitionStatus::Pending);
    assert!(c.submissions.is_empty());
    assert!(c.current_winner.is_none());
}

/// Happy path: Pending → Collecting → WinnerSelected → Finalized.
#[test]
fn test_full_success_lifecycle() {
    let mut c = CheckpointCompetition::new(0);
    assert_eq!(c.status, CompetitionStatus::Pending);

    c.start().unwrap();
    assert_eq!(c.status, CompetitionStatus::Collecting);

    c.submit(submission(0, 100)).unwrap();
    assert!(matches!(c.status, CompetitionStatus::WinnerSelected { .. }));

    let winner_hash = c.finalize(1234).unwrap();
    match c.status {
        CompetitionStatus::Finalized {
            winner_hash: h,
            l1_height,
        } => {
            assert_eq!(h, winner_hash);
            assert_eq!(l1_height, 1234);
        }
        _ => panic!("expected Finalized"),
    }
}

/// Pending → Collecting → Failed.
#[test]
fn test_failure_from_collecting() {
    let mut c = CheckpointCompetition::new(0);
    c.start().unwrap();
    c.fail().unwrap();
    assert_eq!(c.status, CompetitionStatus::Failed);
}

/// Pending → Collecting → WinnerSelected → Failed.
#[test]
fn test_failure_from_winner_selected() {
    let mut c = CheckpointCompetition::new(0);
    c.start().unwrap();
    c.submit(submission(0, 50)).unwrap();
    c.fail().unwrap();
    assert_eq!(c.status, CompetitionStatus::Failed);
}

/// Leader updates across multiple higher-score submissions.
#[test]
fn test_leader_update_cycle() {
    let mut c = CheckpointCompetition::new(0);
    c.start().unwrap();
    c.submit(submission(0, 10)).unwrap();
    c.submit(submission(0, 50)).unwrap();
    c.submit(submission(0, 200)).unwrap();
    match c.status {
        CompetitionStatus::WinnerSelected { winner_score, .. } => {
            assert_eq!(winner_score, 200);
        }
        _ => panic!("expected WinnerSelected"),
    }
    assert_eq!(c.current_winner, Some(2));
}

/// Finalized is terminal: further transitions rejected.
#[test]
fn test_finalized_is_terminal() {
    let mut c = CheckpointCompetition::new(0);
    c.start().unwrap();
    c.submit(submission(0, 10)).unwrap();
    c.finalize(100).unwrap();
    assert!(c.finalize(200).is_err());
    assert!(c.fail().is_err());
    assert!(c.submit(submission(0, 100)).is_err());
    assert!(c.start().is_err());
}

/// Failed is terminal.
#[test]
fn test_failed_is_terminal() {
    let mut c = CheckpointCompetition::new(0);
    c.start().unwrap();
    c.fail().unwrap();
    assert!(c.finalize(200).is_err());
    assert!(c.fail().is_err());
    assert!(c.submit(submission(0, 100)).is_err());
    assert!(c.start().is_err());
}

/// Invalid: Pending → Finalized.
#[test]
fn test_invalid_pending_to_finalized() {
    let mut c = CheckpointCompetition::new(0);
    assert!(c.finalize(100).is_err());
    assert_eq!(c.status, CompetitionStatus::Pending);
}

/// Invalid: Collecting → Finalized (no winner yet).
#[test]
fn test_invalid_collecting_to_finalized() {
    let mut c = CheckpointCompetition::new(0);
    c.start().unwrap();
    assert!(c.finalize(100).is_err());
    assert_eq!(c.status, CompetitionStatus::Collecting);
}

/// Invalid: fail() from Pending (never started) → NotStarted, status unchanged.
#[test]
fn test_fail_from_pending() {
    let mut c = CheckpointCompetition::new(0);
    let err = c.fail().unwrap_err();
    assert!(matches!(
        err,
        dig_epoch::error::CheckpointCompetitionError::NotStarted
    ));
    assert_eq!(c.status, CompetitionStatus::Pending);
}

/// winner() is None until a submission leads, then borrows the leading entry.
#[test]
fn test_winner_accessor() {
    let mut c = CheckpointCompetition::new(0);
    assert!(c.winner().is_none()); // Pending: no winner
    c.start().unwrap();
    assert!(c.winner().is_none()); // Collecting, still empty
    c.submit(submission(0, 100)).unwrap();
    let w = c
        .winner()
        .expect("a leader exists after a positive-score submit");
    assert_eq!(w.score, 100);
    // A higher submission moves the winner pointer.
    c.submit(submission(0, 250)).unwrap();
    assert_eq!(c.winner().unwrap().score, 250);
}
