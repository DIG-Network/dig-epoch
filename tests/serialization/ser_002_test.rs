/// SER-002 — to_bytes/from_bytes convention for all 6 types.
///
/// Normative: docs/requirements/domains/serialization/NORMATIVE.md §SER-002
/// Spec ref:  docs/resources/SPEC.md §11.2
use chia_protocol::Bytes32;
use dig_epoch::error::EpochError;
use dig_epoch::types::checkpoint_competition::CheckpointCompetition;
use dig_epoch::types::dfsp::DfspCloseSnapshot;
use dig_epoch::types::epoch_info::EpochInfo;
use dig_epoch::types::epoch_summary::EpochSummary;
use dig_epoch::types::reward::RewardDistribution;
use dig_epoch::types::verification::EpochCheckpointData;

fn b(x: u8) -> Bytes32 {
    Bytes32::new([x; 32])
}

// -- to_bytes returns non-empty Vec<u8> for each type --

#[test]
fn test_to_bytes_epoch_info() {
    let info = EpochInfo::new(0, 100, 1, b(0));
    let bytes = info.to_bytes();
    assert!(!bytes.is_empty());
}

#[test]
fn test_to_bytes_all_types() {
    let info = EpochInfo::new(0, 100, 1, b(0));
    assert!(!info.to_bytes().is_empty());
    let summary: EpochSummary = info.into();
    assert!(!summary.to_bytes().is_empty());
    let snap = DfspCloseSnapshot {
        collateral_registry_root: b(1),
        cid_state_root: b(2),
        node_registry_root: b(3),
        namespace_epoch_root: b(4),
        dfsp_issuance_total: 0,
        active_cid_count: 0,
        active_node_count: 0,
    };
    assert!(!snap.to_bytes().is_empty());
    let comp = CheckpointCompetition::new(0);
    assert!(!comp.to_bytes().is_empty());
    let reward = RewardDistribution {
        epoch: 0,
        proposer_reward: 0,
        attester_reward: 0,
        ef_spawner_reward: 0,
        score_submitter_reward: 0,
        finalizer_reward: 0,
        proposer_fee_share: 0,
        burned_fees: 0,
    };
    assert!(!reward.to_bytes().is_empty());
    let data = EpochCheckpointData {
        network_id: b(0),
        epoch: 0,
        block_root: b(0),
        state_root: b(0),
        withdrawals_root: b(0),
        checkpoint_hash: b(0),
    };
    assert!(!data.to_bytes().is_empty());
}

// -- from_bytes valid: Ok --

#[test]
fn test_from_bytes_epoch_info_valid() {
    let info = EpochInfo::new(7, 100, 200, b(5));
    let bytes = info.to_bytes();
    let decoded = EpochInfo::from_bytes(&bytes).unwrap();
    assert_eq!(decoded.epoch, 7);
    assert_eq!(decoded.start_l1_height, 100);
}

// -- from_bytes truncated: Err(InvalidData) --

#[test]
fn test_from_bytes_truncated() {
    let info = EpochInfo::new(1, 100, 1, b(0));
    let bytes = info.to_bytes();
    let truncated = &bytes[..bytes.len() / 2];
    match EpochInfo::from_bytes(truncated) {
        Err(EpochError::InvalidData(_)) => {}
        other => panic!("expected InvalidData, got {other:?}"),
    }
}

// -- from_bytes empty: Err(InvalidData) --

#[test]
fn test_from_bytes_empty() {
    match EpochInfo::from_bytes(&[]) {
        Err(EpochError::InvalidData(_)) => {}
        other => panic!("expected InvalidData, got {other:?}"),
    }
    match EpochSummary::from_bytes(&[]) {
        Err(EpochError::InvalidData(_)) => {}
        other => panic!("expected InvalidData, got {other:?}"),
    }
    match DfspCloseSnapshot::from_bytes(&[]) {
        Err(EpochError::InvalidData(_)) => {}
        other => panic!("expected InvalidData, got {other:?}"),
    }
    match CheckpointCompetition::from_bytes(&[]) {
        Err(EpochError::InvalidData(_)) => {}
        other => panic!("expected InvalidData, got {other:?}"),
    }
    match RewardDistribution::from_bytes(&[]) {
        Err(EpochError::InvalidData(_)) => {}
        other => panic!("expected InvalidData, got {other:?}"),
    }
    match EpochCheckpointData::from_bytes(&[]) {
        Err(EpochError::InvalidData(_)) => {}
        other => panic!("expected InvalidData, got {other:?}"),
    }
}

// -- from_bytes garbage: Err(InvalidData) --

#[test]
fn test_from_bytes_garbage() {
    let garbage = vec![0xFFu8; 7];
    assert!(matches!(
        EpochInfo::from_bytes(&garbage),
        Err(EpochError::InvalidData(_))
    ));
}
