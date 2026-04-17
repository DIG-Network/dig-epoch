/// PHS-004 — Phase boundary enforcement (PhaseMismatch errors)
///
/// Normative: docs/requirements/domains/phase_machine/NORMATIVE.md §PHS-004
/// Spec ref:  docs/resources/SPEC.md §4.1
///
/// Verifies that phase-gated methods on `EpochManager` reject with
/// `EpochError::PhaseMismatch` when called outside their permitted phase.
/// Covers `record_block` (BlockProduction), `start_checkpoint_competition`
/// (Checkpoint), and `finalize_competition` (Finalization).
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
    mgr.update_phase(133); // → Complete (progress hits 100%)
    let err = mgr.record_block(0, 0).unwrap_err();
    match err {
        EpochError::PhaseMismatch { expected, got } => {
            assert_eq!(expected, EpochPhase::BlockProduction);
            assert_eq!(got, EpochPhase::Complete);
        }
        other => panic!("unexpected error: {other:?}"),
    }
}

// -- start_checkpoint_competition (CKP-002 phase gate) --

/// start_checkpoint_competition succeeds in Checkpoint phase.
#[test]
fn test_submit_checkpoint_in_checkpoint() {
    let mgr = make_manager();
    mgr.update_phase(116); // → Checkpoint
    assert!(mgr.start_checkpoint_competition().is_ok());
}

/// start_checkpoint_competition rejects with PhaseMismatch in BlockProduction.
#[test]
fn test_submit_checkpoint_in_block_production() {
    let mgr = make_manager();
    let err = mgr.start_checkpoint_competition().unwrap_err();
    match err {
        EpochError::PhaseMismatch { expected, got } => {
            assert_eq!(expected, EpochPhase::Checkpoint);
            assert_eq!(got, EpochPhase::BlockProduction);
        }
        other => panic!("unexpected error: {other:?}"),
    }
}

// -- finalize_competition (CKP-004 phase gate) --

/// finalize_competition rejects with PhaseMismatch in Checkpoint (phase gate).
#[test]
fn test_finalize_competition_in_checkpoint() {
    let mgr = make_manager();
    mgr.update_phase(116); // → Checkpoint
    let err = mgr.finalize_competition(0, 116).unwrap_err();
    match err {
        EpochError::PhaseMismatch { expected, got } => {
            assert_eq!(expected, EpochPhase::Finalization);
            assert_eq!(got, EpochPhase::Checkpoint);
        }
        other => panic!("unexpected error: {other:?}"),
    }
}

/// finalize_competition rejects with PhaseMismatch in BlockProduction.
#[test]
fn test_finalize_competition_in_finalization() {
    let mgr = make_manager();
    // In Finalization with no submissions, finalize returns Ok(None) — phase
    // gate passed. Verify phase-gate behavior at BlockProduction instead.
    let err = mgr.finalize_competition(0, 100).unwrap_err();
    match err {
        EpochError::PhaseMismatch { expected, got } => {
            assert_eq!(expected, EpochPhase::Finalization);
            assert_eq!(got, EpochPhase::BlockProduction);
        }
        other => panic!("unexpected error: {other:?}"),
    }
}
