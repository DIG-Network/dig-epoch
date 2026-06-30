/// CKP-003 — submit_checkpoint with score comparison.
///
/// Normative: docs/requirements/domains/checkpoint_competition/NORMATIVE.md §CKP-003
/// Spec ref:  docs/resources/SPEC.md §6.5
use chia_protocol::Bytes32;
use dig_block::{Checkpoint, CheckpointSubmission, SignerBitmap};
use dig_block::{PublicKey, Signature};
use dig_epoch::error::{CheckpointCompetitionError, EpochError};
use dig_epoch::manager::EpochManager;
use dig_epoch::types::checkpoint_competition::{CheckpointCompetition, CompetitionStatus};
use dig_epoch::types::epoch_phase::EpochPhase;

fn nid() -> Bytes32 {
    Bytes32::new([0u8; 32])
}

fn make_submission(epoch: u64, score: u64, block_count: u32) -> CheckpointSubmission {
    let mut cp = Checkpoint::new();
    cp.epoch = epoch;
    cp.block_count = block_count;
    // Mark the checkpoint's state_root with the score so distinct submissions
    // hash differently — otherwise all-zero checkpoints collide.
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

fn mgr_collecting(epoch: u64) -> EpochManager {
    let m = EpochManager::new(nid(), 100, nid());
    m.update_phase(116); // Checkpoint
    let mut c = CheckpointCompetition::new(epoch);
    c.status = CompetitionStatus::Collecting;
    m.__set_competition_for_test(c);
    m
}

/// First submission with positive score becomes the winner.
#[test]
fn test_first_submission_wins() {
    let m = mgr_collecting(0);
    let became_leader = m.submit_checkpoint(make_submission(0, 100, 3)).unwrap();
    assert!(became_leader);
    let c = m.competition();
    assert_eq!(c.submissions.len(), 1);
    match c.status {
        CompetitionStatus::WinnerSelected { winner_score, .. } => {
            assert_eq!(winner_score, 100);
        }
        _ => panic!("expected WinnerSelected"),
    }
    assert_eq!(c.current_winner, Some(0));
}

/// Higher-scoring submission replaces leader.
#[test]
fn test_higher_score_replaces() {
    let m = mgr_collecting(0);
    m.submit_checkpoint(make_submission(0, 100, 2)).unwrap();
    let became_leader = m.submit_checkpoint(make_submission(0, 150, 3)).unwrap();
    assert!(became_leader);
    let c = m.competition();
    assert_eq!(c.current_winner, Some(1));
    match c.status {
        CompetitionStatus::WinnerSelected { winner_score, .. } => {
            assert_eq!(winner_score, 150);
        }
        _ => panic!("expected WinnerSelected"),
    }
}

/// Lower-scoring submission rejected with ScoreNotHigher; still recorded.
#[test]
fn test_lower_score_rejected() {
    let m = mgr_collecting(0);
    m.submit_checkpoint(make_submission(0, 100, 2)).unwrap();
    let err = m.submit_checkpoint(make_submission(0, 50, 1)).unwrap_err();
    assert!(matches!(
        err,
        EpochError::Competition(CheckpointCompetitionError::ScoreNotHigher {
            current: 100,
            submitted: 50
        })
    ));
    // Submission still recorded despite rejection.
    assert_eq!(m.competition().submissions.len(), 2);
    // Leader unchanged.
    assert_eq!(m.competition().current_winner, Some(0));
}

/// Equal score is rejected.
#[test]
fn test_equal_score_rejected() {
    let m = mgr_collecting(0);
    m.submit_checkpoint(make_submission(0, 100, 2)).unwrap();
    let err = m.submit_checkpoint(make_submission(0, 100, 3)).unwrap_err();
    assert!(matches!(
        err,
        EpochError::Competition(CheckpointCompetitionError::ScoreNotHigher { .. })
    ));
}

/// Epoch-mismatched submission rejected.
#[test]
fn test_epoch_mismatch() {
    let m = mgr_collecting(5);
    let err = m.submit_checkpoint(make_submission(9, 100, 2)).unwrap_err();
    assert!(matches!(
        err,
        EpochError::Competition(CheckpointCompetitionError::EpochMismatch {
            expected: 5,
            got: 9
        })
    ));
}

/// Submit before start (Pending) rejected with NotStarted.
#[test]
fn test_submit_while_pending() {
    let m = EpochManager::new(nid(), 100, nid());
    m.update_phase(116);
    let err = m.submit_checkpoint(make_submission(0, 100, 2)).unwrap_err();
    assert!(matches!(
        err,
        EpochError::Competition(CheckpointCompetitionError::NotStarted)
    ));
}

/// submit_checkpoint outside the `Checkpoint` phase is rejected with
/// `PhaseMismatch` before the competition is ever consulted. Forces the
/// current epoch into `Finalization` (a non-Checkpoint phase) so the phase
/// gate fires regardless of competition state.
#[test]
fn test_submit_outside_checkpoint_phase() {
    let m = EpochManager::new(nid(), 100, nid());
    m.__force_phase_for_test(EpochPhase::Finalization);
    let err = m.submit_checkpoint(make_submission(0, 100, 2)).unwrap_err();
    assert!(matches!(
        err,
        EpochError::PhaseMismatch {
            expected: EpochPhase::Checkpoint,
            got: EpochPhase::Finalization,
        }
    ));
}

/// Submissions are recorded in order regardless of outcome.
#[test]
fn test_submissions_recorded() {
    let m = mgr_collecting(0);
    m.submit_checkpoint(make_submission(0, 100, 2)).unwrap();
    let _ = m.submit_checkpoint(make_submission(0, 50, 1));
    let _ = m.submit_checkpoint(make_submission(0, 200, 4));
    let c = m.competition();
    assert_eq!(c.submissions.len(), 3);
    assert_eq!(c.submissions[0].score, 100);
    assert_eq!(c.submissions[1].score, 50);
    assert_eq!(c.submissions[2].score, 200);
}
