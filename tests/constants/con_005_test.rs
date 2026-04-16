/// CON-005 — DFSP, Consensus, Slashing Constants
///
/// Normative: docs/requirements/domains/constants/NORMATIVE.md §CON-005
/// Spec ref:  docs/resources/SPEC.md §2.4-2.6
use dig_epoch::constants::{
    BLOCKS_PER_EPOCH, CHECKPOINT_THRESHOLD_PCT, CORRELATION_WINDOW_EPOCHS, DFSP_ACTIVATION_HEIGHT,
    DFSP_GENESIS_ISSUANCE_SUBSIDY_MOJOS_V1, DFSP_GRACE_PERIOD_NETWORK_EPOCHS,
    DFSP_SLASH_LOOKBACK_EPOCHS, DFSP_WALL_CLOCK_EPOCH_SECONDS, DIG_DFSP_ACTIVATION_HEIGHT_ENV,
    HARD_FINALITY_THRESHOLD_PCT, L2_BLOCK_TIME_MS, SLASH_LOOKBACK_EPOCHS,
    SOFT_FINALITY_THRESHOLD_PCT, WITHDRAWAL_DELAY_EPOCHS,
};

/// DFSP_WALL_CLOCK_EPOCH_SECONDS is 96.
#[test]
fn test_dfsp_wall_clock_epoch_seconds() {
    assert_eq!(DFSP_WALL_CLOCK_EPOCH_SECONDS, 96u64);
}

/// DFSP_GRACE_PERIOD_NETWORK_EPOCHS is 27_000.
#[test]
fn test_dfsp_grace_period() {
    assert_eq!(DFSP_GRACE_PERIOD_NETWORK_EPOCHS, 27_000u64);
}

/// DFSP_GENESIS_ISSUANCE_SUBSIDY_MOJOS_V1 is 0 and type is u128.
#[test]
fn test_dfsp_genesis_issuance_subsidy() {
    let v: u128 = DFSP_GENESIS_ISSUANCE_SUBSIDY_MOJOS_V1;
    assert_eq!(v, 0u128);
}

/// DFSP_ACTIVATION_HEIGHT defaults to u64::MAX (disabled).
#[test]
fn test_dfsp_activation_height_default() {
    assert_eq!(DFSP_ACTIVATION_HEIGHT, u64::MAX);
}

/// DIG_DFSP_ACTIVATION_HEIGHT_ENV is the expected env var name.
#[test]
fn test_dfsp_activation_height_env_name() {
    assert_eq!(DIG_DFSP_ACTIVATION_HEIGHT_ENV, "DIG_DFSP_ACTIVATION_HEIGHT");
}

/// All three consensus thresholds are 67.
#[test]
fn test_consensus_thresholds() {
    assert_eq!(SOFT_FINALITY_THRESHOLD_PCT, 67u64);
    assert_eq!(HARD_FINALITY_THRESHOLD_PCT, 67u64);
    assert_eq!(CHECKPOINT_THRESHOLD_PCT, 67u64);
}

/// CORRELATION_WINDOW_EPOCHS is u32 and equals 36.
#[test]
fn test_correlation_window_type() {
    let v: u32 = CORRELATION_WINDOW_EPOCHS;
    assert_eq!(v, 36u32);
}

/// SLASH_LOOKBACK_EPOCHS is 1_000.
#[test]
fn test_slash_lookback() {
    assert_eq!(SLASH_LOOKBACK_EPOCHS, 1_000u64);
}

/// DFSP_SLASH_LOOKBACK_EPOCHS equals SLASH_LOOKBACK_EPOCHS.
#[test]
fn test_dfsp_slash_lookback_equals_general() {
    assert_eq!(DFSP_SLASH_LOOKBACK_EPOCHS, SLASH_LOOKBACK_EPOCHS);
}

/// WITHDRAWAL_DELAY_EPOCHS is 50.
#[test]
fn test_withdrawal_delay() {
    assert_eq!(WITHDRAWAL_DELAY_EPOCHS, 50u64);
}

/// DFSP_WALL_CLOCK_EPOCH_SECONDS == (BLOCKS_PER_EPOCH * L2_BLOCK_TIME_MS) / 1000.
#[test]
fn test_dfsp_wall_clock_derivation() {
    assert_eq!(
        DFSP_WALL_CLOCK_EPOCH_SECONDS,
        (BLOCKS_PER_EPOCH * L2_BLOCK_TIME_MS) / 1_000,
    );
}
