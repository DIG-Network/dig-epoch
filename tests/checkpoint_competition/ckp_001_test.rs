/// CKP-001 — CheckpointCompetition struct and CompetitionStatus enum.
///
/// Normative: docs/requirements/domains/checkpoint_competition/NORMATIVE.md §CKP-001
/// Spec ref:  docs/resources/SPEC.md §3.9, §3.10
use chia_protocol::Bytes32;
use dig_epoch::types::checkpoint_competition::{CheckpointCompetition, CompetitionStatus};

fn h(x: u8) -> Bytes32 {
    Bytes32::new([x; 32])
}

/// Construction: epoch, empty submissions, Pending status, no winner.
#[test]
fn test_competition_construction() {
    let c = CheckpointCompetition::new(42);
    assert_eq!(c.epoch, 42);
    assert!(c.submissions.is_empty());
    assert_eq!(c.status, CompetitionStatus::Pending);
    assert!(c.current_winner.is_none());
}

/// Status: Pending variant is the initial state.
#[test]
fn test_status_pending() {
    let c = CheckpointCompetition::new(0);
    assert!(matches!(c.status, CompetitionStatus::Pending));
}

/// Status: Collecting variant is constructible and distinguishable.
#[test]
fn test_status_collecting() {
    let s = CompetitionStatus::Collecting;
    assert!(matches!(s, CompetitionStatus::Collecting));
    assert_ne!(s, CompetitionStatus::Pending);
}

/// Status: WinnerSelected carries winner_hash + winner_score.
#[test]
fn test_status_winner_selected() {
    let s = CompetitionStatus::WinnerSelected {
        winner_hash: h(0xAB),
        winner_score: 99,
    };
    match s {
        CompetitionStatus::WinnerSelected {
            winner_hash,
            winner_score,
        } => {
            assert_eq!(winner_hash, h(0xAB));
            assert_eq!(winner_score, 99);
        }
        _ => panic!("wrong variant"),
    }
}

/// Status: Finalized carries winner_hash + l1_height.
#[test]
fn test_status_finalized() {
    let s = CompetitionStatus::Finalized {
        winner_hash: h(0xCD),
        l1_height: 1234,
    };
    match s {
        CompetitionStatus::Finalized {
            winner_hash,
            l1_height,
        } => {
            assert_eq!(winner_hash, h(0xCD));
            assert_eq!(l1_height, 1234);
        }
        _ => panic!("wrong variant"),
    }
}

/// Status: Failed variant is constructible.
#[test]
fn test_status_failed() {
    let s = CompetitionStatus::Failed;
    assert!(matches!(s, CompetitionStatus::Failed));
}

/// is_finalized returns true only for Finalized variant.
#[test]
fn test_is_finalized() {
    let mut c = CheckpointCompetition::new(0);
    assert!(!c.is_finalized());
    c.status = CompetitionStatus::Collecting;
    assert!(!c.is_finalized());
    c.status = CompetitionStatus::WinnerSelected {
        winner_hash: h(1),
        winner_score: 5,
    };
    assert!(!c.is_finalized());
    c.status = CompetitionStatus::Finalized {
        winner_hash: h(1),
        l1_height: 10,
    };
    assert!(c.is_finalized());
    c.status = CompetitionStatus::Failed;
    assert!(!c.is_finalized());
}

/// Clone produces an equivalent copy.
#[test]
fn test_clone() {
    let c = CheckpointCompetition::new(7);
    let c2 = c.clone();
    assert_eq!(c.epoch, c2.epoch);
    assert_eq!(c.status, c2.status);
    assert_eq!(c.submissions.len(), c2.submissions.len());
}
