/// MGR-002 — record_block increments blocks_produced, total_fees, total_transactions.
///
/// Normative: docs/requirements/domains/epoch_manager/NORMATIVE.md §MGR-002
/// Spec ref:  docs/resources/SPEC.md §6.4
use chia_protocol::Bytes32;
use dig_epoch::error::EpochError;
use dig_epoch::manager::EpochManager;
use dig_epoch::types::epoch_phase::EpochPhase;

fn mgr() -> EpochManager {
    EpochManager::new(Bytes32::new([0u8; 32]), 100, Bytes32::new([0u8; 32]))
}

/// Single call increments counters by (1, fees, tx_count).
#[test]
fn test_single_record() {
    let m = mgr();
    m.record_block(250, 3).unwrap();
    let info = m.current_epoch_info();
    assert_eq!(info.blocks_produced, 1);
    assert_eq!(info.total_fees, 250);
    assert_eq!(info.total_transactions, 3);
}

/// Multiple calls accumulate additively.
#[test]
fn test_multiple_records_accumulate() {
    let m = mgr();
    m.record_block(100, 1).unwrap();
    m.record_block(200, 4).unwrap();
    m.record_block(0, 0).unwrap();
    let info = m.current_epoch_info();
    assert_eq!(info.blocks_produced, 3);
    assert_eq!(info.total_fees, 300);
    assert_eq!(info.total_transactions, 5);
}

/// Zero fees/tx still increments blocks_produced.
#[test]
fn test_zero_values_still_count_block() {
    let m = mgr();
    m.record_block(0, 0).unwrap();
    assert_eq!(m.current_epoch_info().blocks_produced, 1);
}

/// Rejects with PhaseMismatch outside BlockProduction.
#[test]
fn test_rejects_outside_block_production() {
    let m = mgr();
    m.update_phase(116); // → Checkpoint
    let e = m.record_block(0, 0).unwrap_err();
    assert!(matches!(
        e,
        EpochError::PhaseMismatch {
            expected: EpochPhase::BlockProduction,
            got: EpochPhase::Checkpoint
        }
    ));
}
