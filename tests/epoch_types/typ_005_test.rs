/// TYP-005 — EpochEvent Enum
///
/// Normative: docs/requirements/domains/epoch_types/NORMATIVE.md §TYP-005
/// Spec ref:  docs/resources/SPEC.md §3.7
use dig_block::Checkpoint;
use dig_epoch::types::epoch_phase::EpochPhase;
use dig_epoch::types::events::EpochEvent;

/// EpochStarted pattern match extracts epoch and l1_height.
#[test]
fn test_epoch_started() {
    let e = EpochEvent::EpochStarted {
        epoch: 0,
        l1_height: 100,
    };
    if let EpochEvent::EpochStarted { epoch, l1_height } = e {
        assert_eq!(epoch, 0);
        assert_eq!(l1_height, 100);
    } else {
        panic!("wrong variant");
    }
}

/// PhaseChanged pattern match extracts all four fields.
#[test]
fn test_phase_changed() {
    let e = EpochEvent::PhaseChanged {
        epoch: 0,
        from: EpochPhase::BlockProduction,
        to: EpochPhase::Checkpoint,
        l1_height: 116,
    };
    if let EpochEvent::PhaseChanged {
        epoch,
        from,
        to,
        l1_height,
    } = e
    {
        assert_eq!(epoch, 0);
        assert_eq!(from, EpochPhase::BlockProduction);
        assert_eq!(to, EpochPhase::Checkpoint);
        assert_eq!(l1_height, 116);
    } else {
        panic!("wrong variant");
    }
}

/// EpochFinalized pattern match extracts epoch and checkpoint.
#[test]
fn test_epoch_finalized() {
    let cp = Checkpoint::new();
    let e = EpochEvent::EpochFinalized {
        epoch: 0,
        checkpoint: cp.clone(),
    };
    if let EpochEvent::EpochFinalized { epoch, checkpoint } = e {
        assert_eq!(epoch, 0);
        assert_eq!(checkpoint, cp);
    } else {
        panic!("wrong variant");
    }
}

/// EpochFailed pattern match extracts epoch.
#[test]
fn test_epoch_failed() {
    let e = EpochEvent::EpochFailed { epoch: 3 };
    if let EpochEvent::EpochFailed { epoch } = e {
        assert_eq!(epoch, 3);
    } else {
        panic!("wrong variant");
    }
}

/// All four variants produce non-empty Debug output.
#[test]
fn test_epoch_event_debug() {
    let events = [
        EpochEvent::EpochStarted {
            epoch: 0,
            l1_height: 0,
        },
        EpochEvent::PhaseChanged {
            epoch: 0,
            from: EpochPhase::BlockProduction,
            to: EpochPhase::Checkpoint,
            l1_height: 0,
        },
        EpochEvent::EpochFinalized {
            epoch: 0,
            checkpoint: Checkpoint::new(),
        },
        EpochEvent::EpochFailed { epoch: 0 },
    ];
    for e in &events {
        assert!(!format!("{e:?}").is_empty());
    }
}

/// All four variants can be cloned.
#[test]
fn test_epoch_event_clone() {
    let events = vec![
        EpochEvent::EpochStarted {
            epoch: 1,
            l1_height: 10,
        },
        EpochEvent::PhaseChanged {
            epoch: 1,
            from: EpochPhase::Checkpoint,
            to: EpochPhase::Finalization,
            l1_height: 24,
        },
        EpochEvent::EpochFinalized {
            epoch: 1,
            checkpoint: Checkpoint::new(),
        },
        EpochEvent::EpochFailed { epoch: 1 },
    ];
    for e in &events {
        let cloned = e.clone();
        assert_eq!(format!("{e:?}"), format!("{cloned:?}"));
    }
}
