/// PHS-001 — l1_progress_phase_for_network_epoch
///
/// Normative: docs/requirements/domains/phase_machine/NORMATIVE.md §PHS-001
/// Spec ref:  docs/resources/SPEC.md §4.2
use dig_epoch::phase::l1_progress_phase_for_network_epoch;
use dig_epoch::types::epoch_phase::EpochPhase;

// genesis_l1_height=100, epoch=0 → start=100, end=131 (EPOCH_L1_BLOCKS=32)
// progress = (l1 - 100) * 100 / 32

/// Progress 0% (l1 == epoch start) → BlockProduction.
#[test]
fn test_progress_0_pct() {
    assert_eq!(
        l1_progress_phase_for_network_epoch(100, 0, 100),
        EpochPhase::BlockProduction
    );
}

/// Progress 25% (l1=108) → BlockProduction.
#[test]
fn test_progress_25_pct() {
    // (108 - 100) * 100 / 32 = 25
    assert_eq!(
        l1_progress_phase_for_network_epoch(100, 0, 108),
        EpochPhase::BlockProduction
    );
}

/// Progress 49% just below boundary (l1=115) → BlockProduction.
#[test]
fn test_progress_49_pct() {
    // (115 - 100) * 100 / 32 = 46
    assert_eq!(
        l1_progress_phase_for_network_epoch(100, 0, 115),
        EpochPhase::BlockProduction
    );
}

/// Progress exactly 50% (l1=116) → Checkpoint.
#[test]
fn test_progress_exactly_50_pct() {
    // (116 - 100) * 100 / 32 = 50
    assert_eq!(
        l1_progress_phase_for_network_epoch(100, 0, 116),
        EpochPhase::Checkpoint
    );
}

/// Progress 60% (l1=119) → Checkpoint.
#[test]
fn test_progress_60_pct() {
    // (119 - 100) * 100 / 32 = 59
    assert_eq!(
        l1_progress_phase_for_network_epoch(100, 0, 119),
        EpochPhase::Checkpoint
    );
}

/// Progress exactly 75% (l1=124) → Finalization.
#[test]
fn test_progress_exactly_75_pct() {
    // (124 - 100) * 100 / 32 = 75
    assert_eq!(
        l1_progress_phase_for_network_epoch(100, 0, 124),
        EpochPhase::Finalization
    );
}

/// Progress 90% (l1=128) → Finalization.
#[test]
fn test_progress_90_pct() {
    // (128 - 100) * 100 / 32 = 87
    assert_eq!(
        l1_progress_phase_for_network_epoch(100, 0, 128),
        EpochPhase::Finalization
    );
}

/// Progress exactly 100% (l1=132) → Complete.
#[test]
fn test_progress_exactly_100_pct() {
    // (132 - 100) * 100 / 32 = 100
    assert_eq!(
        l1_progress_phase_for_network_epoch(100, 0, 132),
        EpochPhase::Complete
    );
}

/// Progress beyond 100% (l1=150) → Complete.
#[test]
fn test_progress_beyond_100_pct() {
    assert_eq!(
        l1_progress_phase_for_network_epoch(100, 0, 150),
        EpochPhase::Complete
    );
}

/// Determinism: two identical calls return the same result.
#[test]
fn test_determinism() {
    let r1 = l1_progress_phase_for_network_epoch(100, 0, 116);
    let r2 = l1_progress_phase_for_network_epoch(100, 0, 116);
    assert_eq!(r1, r2);
}

/// Works correctly for epoch > 0 (epoch 1, genesis_l1=100 → start=132).
#[test]
fn test_epoch_1() {
    // epoch 1 start = 100 + 1*32 = 132; 50% boundary at 132 + 16 = 148
    assert_eq!(
        l1_progress_phase_for_network_epoch(100, 1, 131),
        EpochPhase::BlockProduction
    );
    assert_eq!(
        l1_progress_phase_for_network_epoch(100, 1, 148),
        EpochPhase::Checkpoint
    );
}
