/// MGR-008 — Core instance methods and accessors.
///
/// Normative: docs/requirements/domains/epoch_manager/NORMATIVE.md §MGR-008
/// Spec ref:  docs/resources/SPEC.md §6.3, §6.8, §6.9
use chia_protocol::Bytes32;
use dig_epoch::arithmetic::l1_range_for_epoch as free_l1_range;
use dig_epoch::constants::EPOCH_L1_BLOCKS;
use dig_epoch::manager::EpochManager;
use dig_epoch::types::epoch_phase::EpochPhase;
use dig_epoch::types::reward::RewardDistribution;

fn nid() -> Bytes32 {
    Bytes32::new([1u8; 32])
}
fn r(x: u8) -> Bytes32 {
    Bytes32::new([x; 32])
}

/// current_epoch() returns 0 on a fresh manager.
#[test]
fn test_current_epoch_zero() {
    let m = EpochManager::new(nid(), 100, r(0));
    assert_eq!(m.current_epoch(), 0);
}

/// current_epoch_info() returns EpochInfo for the current epoch.
#[test]
fn test_current_epoch_info() {
    let m = EpochManager::new(nid(), 100, r(7));
    let info = m.current_epoch_info();
    assert_eq!(info.epoch, 0);
    assert_eq!(info.start_state_root, r(7));
}

/// current_phase() returns BlockProduction at start.
#[test]
fn test_current_phase_initial() {
    let m = EpochManager::new(nid(), 100, r(0));
    assert_eq!(m.current_phase(), EpochPhase::BlockProduction);
}

/// genesis_l1_height() returns the value passed at construction.
#[test]
fn test_genesis_l1_height_accessor() {
    let m = EpochManager::new(nid(), 424242, r(0));
    assert_eq!(m.genesis_l1_height(), 424242);
}

/// network_id() returns the value passed at construction.
#[test]
fn test_network_id_accessor() {
    let m = EpochManager::new(nid(), 100, r(0));
    assert_eq!(m.network_id(), nid());
}

/// epoch_for_l1_height maps heights correctly.
#[test]
fn test_epoch_for_l1_height() {
    let m = EpochManager::new(nid(), 100, r(0));
    assert_eq!(m.epoch_for_l1_height(50), 0); // before genesis
    assert_eq!(m.epoch_for_l1_height(100), 0); // at genesis
    assert_eq!(m.epoch_for_l1_height(100 + EPOCH_L1_BLOCKS - 1), 0);
    assert_eq!(m.epoch_for_l1_height(100 + EPOCH_L1_BLOCKS), 1);
    assert_eq!(m.epoch_for_l1_height(100 + 5 * EPOCH_L1_BLOCKS), 5);
}

/// l1_range_for_epoch(e) matches the free-function result for the manager's
/// genesis_l1_height.
#[test]
fn test_l1_range_for_epoch() {
    let m = EpochManager::new(nid(), 100, r(0));
    for e in 0u64..5 {
        assert_eq!(m.l1_range_for_epoch(e), free_l1_range(100, e));
    }
}

/// store_rewards + get_rewards round-trip.
#[test]
fn test_store_and_get_rewards() {
    let m = EpochManager::new(nid(), 100, r(0));
    let d = RewardDistribution {
        epoch: 42,
        proposer_reward: 1,
        attester_reward: 2,
        ef_spawner_reward: 3,
        score_submitter_reward: 4,
        finalizer_reward: 5,
        proposer_fee_share: 6,
        burned_fees: 7,
    };
    m.store_rewards(d);
    let got = m.get_rewards(42).unwrap();
    assert_eq!(got.proposer_reward, 1);
    assert_eq!(got.attester_reward, 2);
    assert_eq!(got.finalizer_reward, 5);
    assert_eq!(got.burned_fees, 7);
    assert!(m.get_rewards(43).is_none());
}
