/// CKP-004 — finalize_competition + get_competition.
///
/// Normative: docs/requirements/domains/checkpoint_competition/NORMATIVE.md §CKP-004
/// Spec ref:  docs/resources/SPEC.md §6.5
use chia_protocol::Bytes32;
use dig_block::{Checkpoint, CheckpointSubmission, PublicKey, Signature, SignerBitmap};
use dig_epoch::error::EpochError;
use dig_epoch::manager::EpochManager;
use dig_epoch::types::checkpoint_competition::{CheckpointCompetition, CompetitionStatus};

fn nid() -> Bytes32 {
    Bytes32::new([0u8; 32])
}

fn make_submission(epoch: u64, score: u64) -> CheckpointSubmission {
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

/// Drive a manager to Finalization phase with one winning submission.
fn prepare_with_winner() -> EpochManager {
    let m = EpochManager::new(nid(), 100, nid());
    // Checkpoint phase → start + submit winner.
    m.update_phase(116);
    m.start_checkpoint_competition().unwrap();
    m.submit_checkpoint(make_submission(0, 100)).unwrap();
    // Advance phase to Finalization.
    m.update_phase(124);
    m
}

/// Finalize from WinnerSelected returns Some(checkpoint), status = Finalized.
#[test]
fn test_finalize_from_winner_selected() {
    let m = prepare_with_winner();
    let winner = m.finalize_competition(0, 200).unwrap();
    assert!(winner.is_some());
    let c = m.competition();
    match c.status {
        CompetitionStatus::Finalized { l1_height, .. } => {
            assert_eq!(l1_height, 200);
        }
        _ => panic!("expected Finalized"),
    }
}

/// Finalize sets the winning checkpoint on the current EpochInfo.
#[test]
fn test_finalize_sets_epoch_checkpoint() {
    let m = prepare_with_winner();
    m.finalize_competition(0, 200).unwrap();
    let info = m.current_epoch_info();
    assert!(info.checkpoint.is_some());
    let cp = info.checkpoint.unwrap();
    assert_eq!(cp.epoch, 0);
}

/// Finalize preserves the l1_height in the Finalized status.
#[test]
fn test_finalize_preserves_l1_height() {
    let m = prepare_with_winner();
    m.finalize_competition(0, 12_345).unwrap();
    match m.competition().status {
        CompetitionStatus::Finalized { l1_height, .. } => {
            assert_eq!(l1_height, 12_345);
        }
        _ => panic!(),
    }
}

/// Finalize with no submissions: transitions to Failed, returns None.
#[test]
fn test_finalize_with_no_winner_fails() {
    let m = EpochManager::new(nid(), 100, nid());
    m.update_phase(116);
    m.start_checkpoint_competition().unwrap();
    m.update_phase(124); // Finalization, still Collecting with no submissions
    let result = m.finalize_competition(0, 200).unwrap();
    assert!(result.is_none());
    assert_eq!(m.competition().status, CompetitionStatus::Failed);
}

/// Finalize outside Finalization phase: PhaseMismatch.
#[test]
fn test_finalize_wrong_phase() {
    let m = EpochManager::new(nid(), 100, nid());
    // BlockProduction
    let err = m.finalize_competition(0, 200).unwrap_err();
    assert!(matches!(err, EpochError::PhaseMismatch { .. }));
}

/// Epoch mismatch is rejected.
#[test]
fn test_finalize_epoch_mismatch() {
    let m = prepare_with_winner();
    let err = m.finalize_competition(999, 200).unwrap_err();
    assert!(matches!(err, EpochError::EpochMismatch { .. }));
}

/// Double-finalize returns error.
#[test]
fn test_finalize_already_finalized() {
    let m = prepare_with_winner();
    m.finalize_competition(0, 200).unwrap();
    let err = m.finalize_competition(0, 300).unwrap_err();
    assert!(matches!(err, EpochError::Competition(_)));
}

/// get_competition returns Some for current epoch, None otherwise.
#[test]
fn test_get_competition() {
    let m = EpochManager::new(nid(), 100, nid());
    let c0 = m.get_competition(0).unwrap();
    assert_eq!(c0.epoch, 0);
    assert_eq!(c0.status, CompetitionStatus::Pending);
    assert!(m.get_competition(1).is_none());
    assert!(m.get_competition(999).is_none());
}

/// Struct-level finalize: WinnerSelected → Finalized.
#[test]
fn test_struct_finalize() {
    let mut c = CheckpointCompetition::new(0);
    c.status = CompetitionStatus::WinnerSelected {
        winner_hash: Bytes32::new([9u8; 32]),
        winner_score: 42,
    };
    c.current_winner = Some(0);
    let h = c.finalize(1234).unwrap();
    assert_eq!(h, Bytes32::new([9u8; 32]));
    assert!(matches!(c.status, CompetitionStatus::Finalized { .. }));
}
