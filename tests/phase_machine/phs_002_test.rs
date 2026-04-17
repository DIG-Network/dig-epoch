/// PHS-002 — EpochManager phase tracking (current_phase, update_phase)
///
/// Normative: docs/requirements/domains/phase_machine/NORMATIVE.md §PHS-002
/// Spec ref:  docs/resources/SPEC.md §6.3
use chia_protocol::Bytes32;
use dig_epoch::manager::EpochManager;
use dig_epoch::types::epoch_phase::EpochPhase;

fn zero_root() -> Bytes32 {
    Bytes32::new([0u8; 32])
}

fn make_manager() -> EpochManager {
    // network_id=zero, genesis_l1_height=100; always starts at epoch 0.
    // 50% boundary at l1=116, 75% at l1=124, 100% at l1=132.
    EpochManager::new(zero_root(), 100, zero_root())
}

/// Initial phase is BlockProduction.
#[test]
fn test_initial_phase() {
    let mgr = make_manager();
    assert_eq!(mgr.current_phase(), EpochPhase::BlockProduction);
}

/// update_phase() returns None when phase is unchanged.
#[test]
fn test_update_phase_no_change() {
    let mgr = make_manager();
    // Still in BlockProduction (l1=110, progress=31%)
    assert!(mgr.update_phase(110).is_none());
    assert_eq!(mgr.current_phase(), EpochPhase::BlockProduction);
}

/// update_phase() returns Some(PhaseTransition) on phase change.
#[test]
fn test_update_phase_triggers_transition() {
    let mgr = make_manager();
    // Cross from BlockProduction to Checkpoint at l1=116
    let transition = mgr.update_phase(116);
    assert!(transition.is_some());
    let t = transition.unwrap();
    assert_eq!(t.from, EpochPhase::BlockProduction);
    assert_eq!(t.to, EpochPhase::Checkpoint);
    assert_eq!(t.l1_height, 116);
    assert_eq!(t.epoch, 0);
}

/// Phase persists after update_phase().
#[test]
fn test_phase_persists_after_update() {
    let mgr = make_manager();
    mgr.update_phase(116); // → Checkpoint
    assert_eq!(mgr.current_phase(), EpochPhase::Checkpoint);
}

/// Multiple transitions: BlockProduction → Checkpoint → Finalization → Complete.
#[test]
fn test_multiple_transitions() {
    let mgr = make_manager();
    let t1 = mgr.update_phase(116).unwrap(); // → Checkpoint
    assert_eq!(t1.to, EpochPhase::Checkpoint);
    let t2 = mgr.update_phase(124).unwrap(); // → Finalization
    assert_eq!(t2.to, EpochPhase::Finalization);
    let t3 = mgr.update_phase(132).unwrap(); // → Complete
    assert_eq!(t3.to, EpochPhase::Complete);
}

/// update_phase() returns None when called again with same phase range.
#[test]
fn test_no_double_transition() {
    let mgr = make_manager();
    mgr.update_phase(116); // → Checkpoint
                           // Still in Checkpoint window
    assert!(mgr.update_phase(120).is_none());
}

/// Transition fields: epoch, from, to, l1_height are all correct.
#[test]
fn test_transition_fields_correctness() {
    let mgr = make_manager();
    let t = mgr.update_phase(124).unwrap(); // BlockProduction → Finalization (skips Checkpoint if big jump)
    assert_eq!(t.l1_height, 124);
    assert_eq!(t.epoch, 0);
    assert!(t.from != t.to);
}
