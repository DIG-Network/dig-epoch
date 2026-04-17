/// TYP-001 — EpochPhase and PhaseTransition
///
/// Normative: docs/requirements/domains/epoch_types/NORMATIVE.md §TYP-001
/// Spec ref:  docs/resources/SPEC.md §3.2-3.3
use dig_epoch::types::epoch_phase::{EpochPhase, PhaseTransition};
use std::collections::HashMap;

/// All four EpochPhase variants are constructable.
#[test]
fn test_epoch_phase_variants() {
    let _bp = EpochPhase::BlockProduction;
    let _ck = EpochPhase::Checkpoint;
    let _fi = EpochPhase::Finalization;
    let _co = EpochPhase::Complete;
}

/// EpochPhase can be cloned and copied; copied values equal originals.
#[test]
fn test_epoch_phase_clone_copy() {
    for phase in [
        EpochPhase::BlockProduction,
        EpochPhase::Checkpoint,
        EpochPhase::Finalization,
        EpochPhase::Complete,
    ] {
        let cloned = phase;
        let copied = phase;
        assert_eq!(phase, cloned);
        assert_eq!(phase, copied);
    }
}

/// Debug output matches the variant names.
#[test]
fn test_epoch_phase_debug() {
    assert_eq!(
        format!("{:?}", EpochPhase::BlockProduction),
        "BlockProduction"
    );
    assert_eq!(format!("{:?}", EpochPhase::Checkpoint), "Checkpoint");
    assert_eq!(format!("{:?}", EpochPhase::Finalization), "Finalization");
    assert_eq!(format!("{:?}", EpochPhase::Complete), "Complete");
}

/// PartialEq / Eq: same variants are equal, different are not.
#[test]
fn test_epoch_phase_eq() {
    assert_eq!(EpochPhase::BlockProduction, EpochPhase::BlockProduction);
    assert_ne!(EpochPhase::BlockProduction, EpochPhase::Checkpoint);
    assert_ne!(EpochPhase::Checkpoint, EpochPhase::Finalization);
    assert_ne!(EpochPhase::Finalization, EpochPhase::Complete);
}

/// All four variants can be used as distinct HashMap keys.
#[test]
fn test_epoch_phase_hash() {
    let mut map = HashMap::new();
    map.insert(EpochPhase::BlockProduction, 0u8);
    map.insert(EpochPhase::Checkpoint, 1u8);
    map.insert(EpochPhase::Finalization, 2u8);
    map.insert(EpochPhase::Complete, 3u8);
    assert_eq!(map.len(), 4);
    assert_eq!(map[&EpochPhase::BlockProduction], 0);
    assert_eq!(map[&EpochPhase::Complete], 3);
}

/// Serde round-trip: serialize to JSON and back preserves the variant.
#[test]
fn test_epoch_phase_serde_roundtrip() {
    for phase in [
        EpochPhase::BlockProduction,
        EpochPhase::Checkpoint,
        EpochPhase::Finalization,
        EpochPhase::Complete,
    ] {
        let json = serde_json::to_string(&phase).expect("serialize");
        let back: EpochPhase = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(phase, back);
    }
}

/// index() returns 0..3 for BlockProduction..Complete.
#[test]
fn test_epoch_phase_index() {
    assert_eq!(EpochPhase::BlockProduction.index(), 0);
    assert_eq!(EpochPhase::Checkpoint.index(), 1);
    assert_eq!(EpochPhase::Finalization.index(), 2);
    assert_eq!(EpochPhase::Complete.index(), 3);
}

/// next() returns the successor or None for Complete.
#[test]
fn test_epoch_phase_next() {
    assert_eq!(
        EpochPhase::BlockProduction.next(),
        Some(EpochPhase::Checkpoint)
    );
    assert_eq!(
        EpochPhase::Checkpoint.next(),
        Some(EpochPhase::Finalization)
    );
    assert_eq!(EpochPhase::Finalization.next(), Some(EpochPhase::Complete));
    assert_eq!(EpochPhase::Complete.next(), None);
}

/// previous() returns the predecessor or None for BlockProduction.
#[test]
fn test_epoch_phase_previous() {
    assert_eq!(
        EpochPhase::Complete.previous(),
        Some(EpochPhase::Finalization)
    );
    assert_eq!(
        EpochPhase::Finalization.previous(),
        Some(EpochPhase::Checkpoint)
    );
    assert_eq!(
        EpochPhase::Checkpoint.previous(),
        Some(EpochPhase::BlockProduction)
    );
    assert_eq!(EpochPhase::BlockProduction.previous(), None);
}

/// name() returns the spec-mandated string for each variant.
#[test]
fn test_epoch_phase_name() {
    assert_eq!(EpochPhase::BlockProduction.name(), "BlockProduction");
    assert_eq!(EpochPhase::Checkpoint.name(), "Checkpoint");
    assert_eq!(EpochPhase::Finalization.name(), "Finalization");
    assert_eq!(EpochPhase::Complete.name(), "Complete");
}

/// allows_block_production() is true only for BlockProduction.
#[test]
fn test_allows_block_production() {
    assert!(EpochPhase::BlockProduction.allows_block_production());
    assert!(!EpochPhase::Checkpoint.allows_block_production());
    assert!(!EpochPhase::Finalization.allows_block_production());
    assert!(!EpochPhase::Complete.allows_block_production());
}

/// allows_checkpoint_submission() is true only for Checkpoint.
#[test]
fn test_allows_checkpoint_submission() {
    assert!(!EpochPhase::BlockProduction.allows_checkpoint_submission());
    assert!(EpochPhase::Checkpoint.allows_checkpoint_submission());
    assert!(!EpochPhase::Finalization.allows_checkpoint_submission());
    assert!(!EpochPhase::Complete.allows_checkpoint_submission());
}

/// allows_finalization() is true only for Finalization.
#[test]
fn test_allows_finalization() {
    assert!(!EpochPhase::BlockProduction.allows_finalization());
    assert!(!EpochPhase::Checkpoint.allows_finalization());
    assert!(EpochPhase::Finalization.allows_finalization());
    assert!(!EpochPhase::Complete.allows_finalization());
}

/// PhaseTransition fields are accessible with correct values.
#[test]
fn test_phase_transition_fields() {
    let pt = PhaseTransition {
        epoch: 5,
        from: EpochPhase::BlockProduction,
        to: EpochPhase::Checkpoint,
        l1_height: 260,
    };
    assert_eq!(pt.epoch, 5);
    assert_eq!(pt.from, EpochPhase::BlockProduction);
    assert_eq!(pt.to, EpochPhase::Checkpoint);
    assert_eq!(pt.l1_height, 260);
}

/// PhaseTransition can be cloned; cloned fields match originals.
#[test]
fn test_phase_transition_clone() {
    let pt = PhaseTransition {
        epoch: 7,
        from: EpochPhase::Checkpoint,
        to: EpochPhase::Finalization,
        l1_height: 120,
    };
    let cloned = pt.clone();
    assert_eq!(cloned.epoch, pt.epoch);
    assert_eq!(cloned.from, pt.from);
    assert_eq!(cloned.to, pt.to);
    assert_eq!(cloned.l1_height, pt.l1_height);
}
