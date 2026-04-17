/// PHS-004 — Phase boundary enforcement (PhaseMismatch errors)
///
/// Normative: docs/requirements/domains/phase_machine/NORMATIVE.md §PHS-004
/// Spec ref:  docs/resources/SPEC.md §4.1
use chia_protocol::Bytes32;
use dig_epoch::error::EpochError;
use dig_epoch::manager::EpochManager;
use dig_epoch::types::epoch_phase::EpochPhase;

fn zero_root() -> Bytes32 {
    Bytes32::new([0u8; 32])
}

fn make_manager() -> EpochManager {
    EpochManager::new(zero_root(), 100, zero_root())
}

// -- record_block --

/// record_block succeeds in BlockProduction phase.
#[test]
fn test_record_block_in_block_production() {
    let mgr = make_manager();
    assert!(mgr.record_block(500, 10).is_ok());
}

/// record_block rejects with PhaseMismatch in Checkpoint phase.
#[test]
fn test_record_block_in_checkpoint() {
    let mgr = make_manager();
    mgr.update_phase(116); // → Checkpoint
    let err = mgr.record_block(500, 10).unwrap_err();
    match err {
        EpochError::PhaseMismatch { expected, got } => {
            assert_eq!(expected, EpochPhase::BlockProduction);
            assert_eq!(got, EpochPhase::Checkpoint);
        }
        other => panic!("unexpected error: {other:?}"),
    }
}

/// record_block rejects with PhaseMismatch in Finalization phase.
#[test]
fn test_record_block_in_finalization() {
    let mgr = make_manager();
    mgr.update_phase(124); // → Finalization
    let err = mgr.record_block(0, 0).unwrap_err();
    match err {
        EpochError::PhaseMismatch { expected, got } => {
            assert_eq!(expected, EpochPhase::BlockProduction);
            assert_eq!(got, EpochPhase::Finalization);
        }
        other => panic!("unexpected error: {other:?}"),
    }
}

/// record_block rejects with PhaseMismatch in Complete phase.
#[test]
fn test_record_block_in_complete() {
    let mgr = make_manager();
    mgr.update_phase(132); // → Complete
    let err = mgr.record_block(0, 0).unwrap_err();
    match err {
        EpochError::PhaseMismatch { expected, got } => {
            assert_eq!(expected, EpochPhase::BlockProduction);
            assert_eq!(got, EpochPhase::Complete);
        }
        other => panic!("unexpected error: {other:?}"),
    }
}

// -- submit_checkpoint --

/// submit_checkpoint succeeds in Checkpoint phase.
#[test]
fn test_submit_checkpoint_in_checkpoint() {
    let mgr = make_manager();
    mgr.update_phase(116); // → Checkpoint
    assert!(mgr.submit_checkpoint().is_ok());
}

/// submit_checkpoint rejects with PhaseMismatch in BlockProduction.
#[test]
fn test_submit_checkpoint_in_block_production() {
    let mgr = make_manager();
    let err = mgr.submit_checkpoint().unwrap_err();
    match err {
        EpochError::PhaseMismatch { expected, got } => {
            assert_eq!(expected, EpochPhase::Checkpoint);
            assert_eq!(got, EpochPhase::BlockProduction);
        }
        other => panic!("unexpected error: {other:?}"),
    }
}

// -- finalize_competition --

/// finalize_competition succeeds in Finalization phase.
#[test]
fn test_finalize_competition_in_finalization() {
    let mgr = make_manager();
    mgr.update_phase(124); // → Finalization
    assert!(mgr.finalize_competition().is_ok());
}

/// finalize_competition rejects with PhaseMismatch in Checkpoint.
#[test]
fn test_finalize_competition_in_checkpoint() {
    let mgr = make_manager();
    mgr.update_phase(116); // → Checkpoint
    let err = mgr.finalize_competition().unwrap_err();
    match err {
        EpochError::PhaseMismatch { expected, got } => {
            assert_eq!(expected, EpochPhase::Finalization);
            assert_eq!(got, EpochPhase::Checkpoint);
        }
        other => panic!("unexpected error: {other:?}"),
    }
}
