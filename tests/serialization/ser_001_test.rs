/// SER-001 — All 6 serializable epoch types use bincode.
///
/// Normative: docs/requirements/domains/serialization/NORMATIVE.md §SER-001
/// Spec ref:  docs/resources/SPEC.md §11.1
use chia_protocol::Bytes32;
use dig_epoch::types::checkpoint_competition::CheckpointCompetition;
use dig_epoch::types::dfsp::DfspCloseSnapshot;
use dig_epoch::types::epoch_info::EpochInfo;
use dig_epoch::types::epoch_summary::EpochSummary;
use dig_epoch::types::reward::RewardDistribution;
use dig_epoch::types::verification::EpochCheckpointData;

fn b(x: u8) -> Bytes32 {
    Bytes32::new([x; 32])
}

fn sample_info() -> EpochInfo {
    EpochInfo::new(3, 100, 97, b(0xAB))
}

fn sample_summary() -> EpochSummary {
    sample_info().into()
}

fn sample_snap() -> DfspCloseSnapshot {
    DfspCloseSnapshot {
        collateral_registry_root: b(1),
        cid_state_root: b(2),
        node_registry_root: b(3),
        namespace_epoch_root: b(4),
        dfsp_issuance_total: 500,
        active_cid_count: 7,
        active_node_count: 11,
    }
}

fn sample_reward() -> RewardDistribution {
    RewardDistribution {
        epoch: 9,
        proposer_reward: 100,
        attester_reward: 800,
        ef_spawner_reward: 30,
        score_submitter_reward: 40,
        finalizer_reward: 30,
        proposer_fee_share: 50,
        burned_fees: 50,
    }
}

fn sample_competition() -> CheckpointCompetition {
    CheckpointCompetition::new(5)
}

fn sample_checkpoint_data() -> EpochCheckpointData {
    EpochCheckpointData {
        network_id: b(0xA1),
        epoch: 1,
        block_root: b(2),
        state_root: b(3),
        withdrawals_root: b(4),
        checkpoint_hash: b(5),
    }
}

#[test]
fn test_epoch_info_bincode() {
    let bytes = bincode::serialize(&sample_info()).unwrap();
    assert!(!bytes.is_empty());
}

#[test]
fn test_epoch_summary_bincode() {
    let bytes = bincode::serialize(&sample_summary()).unwrap();
    assert!(!bytes.is_empty());
}

#[test]
fn test_dfsp_close_snapshot_bincode() {
    let bytes = bincode::serialize(&sample_snap()).unwrap();
    assert!(!bytes.is_empty());
}

#[test]
fn test_checkpoint_competition_bincode() {
    let bytes = bincode::serialize(&sample_competition()).unwrap();
    assert!(!bytes.is_empty());
}

#[test]
fn test_reward_distribution_bincode() {
    let bytes = bincode::serialize(&sample_reward()).unwrap();
    assert!(!bytes.is_empty());
}

#[test]
fn test_epoch_checkpoint_data_bincode() {
    let bytes = bincode::serialize(&sample_checkpoint_data()).unwrap();
    assert!(!bytes.is_empty());
}

/// Bincode output is smaller than JSON for EpochInfo.
#[test]
fn test_no_schema_overhead() {
    let info = sample_info();
    let bincode_bytes = bincode::serialize(&info).unwrap();
    let json_bytes = serde_json::to_vec(&info).unwrap();
    assert!(
        bincode_bytes.len() < json_bytes.len(),
        "bincode {} should be smaller than JSON {}",
        bincode_bytes.len(),
        json_bytes.len()
    );
}
