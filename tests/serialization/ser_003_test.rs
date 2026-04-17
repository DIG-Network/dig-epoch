/// SER-003 — Round-trip integrity for all serializable types.
///
/// Normative: docs/requirements/domains/serialization/NORMATIVE.md §SER-003
/// Spec ref:  docs/resources/SPEC.md §11.3
use chia_protocol::Bytes32;
use dig_epoch::types::checkpoint_competition::{CheckpointCompetition, CompetitionStatus};
use dig_epoch::types::dfsp::DfspCloseSnapshot;
use dig_epoch::types::epoch_info::EpochInfo;
use dig_epoch::types::epoch_summary::EpochSummary;
use dig_epoch::types::reward::RewardDistribution;
use dig_epoch::types::verification::EpochCheckpointData;

fn b(x: u8) -> Bytes32 {
    Bytes32::new([x; 32])
}

/// EpochInfo round-trip preserves all fields.
#[test]
fn test_epoch_info_roundtrip() {
    let mut info = EpochInfo::new(7, 100, 50, b(1));
    info.record_block(250, 3);
    info.record_block(100, 1);
    info.collateral_registry_root = b(9);
    let bytes = info.to_bytes();
    let decoded = EpochInfo::from_bytes(&bytes).unwrap();
    assert_eq!(decoded.epoch, 7);
    assert_eq!(decoded.start_l1_height, 100);
    assert_eq!(decoded.start_l2_height, 50);
    assert_eq!(decoded.start_state_root, b(1));
    assert_eq!(decoded.blocks_produced, 2);
    assert_eq!(decoded.total_fees, 350);
    assert_eq!(decoded.total_transactions, 4);
    assert_eq!(decoded.collateral_registry_root, b(9));
}

/// EpochSummary round-trip preserves all fields.
#[test]
fn test_epoch_summary_roundtrip() {
    let mut info = EpochInfo::new(2, 100, 30, b(2));
    info.record_block(1_000, 10);
    info.dfsp_issuance_total = 555;
    info.active_node_count = 4;
    let summary: EpochSummary = info.into();
    let bytes = summary.to_bytes();
    let decoded = EpochSummary::from_bytes(&bytes).unwrap();
    assert_eq!(decoded.epoch, 2);
    assert_eq!(decoded.blocks, 1);
    assert_eq!(decoded.fees, 1_000);
    assert_eq!(decoded.transactions, 10);
    assert_eq!(decoded.dfsp_issuance_total, 555);
    assert_eq!(decoded.active_node_count, 4);
}

/// DfspCloseSnapshot round-trip.
#[test]
fn test_dfsp_close_snapshot_roundtrip() {
    let snap = DfspCloseSnapshot {
        collateral_registry_root: b(1),
        cid_state_root: b(2),
        node_registry_root: b(3),
        namespace_epoch_root: b(4),
        dfsp_issuance_total: 9_000,
        active_cid_count: 7,
        active_node_count: 11,
    };
    let bytes = snap.to_bytes();
    let decoded = DfspCloseSnapshot::from_bytes(&bytes).unwrap();
    assert_eq!(decoded.collateral_registry_root, b(1));
    assert_eq!(decoded.cid_state_root, b(2));
    assert_eq!(decoded.dfsp_issuance_total, 9_000);
    assert_eq!(decoded.active_cid_count, 7);
    assert_eq!(decoded.active_node_count, 11);
}

/// CheckpointCompetition round-trip (Pending state).
#[test]
fn test_checkpoint_competition_roundtrip_pending() {
    let c = CheckpointCompetition::new(42);
    let bytes = c.to_bytes();
    let decoded = CheckpointCompetition::from_bytes(&bytes).unwrap();
    assert_eq!(decoded.epoch, 42);
    assert_eq!(decoded.status, CompetitionStatus::Pending);
    assert!(decoded.submissions.is_empty());
    assert!(decoded.current_winner.is_none());
}

/// CheckpointCompetition round-trip (Finalized state).
#[test]
fn test_checkpoint_competition_roundtrip_finalized() {
    let mut c = CheckpointCompetition::new(3);
    c.status = CompetitionStatus::Finalized {
        winner_hash: b(0xAB),
        l1_height: 2000,
    };
    let bytes = c.to_bytes();
    let decoded = CheckpointCompetition::from_bytes(&bytes).unwrap();
    match decoded.status {
        CompetitionStatus::Finalized {
            winner_hash,
            l1_height,
        } => {
            assert_eq!(winner_hash, b(0xAB));
            assert_eq!(l1_height, 2000);
        }
        _ => panic!("expected Finalized"),
    }
}

/// RewardDistribution round-trip.
#[test]
fn test_reward_distribution_roundtrip() {
    let d = RewardDistribution {
        epoch: 9,
        proposer_reward: 123,
        attester_reward: 456,
        ef_spawner_reward: 7,
        score_submitter_reward: 8,
        finalizer_reward: 9,
        proposer_fee_share: 10,
        burned_fees: 11,
    };
    let bytes = d.to_bytes();
    let decoded = RewardDistribution::from_bytes(&bytes).unwrap();
    assert_eq!(decoded.epoch, 9);
    assert_eq!(decoded.proposer_reward, 123);
    assert_eq!(decoded.attester_reward, 456);
    assert_eq!(decoded.ef_spawner_reward, 7);
    assert_eq!(decoded.score_submitter_reward, 8);
    assert_eq!(decoded.finalizer_reward, 9);
    assert_eq!(decoded.proposer_fee_share, 10);
    assert_eq!(decoded.burned_fees, 11);
}

/// EpochCheckpointData round-trip and signing_digest stability.
#[test]
fn test_epoch_checkpoint_data_roundtrip() {
    let d = EpochCheckpointData {
        network_id: b(0xA1),
        epoch: 12,
        block_root: b(2),
        state_root: b(3),
        withdrawals_root: b(4),
        checkpoint_hash: b(5),
    };
    let bytes = d.to_bytes();
    let decoded = EpochCheckpointData::from_bytes(&bytes).unwrap();
    assert_eq!(decoded, d);
    assert_eq!(decoded.signing_digest(), d.signing_digest());
}

/// Repeated round-trips are stable.
#[test]
fn test_repeated_roundtrip_stable() {
    let info = EpochInfo::new(1, 100, 10, b(1));
    let a = info.to_bytes();
    let decoded = EpochInfo::from_bytes(&a).unwrap();
    let b = decoded.to_bytes();
    assert_eq!(a, b);
}
