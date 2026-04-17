/// MGR-006 — set_current_epoch_dfsp_close_snapshot applies DFSP close in Finalization.
///
/// Normative: docs/requirements/domains/epoch_manager/NORMATIVE.md §MGR-006
/// Spec ref:  docs/resources/SPEC.md §6.6
use chia_protocol::Bytes32;
use dig_epoch::error::EpochError;
use dig_epoch::manager::EpochManager;
use dig_epoch::types::checkpoint_competition::{CheckpointCompetition, CompetitionStatus};
use dig_epoch::types::dfsp::DfspCloseSnapshot;
use dig_epoch::types::epoch_phase::EpochPhase;

fn nid() -> Bytes32 {
    Bytes32::new([0u8; 32])
}
fn r(x: u8) -> Bytes32 {
    Bytes32::new([x; 32])
}

fn sample_snapshot() -> DfspCloseSnapshot {
    DfspCloseSnapshot {
        collateral_registry_root: r(1),
        cid_state_root: r(2),
        node_registry_root: r(3),
        namespace_epoch_root: r(4),
        dfsp_issuance_total: 9_999,
        active_cid_count: 7,
        active_node_count: 11,
    }
}

fn mgr_in_finalization() -> EpochManager {
    let m = EpochManager::new(nid(), 100, r(0));
    m.update_phase(124); // → Finalization
    assert_eq!(m.current_phase(), EpochPhase::Finalization);
    m
}

/// Setting in Finalization phase succeeds and copies all 7 fields.
#[test]
fn test_set_in_finalization_copies_all_fields() {
    let m = mgr_in_finalization();
    m.set_current_epoch_dfsp_close_snapshot(sample_snapshot())
        .unwrap();
    let info = m.current_epoch_info();
    assert_eq!(info.collateral_registry_root, r(1));
    assert_eq!(info.cid_state_root, r(2));
    assert_eq!(info.node_registry_root, r(3));
    assert_eq!(info.namespace_epoch_root, r(4));
    assert_eq!(info.dfsp_issuance_total, 9_999);
    assert_eq!(info.active_cid_count, 7);
    assert_eq!(info.active_node_count, 11);
}

/// Reject in BlockProduction.
#[test]
fn test_reject_in_block_production() {
    let m = EpochManager::new(nid(), 100, r(0));
    let e = m
        .set_current_epoch_dfsp_close_snapshot(sample_snapshot())
        .unwrap_err();
    assert!(matches!(
        e,
        EpochError::PhaseMismatch {
            expected: EpochPhase::Finalization,
            got: EpochPhase::BlockProduction
        }
    ));
}

/// Reject in Checkpoint.
#[test]
fn test_reject_in_checkpoint() {
    let m = EpochManager::new(nid(), 100, r(0));
    m.update_phase(116); // → Checkpoint
    let e = m
        .set_current_epoch_dfsp_close_snapshot(sample_snapshot())
        .unwrap_err();
    assert!(matches!(
        e,
        EpochError::PhaseMismatch {
            expected: EpochPhase::Finalization,
            got: EpochPhase::Checkpoint
        }
    ));
}

/// Reject in Complete.
#[test]
fn test_reject_in_complete() {
    let m = EpochManager::new(nid(), 100, r(0));
    m.update_phase(132); // → Complete
    let e = m
        .set_current_epoch_dfsp_close_snapshot(sample_snapshot())
        .unwrap_err();
    assert!(matches!(
        e,
        EpochError::PhaseMismatch {
            expected: EpochPhase::Finalization,
            got: EpochPhase::Complete
        }
    ));
}

/// Second call overwrites first.
#[test]
fn test_overwrite_snapshot() {
    let m = mgr_in_finalization();
    m.set_current_epoch_dfsp_close_snapshot(sample_snapshot())
        .unwrap();
    let snap2 = DfspCloseSnapshot {
        collateral_registry_root: r(99),
        cid_state_root: r(99),
        node_registry_root: r(99),
        namespace_epoch_root: r(99),
        dfsp_issuance_total: 1,
        active_cid_count: 1,
        active_node_count: 1,
    };
    m.set_current_epoch_dfsp_close_snapshot(snap2).unwrap();
    let info = m.current_epoch_info();
    assert_eq!(info.dfsp_issuance_total, 1);
    assert_eq!(info.active_cid_count, 1);
    assert_eq!(info.collateral_registry_root, r(99));
}

/// Snapshot values survive archival into EpochSummary via advance_epoch.
#[test]
fn test_snapshot_preserved_in_summary() {
    let m = mgr_in_finalization();
    m.set_current_epoch_dfsp_close_snapshot(sample_snapshot())
        .unwrap();
    // Move to Complete + finalize competition so we can advance.
    m.update_phase(132);
    let mut c = CheckpointCompetition::new(0);
    c.status = CompetitionStatus::Finalized {
        winner_hash: r(0xAB),
        l1_height: 132,
    };
    m.__set_competition_for_test(c);
    m.advance_epoch(132, r(1)).unwrap();
    let s = m.get_epoch_summary(0).unwrap();
    assert_eq!(s.collateral_registry_root, r(1));
    assert_eq!(s.cid_state_root, r(2));
    assert_eq!(s.dfsp_issuance_total, 9_999);
    assert_eq!(s.active_node_count, 11);
}
