/// MGR-005 — Query methods: get_epoch_info, get_epoch_summary, recent_summaries,
///           total_stats, get_rewards.
///
/// Normative: docs/requirements/domains/epoch_manager/NORMATIVE.md §MGR-005
/// Spec ref:  docs/resources/SPEC.md §6.9
use chia_protocol::Bytes32;
use dig_epoch::manager::EpochManager;
use dig_epoch::types::checkpoint_competition::{CheckpointCompetition, CompetitionStatus};
use dig_epoch::types::epoch_phase::EpochPhase;
use dig_epoch::types::reward::RewardDistribution;

fn nid() -> Bytes32 {
    Bytes32::new([0u8; 32])
}
fn root(x: u8) -> Bytes32 {
    Bytes32::new([x; 32])
}

fn finalize(m: &EpochManager) {
    let e = m.current_epoch();
    let mut c = CheckpointCompetition::new(e);
    c.status = CompetitionStatus::Finalized {
        winner_hash: root(0xAB),
        l1_height: 132,
    };
    m.__set_competition_for_test(c);
}

fn advance_one(m: &EpochManager) {
    let e = m.current_epoch();
    let (_, end_l1) = m.l1_range_for_epoch(e);
    m.update_phase(end_l1 + 1);
    finalize(m);
    m.advance_epoch(end_l1 + 1, root(0)).unwrap();
}

/// get_epoch_info returns a clone (not affected by subsequent mutation).
#[test]
fn test_get_epoch_info_is_snapshot() {
    let m = EpochManager::new(nid(), 100, root(0));
    m.record_block(100, 1).unwrap();
    let snap = m.get_epoch_info();
    m.record_block(200, 2).unwrap();
    assert_eq!(snap.blocks_produced, 1);
    assert_eq!(snap.total_fees, 100);
    // Current state has advanced past the snapshot.
    assert_eq!(m.current_epoch_info().blocks_produced, 2);
}

/// get_epoch_summary returns None for unknown epoch.
#[test]
fn test_get_epoch_summary_unknown() {
    let m = EpochManager::new(nid(), 100, root(0));
    assert!(m.get_epoch_summary(999).is_none());
}

/// get_epoch_summary returns Some for archived epoch.
#[test]
fn test_get_epoch_summary_known() {
    let m = EpochManager::new(nid(), 100, root(0));
    m.record_block(10, 1).unwrap();
    advance_one(&m);
    let s = m.get_epoch_summary(0).unwrap();
    assert_eq!(s.epoch, 0);
    assert_eq!(s.blocks, 1);
    assert_eq!(s.fees, 10);
}

/// recent_summaries returns last N in order.
#[test]
fn test_recent_summaries_tail() {
    let m = EpochManager::new(nid(), 100, root(0));
    for _ in 0..5 {
        advance_one(&m);
    }
    let r = m.recent_summaries(3);
    assert_eq!(r.len(), 3);
    assert_eq!(r[0].epoch, 2);
    assert_eq!(r[1].epoch, 3);
    assert_eq!(r[2].epoch, 4);
}

/// recent_summaries returns fewer than N when history is shorter.
#[test]
fn test_recent_summaries_partial() {
    let m = EpochManager::new(nid(), 100, root(0));
    advance_one(&m);
    advance_one(&m);
    let r = m.recent_summaries(10);
    assert_eq!(r.len(), 2);
}

/// recent_summaries(0) returns empty.
#[test]
fn test_recent_summaries_zero() {
    let m = EpochManager::new(nid(), 100, root(0));
    advance_one(&m);
    assert!(m.recent_summaries(0).is_empty());
}

/// total_stats aggregates summaries + current epoch.
#[test]
fn test_total_stats_aggregation() {
    let m = EpochManager::new(nid(), 100, root(0));
    m.record_block(100, 2).unwrap();
    advance_one(&m); // epoch 0 archived with blocks=1, fees=100, txns=2
    m.record_block(50, 3).unwrap();
    m.record_block(50, 1).unwrap(); // current epoch 1: blocks=2, fees=100, txns=4
    let s = m.total_stats();
    assert_eq!(s.total_epochs, 2); // 1 archived + 1 current
    assert_eq!(s.total_blocks, 3);
    assert_eq!(s.total_fees, 200);
    assert_eq!(s.total_transactions, 6);
}

/// total_stats counts the *current* (not-yet-archived) epoch as finalized once
/// its winning checkpoint has been recorded via finalize_competition. This
/// exercises the `cur.is_finalized()` branch of total_stats, which is distinct
/// from the archived-summary `s.finalized` branch.
#[test]
fn test_total_stats_counts_finalized_current_epoch() {
    use dig_block::{Checkpoint, CheckpointSubmission, PublicKey, Signature, SignerBitmap};

    let m = EpochManager::new(nid(), 100, root(0));
    m.record_block(10, 1).unwrap();

    // Drive into Checkpoint phase and run a one-submission competition.
    m.update_phase(116); // 50% of the 32-block L1 window → Checkpoint
    assert_eq!(m.current_phase(), EpochPhase::Checkpoint);
    m.start_checkpoint_competition().unwrap();
    let mut cp = Checkpoint::new();
    cp.epoch = 0;
    cp.block_count = 1;
    cp.state_root = root(0xCC);
    let sub = CheckpointSubmission::new(
        cp,
        SignerBitmap::new(0),
        Signature::default(),
        PublicKey::default(),
        100,
        0,
    );
    assert!(m.submit_checkpoint(sub).unwrap());

    // Advance to Finalization and finalize — this records the winning checkpoint
    // on the CURRENT EpochInfo, so is_finalized() becomes true without advancing.
    m.update_phase(124); // 75% → Finalization
    assert_eq!(m.current_phase(), EpochPhase::Finalization);
    let winner = m.finalize_competition(0, 124).unwrap();
    assert!(winner.is_some());

    let s = m.total_stats();
    // No archived summaries yet; only the current epoch, which is now finalized.
    assert_eq!(s.total_epochs, 1);
    assert_eq!(s.finalized_epochs, 1);
}

/// get_rewards returns None for unknown epoch and Some after store_rewards.
#[test]
fn test_get_rewards_roundtrip() {
    let m = EpochManager::new(nid(), 100, root(0));
    assert!(m.get_rewards(0).is_none());
    let d = RewardDistribution {
        epoch: 0,
        proposer_reward: 100,
        attester_reward: 800,
        ef_spawner_reward: 30,
        score_submitter_reward: 40,
        finalizer_reward: 30,
        proposer_fee_share: 50,
        burned_fees: 50,
    };
    m.store_rewards(d);
    let got = m.get_rewards(0).unwrap();
    assert_eq!(got.epoch, 0);
    assert_eq!(got.proposer_reward, 100);
    assert_eq!(got.attester_reward, 800);
}

/// current_phase reads without blocking (smoke test for read-lock use).
#[test]
fn test_query_methods_use_read_lock() {
    let m = EpochManager::new(nid(), 100, root(0));
    let _ = m.current_phase();
    let _ = m.current_epoch_info();
    let _ = m.recent_summaries(5);
    // Sanity: phase is BlockProduction initially.
    assert_eq!(m.current_phase(), EpochPhase::BlockProduction);
}
