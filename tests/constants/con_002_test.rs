/// CON-002 — Phase Boundary Constants
///
/// Normative: docs/requirements/domains/constants/NORMATIVE.md §CON-002
/// Spec ref:  docs/resources/SPEC.md §2.2
///
/// Verifies that `PHASE_BLOCK_PRODUCTION_END_PCT`, `PHASE_CHECKPOINT_END_PCT`,
/// and `PHASE_FINALIZATION_END_PCT` are declared with the correct types,
/// values, and ordering, and that they are accessible from the crate root.
use dig_epoch::constants::{
    PHASE_BLOCK_PRODUCTION_END_PCT, PHASE_CHECKPOINT_END_PCT, PHASE_FINALIZATION_END_PCT,
};

/// PHASE_BLOCK_PRODUCTION_END_PCT is 50.
#[test]
fn test_phase_block_production_end_pct_value() {
    assert_eq!(PHASE_BLOCK_PRODUCTION_END_PCT, 50u32);
}

/// PHASE_CHECKPOINT_END_PCT is 75.
#[test]
fn test_phase_checkpoint_end_pct_value() {
    assert_eq!(PHASE_CHECKPOINT_END_PCT, 75u32);
}

/// PHASE_FINALIZATION_END_PCT is 100.
#[test]
fn test_phase_finalization_end_pct_value() {
    assert_eq!(PHASE_FINALIZATION_END_PCT, 100u32);
}

/// All three constants are `u32` (participate in u32 arithmetic without casts).
#[test]
fn test_phase_boundary_types_are_u32() {
    let bp: u32 = PHASE_BLOCK_PRODUCTION_END_PCT;
    let ck: u32 = PHASE_CHECKPOINT_END_PCT;
    let fi: u32 = PHASE_FINALIZATION_END_PCT;
    // Values are just used to prevent dead-code optimisation; real assertion is
    // that the assignment above compiled without a cast.
    let _ = (bp, ck, fi);
}

/// Boundaries are strictly ascending: BlockProduction < Checkpoint < Finalization.
#[test]
fn test_phase_boundaries_ascending() {
    assert!(
        PHASE_BLOCK_PRODUCTION_END_PCT < PHASE_CHECKPOINT_END_PCT,
        "BlockProduction end ({}) must be < Checkpoint end ({})",
        PHASE_BLOCK_PRODUCTION_END_PCT,
        PHASE_CHECKPOINT_END_PCT,
    );
    assert!(
        PHASE_CHECKPOINT_END_PCT < PHASE_FINALIZATION_END_PCT,
        "Checkpoint end ({}) must be < Finalization end ({})",
        PHASE_CHECKPOINT_END_PCT,
        PHASE_FINALIZATION_END_PCT,
    );
}

/// Finalization boundary equals 100 — the phase window is closed at full completion.
#[test]
fn test_phase_finalization_end_pct_is_100_pct() {
    assert_eq!(
        PHASE_FINALIZATION_END_PCT, 100,
        "Finalization must end at exactly 100%% of the L1 window",
    );
}

/// Phase widths are asymmetric (SPEC §2.2): BlockProduction 50 pp,
/// Checkpoint 25 pp, Finalization 25 pp.
#[test]
fn test_phase_widths() {
    let bp_width = PHASE_BLOCK_PRODUCTION_END_PCT; // 0 → 50
    let ck_width = PHASE_CHECKPOINT_END_PCT - PHASE_BLOCK_PRODUCTION_END_PCT; // 50 → 75
    let fi_width = PHASE_FINALIZATION_END_PCT - PHASE_CHECKPOINT_END_PCT; // 75 → 100
    assert_eq!(
        bp_width, 50,
        "BlockProduction phase must span 50 percentage points"
    );
    assert_eq!(
        ck_width, 25,
        "Checkpoint phase must span 25 percentage points"
    );
    assert_eq!(
        fi_width, 25,
        "Finalization phase must span 25 percentage points"
    );
}
