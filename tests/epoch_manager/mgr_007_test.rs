/// MGR-007 — Epoch history is ordered and append-only via advance_epoch.
///
/// Normative: docs/requirements/domains/epoch_manager/NORMATIVE.md §MGR-007
/// Spec ref:  docs/resources/SPEC.md §6.7
use chia_protocol::Bytes32;
use dig_epoch::manager::EpochManager;
use dig_epoch::types::checkpoint_competition::{CheckpointCompetition, CompetitionStatus};

fn nid() -> Bytes32 {
    Bytes32::new([0u8; 32])
}
fn r(x: u8) -> Bytes32 {
    Bytes32::new([x; 32])
}

fn advance(m: &EpochManager) {
    let e = m.current_epoch();
    let (_, end_l1) = m.l1_range_for_epoch(e);
    m.update_phase(end_l1 + 1);
    let mut c = CheckpointCompetition::new(e);
    c.status = CompetitionStatus::Finalized {
        winner_hash: r(0xAB),
        l1_height: end_l1 + 1,
    };
    m.__set_competition_for_test(c);
    m.advance_epoch(end_l1 + 1, r(1)).unwrap();
}

/// Each advance_epoch call appends exactly one summary.
#[test]
fn test_each_advance_appends_one() {
    let m = EpochManager::new(nid(), 100, r(0));
    assert_eq!(m.recent_summaries(1000).len(), 0);
    advance(&m);
    assert_eq!(m.recent_summaries(1000).len(), 1);
    advance(&m);
    assert_eq!(m.recent_summaries(1000).len(), 2);
}

/// Summaries ordered ascending by epoch number.
#[test]
fn test_summaries_ordered_ascending() {
    let m = EpochManager::new(nid(), 100, r(0));
    for _ in 0..5 {
        advance(&m);
    }
    let r = m.recent_summaries(10);
    for (i, s) in r.iter().enumerate() {
        assert_eq!(s.epoch, i as u64);
    }
}

/// Epoch numbers are consecutive (no gaps).
#[test]
fn test_consecutive_epoch_numbers() {
    let m = EpochManager::new(nid(), 100, r(0));
    for _ in 0..4 {
        advance(&m);
    }
    let r = m.recent_summaries(10);
    for w in r.windows(2) {
        assert_eq!(w[1].epoch, w[0].epoch + 1);
    }
}

/// History is append-only: length monotonically increases.
#[test]
fn test_append_only() {
    let m = EpochManager::new(nid(), 100, r(0));
    let mut last_len = 0usize;
    for _ in 0..3 {
        advance(&m);
        let cur_len = m.recent_summaries(1000).len();
        assert!(cur_len > last_len);
        last_len = cur_len;
    }
}

/// Summary content captures source epoch's recorded data.
#[test]
fn test_summary_content_matches_source() {
    let m = EpochManager::new(nid(), 100, r(0));
    m.record_block(100, 1).unwrap();
    m.record_block(200, 2).unwrap();
    m.record_block(300, 4).unwrap();
    advance(&m);
    let s = m.get_epoch_summary(0).unwrap();
    assert_eq!(s.blocks, 3);
    assert_eq!(s.fees, 600);
    assert_eq!(s.transactions, 7);
    assert!(!s.finalized); // no checkpoint was set on EpochInfo.checkpoint
}
