/// MGR-004 — advance_epoch archives summary, creates epoch+1, resets competition.
///
/// Normative: docs/requirements/domains/epoch_manager/NORMATIVE.md §MGR-004
/// Spec ref:  docs/resources/SPEC.md §6.7
use chia_protocol::Bytes32;
use dig_epoch::constants::BLOCKS_PER_EPOCH;
use dig_epoch::error::EpochError;
use dig_epoch::manager::EpochManager;
use dig_epoch::types::checkpoint_competition::{CheckpointCompetition, CompetitionStatus};
use dig_epoch::types::epoch_phase::EpochPhase;

fn nid() -> Bytes32 {
    Bytes32::new([0u8; 32])
}
fn root(x: u8) -> Bytes32 {
    Bytes32::new([x; 32])
}

fn finalize_current_competition(m: &EpochManager) {
    // Use the test-helper to install a Finalized competition on the current epoch.
    let cur = m.current_epoch();
    let mut c = CheckpointCompetition::new(cur);
    c.status = CompetitionStatus::Finalized {
        winner_hash: root(0xAB),
        l1_height: 132,
    };
    m.__set_competition_for_test(c);
}

fn complete_and_finalize(m: &EpochManager) {
    // Progress past the 100% boundary (end_l1 from l1_range is inclusive end,
    // so use end+1 to push pct to 100 and hit Complete).
    let e = m.current_epoch();
    let (_, end_l1) = m.l1_range_for_epoch(e);
    m.update_phase(end_l1 + 1);
    assert_eq!(m.current_phase(), EpochPhase::Complete);
    finalize_current_competition(m);
}

/// Happy path: returns new epoch number.
#[test]
fn test_advance_returns_new_epoch_number() {
    let m = EpochManager::new(nid(), 100, root(0));
    complete_and_finalize(&m);
    let next = m.advance_epoch(132, root(1)).unwrap();
    assert_eq!(next, 1);
    assert_eq!(m.current_epoch(), 1);
}

/// Rejects when phase is not Complete.
#[test]
fn test_rejects_non_complete_phase() {
    let m = EpochManager::new(nid(), 100, root(0));
    finalize_current_competition(&m); // still in BlockProduction
    let e = m.advance_epoch(132, root(1)).unwrap_err();
    assert!(matches!(e, EpochError::EpochNotComplete(0)));
}

/// Rejects when competition is not finalized.
#[test]
fn test_rejects_missing_finalized_checkpoint() {
    let m = EpochManager::new(nid(), 100, root(0));
    m.update_phase(132); // → Complete; competition still Pending
    let e = m.advance_epoch(132, root(1)).unwrap_err();
    assert!(matches!(e, EpochError::NoFinalizedCheckpoint(0)));
}

/// Summary is archived with source epoch's data.
#[test]
fn test_summary_archived() {
    let m = EpochManager::new(nid(), 100, root(0));
    m.record_block(100, 2).unwrap();
    m.record_block(200, 3).unwrap();
    complete_and_finalize(&m);
    m.advance_epoch(132, root(1)).unwrap();
    let s = m.get_epoch_summary(0).unwrap();
    assert_eq!(s.epoch, 0);
    assert_eq!(s.blocks, 2);
    assert_eq!(s.fees, 300);
    assert_eq!(s.transactions, 5);
}

/// New epoch uses provided state_root and starts in BlockProduction.
#[test]
fn test_new_epoch_state_root_and_phase() {
    let m = EpochManager::new(nid(), 100, root(0));
    complete_and_finalize(&m);
    m.advance_epoch(132, root(9)).unwrap();
    let info = m.current_epoch_info();
    assert_eq!(info.epoch, 1);
    assert_eq!(info.start_state_root, root(9));
    assert_eq!(info.phase, EpochPhase::BlockProduction);
}

/// New epoch counters are zeroed.
#[test]
fn test_zeroed_counters() {
    let m = EpochManager::new(nid(), 100, root(0));
    m.record_block(500, 7).unwrap();
    complete_and_finalize(&m);
    m.advance_epoch(132, root(1)).unwrap();
    let info = m.current_epoch_info();
    assert_eq!(info.blocks_produced, 0);
    assert_eq!(info.total_fees, 0);
    assert_eq!(info.total_transactions, 0);
}

/// Competition is reset for the new epoch.
#[test]
fn test_competition_reset() {
    let m = EpochManager::new(nid(), 100, root(0));
    complete_and_finalize(&m);
    m.advance_epoch(132, root(1)).unwrap();
    let c = m.competition();
    assert_eq!(c.epoch, 1);
    assert_eq!(c.status, CompetitionStatus::Pending);
    assert!(c.submissions.is_empty());
    assert!(c.current_winner.is_none());
}

/// Multiple sequential advances preserve all summaries in order.
#[test]
fn test_multiple_advances_preserve_summaries() {
    let m = EpochManager::new(nid(), 100, root(0));
    for _ in 0..3 {
        complete_and_finalize(&m);
        m.advance_epoch(132, root(1)).unwrap();
    }
    assert_eq!(m.current_epoch(), 3);
    let recent = m.recent_summaries(10);
    assert_eq!(recent.len(), 3);
    assert_eq!(recent[0].epoch, 0);
    assert_eq!(recent[1].epoch, 1);
    assert_eq!(recent[2].epoch, 2);
}

/// New epoch's start_l2_height advances by BLOCKS_PER_EPOCH.
#[test]
fn test_start_l2_height_advances() {
    let m = EpochManager::new(nid(), 100, root(0));
    let first_start = m.current_epoch_info().start_l2_height;
    complete_and_finalize(&m);
    m.advance_epoch(132, root(1)).unwrap();
    let second_start = m.current_epoch_info().start_l2_height;
    assert_eq!(second_start, first_start + BLOCKS_PER_EPOCH);
}
