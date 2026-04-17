/// STR-004 — EpochManager::new() constructor post-conditions.
///
/// Normative: docs/requirements/domains/crate_structure/NORMATIVE.md §STR-004
/// Spec ref:  docs/resources/SPEC.md §6.2
use chia_protocol::Bytes32;
use dig_epoch::constants::{EMPTY_ROOT, EPOCH_L1_BLOCKS, GENESIS_HEIGHT};
use dig_epoch::manager::EpochManager;
use dig_epoch::types::epoch_phase::EpochPhase;

fn nid() -> Bytes32 {
    Bytes32::new([0xA1; 32])
}
fn root() -> Bytes32 {
    Bytes32::new([0xBB; 32])
}

fn mgr() -> EpochManager {
    EpochManager::new(nid(), 100, root())
}

#[test]
fn test_new_epoch_zero() {
    assert_eq!(mgr().current_epoch(), 0);
}

#[test]
fn test_new_phase_block_production() {
    assert_eq!(mgr().current_phase(), EpochPhase::BlockProduction);
}

#[test]
fn test_new_epoch_info_fields() {
    let info = mgr().current_epoch_info();
    assert_eq!(info.epoch, 0);
    assert_eq!(info.start_l1_height, 100);
    assert_eq!(info.end_l1_height, 100 + EPOCH_L1_BLOCKS);
    assert_eq!(info.start_l2_height, GENESIS_HEIGHT);
    assert_eq!(info.blocks_produced, 0);
    assert_eq!(info.total_fees, 0);
    assert_eq!(info.total_transactions, 0);
    assert!(info.checkpoint.is_none());
    assert_eq!(info.start_state_root, root());
    assert_eq!(info.collateral_registry_root, EMPTY_ROOT);
    assert_eq!(info.cid_state_root, EMPTY_ROOT);
    assert_eq!(info.node_registry_root, EMPTY_ROOT);
    assert_eq!(info.namespace_epoch_root, EMPTY_ROOT);
    assert_eq!(info.dfsp_issuance_total, 0);
    assert_eq!(info.active_cid_count, 0);
    assert_eq!(info.active_node_count, 0);
}

#[test]
fn test_new_empty_history() {
    assert!(mgr().get_epoch_summary(0).is_none());
    assert!(mgr().get_epoch_summary(999).is_none());
}

#[test]
fn test_new_empty_competitions_for_nonexistent() {
    // Current competition (epoch 0) exists in Pending state; epochs not tracked
    // yet return None.
    let m = mgr();
    assert!(m.get_competition(1).is_none());
    assert!(m.get_competition(999).is_none());
}

#[test]
fn test_new_empty_rewards() {
    assert!(mgr().get_rewards(0).is_none());
    assert!(mgr().get_rewards(999).is_none());
}

#[test]
fn test_new_genesis_l1_height() {
    assert_eq!(mgr().genesis_l1_height(), 100);
}

#[test]
fn test_new_network_id() {
    assert_eq!(mgr().network_id(), nid());
}

#[test]
fn test_new_state_root() {
    assert_eq!(mgr().current_epoch_info().start_state_root, root());
}

#[test]
fn test_new_l1_range() {
    let info = mgr().current_epoch_info();
    assert_eq!(info.end_l1_height, 100 + EPOCH_L1_BLOCKS);
}

/// Varying genesis_l1_height shifts the L1 window.
#[test]
fn test_new_varying_genesis() {
    for g in [0u32, 1, 100, 1_000_000] {
        let m = EpochManager::new(nid(), g, root());
        let info = m.current_epoch_info();
        assert_eq!(info.start_l1_height, g);
        assert_eq!(info.end_l1_height, g + EPOCH_L1_BLOCKS);
    }
}
