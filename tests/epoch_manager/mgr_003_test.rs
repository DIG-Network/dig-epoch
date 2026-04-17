/// MGR-003 — set_current_epoch_chain_totals overwrites counters without phase check.
///
/// Normative: docs/requirements/domains/epoch_manager/NORMATIVE.md §MGR-003
/// Spec ref:  docs/resources/SPEC.md §6.4
use chia_protocol::Bytes32;
use dig_epoch::manager::EpochManager;

fn mgr() -> EpochManager {
    EpochManager::new(Bytes32::new([0u8; 32]), 100, Bytes32::new([0u8; 32]))
}

/// Fresh epoch: set values from zero.
#[test]
fn test_set_on_fresh_epoch() {
    let m = mgr();
    m.set_current_epoch_chain_totals(10, 500, 50);
    let info = m.current_epoch_info();
    assert_eq!(info.blocks_produced, 10);
    assert_eq!(info.total_fees, 500);
    assert_eq!(info.total_transactions, 50);
}

/// Overwrites (not increments) previously recorded values.
#[test]
fn test_overwrites_recorded_values() {
    let m = mgr();
    m.record_block(100, 2).unwrap();
    m.record_block(200, 4).unwrap();
    m.set_current_epoch_chain_totals(1, 1, 1);
    let info = m.current_epoch_info();
    assert_eq!(info.blocks_produced, 1);
    assert_eq!(info.total_fees, 1);
    assert_eq!(info.total_transactions, 1);
}

/// Set to zero wipes counters.
#[test]
fn test_set_to_zero() {
    let m = mgr();
    m.record_block(100, 2).unwrap();
    m.set_current_epoch_chain_totals(0, 0, 0);
    let info = m.current_epoch_info();
    assert_eq!(info.blocks_produced, 0);
    assert_eq!(info.total_fees, 0);
    assert_eq!(info.total_transactions, 0);
}

/// Idempotent: repeated calls with same args produce same state.
#[test]
fn test_idempotent() {
    let m = mgr();
    m.set_current_epoch_chain_totals(7, 77, 777);
    m.set_current_epoch_chain_totals(7, 77, 777);
    let info = m.current_epoch_info();
    assert_eq!(info.blocks_produced, 7);
    assert_eq!(info.total_fees, 77);
    assert_eq!(info.total_transactions, 777);
}

/// Works in non-BlockProduction phase (no phase restriction).
#[test]
fn test_no_phase_restriction() {
    let m = mgr();
    m.update_phase(116); // → Checkpoint
    m.set_current_epoch_chain_totals(5, 50, 500);
    let info = m.current_epoch_info();
    assert_eq!(info.blocks_produced, 5);
    assert_eq!(info.total_fees, 50);
    assert_eq!(info.total_transactions, 500);
}

/// Fields are independent: each argument maps to its own field.
#[test]
fn test_fields_independent() {
    let m = mgr();
    m.set_current_epoch_chain_totals(1000, 2, 3);
    let info = m.current_epoch_info();
    assert_eq!(info.blocks_produced, 1000);
    assert_eq!(info.total_fees, 2);
    assert_eq!(info.total_transactions, 3);
}
