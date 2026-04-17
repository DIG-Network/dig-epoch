/// PHS-003 — Phase transition events and should_advance()
///
/// Normative: docs/requirements/domains/phase_machine/NORMATIVE.md §PHS-003
/// Spec ref:  docs/resources/SPEC.md §4.3
use chia_protocol::Bytes32;
use dig_epoch::manager::EpochManager;
use dig_epoch::types::epoch_phase::EpochPhase;
use dig_epoch::types::events::EpochEvent;

fn zero_root() -> Bytes32 {
    Bytes32::new([0u8; 32])
}

fn make_manager() -> EpochManager {
    EpochManager::new(zero_root(), 100, zero_root())
}

/// should_advance() returns false during BlockProduction.
#[test]
fn test_should_advance_block_production() {
    let mgr = make_manager();
    assert!(!mgr.should_advance(100));
}

/// should_advance() returns false during Checkpoint.
#[test]
fn test_should_advance_checkpoint() {
    let mgr = make_manager();
    mgr.update_phase(116);
    assert!(!mgr.should_advance(116));
}

/// should_advance() returns false during Finalization.
#[test]
fn test_should_advance_finalization() {
    let mgr = make_manager();
    mgr.update_phase(124);
    assert!(!mgr.should_advance(124));
}

/// should_advance() returns true when Complete.
#[test]
fn test_should_advance_complete() {
    let mgr = make_manager();
    mgr.update_phase(132);
    assert!(mgr.should_advance(132));
}

/// update_phase() returns PhaseTransition with correct fields for PhaseChanged event.
#[test]
fn test_phase_transition_fields_for_event() {
    let mgr = make_manager();
    let t = mgr.update_phase(116).unwrap();
    assert_eq!(t.epoch, 0);
    assert_eq!(t.from, EpochPhase::BlockProduction);
    assert_eq!(t.to, EpochPhase::Checkpoint);
    assert_eq!(t.l1_height, 116);
    // Verify it can be used to construct EpochEvent::PhaseChanged
    let _event = EpochEvent::PhaseChanged {
        epoch: t.epoch,
        from: t.from,
        to: t.to,
        l1_height: t.l1_height,
    };
}

/// No transition when l1 stays in same phase window.
#[test]
fn test_no_event_when_phase_unchanged() {
    let mgr = make_manager();
    assert!(mgr.update_phase(105).is_none());
    assert!(mgr.update_phase(110).is_none());
}

/// Transition from BlockProduction all the way to Complete emits 3 transitions.
#[test]
fn test_transitions_through_all_phases() {
    let mgr = make_manager();
    let t1 = mgr.update_phase(116).unwrap();
    let t2 = mgr.update_phase(124).unwrap();
    let t3 = mgr.update_phase(132).unwrap();
    assert_eq!(t1.to, EpochPhase::Checkpoint);
    assert_eq!(t2.to, EpochPhase::Finalization);
    assert_eq!(t3.to, EpochPhase::Complete);
}
