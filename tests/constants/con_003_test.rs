/// CON-003 — Reward Economics Constants
///
/// Normative: docs/requirements/domains/constants/NORMATIVE.md §CON-003
/// Spec ref:  docs/resources/SPEC.md §2.3
///
/// Verifies that all reward economics constants are declared with the correct
/// u64 types and spec-mandated values, and that the emission derivation chain
/// holds.
use dig_epoch::constants::{
    EPOCH_FIRST_BLOCK_BONUS, HALVINGS_BEFORE_TAIL, HALVING_INTERVAL_BLOCKS,
    HALVING_INTERVAL_EPOCHS, INITIAL_BLOCK_REWARD, INITIAL_EMISSION_PER_10_MIN,
    INITIAL_EPOCH_REWARD, L2_BLOCKS_PER_10_MIN, L2_BLOCK_TIME_MS, MINIMUM_EPOCH_REWARD,
    MOJOS_PER_L2, TAIL_BLOCK_REWARD, TAIL_EMISSION_PER_10_MIN,
};

/// MOJOS_PER_L2 is 1_000_000_000_000 (10^12).
#[test]
fn test_mojos_per_l2() {
    assert_eq!(MOJOS_PER_L2, 1_000_000_000_000u64);
}

/// L2_BLOCK_TIME_MS is 3_000 ms (3 seconds).
#[test]
fn test_l2_block_time_ms() {
    assert_eq!(L2_BLOCK_TIME_MS, 3_000u64);
}

/// L2_BLOCKS_PER_10_MIN is 200 (600_000 ms / 3_000 ms).
#[test]
fn test_l2_blocks_per_10_min() {
    assert_eq!(L2_BLOCKS_PER_10_MIN, 200u64);
}

/// INITIAL_EMISSION_PER_10_MIN == 64 * MOJOS_PER_L2.
#[test]
fn test_initial_emission_derivation() {
    assert_eq!(INITIAL_EMISSION_PER_10_MIN, 64 * MOJOS_PER_L2);
}

/// TAIL_EMISSION_PER_10_MIN == 4 * MOJOS_PER_L2.
#[test]
fn test_tail_emission_derivation() {
    assert_eq!(TAIL_EMISSION_PER_10_MIN, 4 * MOJOS_PER_L2);
}

/// INITIAL_BLOCK_REWARD == INITIAL_EMISSION_PER_10_MIN / L2_BLOCKS_PER_10_MIN.
#[test]
fn test_initial_block_reward_derivation() {
    assert_eq!(
        INITIAL_BLOCK_REWARD,
        INITIAL_EMISSION_PER_10_MIN / L2_BLOCKS_PER_10_MIN
    );
}

/// TAIL_BLOCK_REWARD == TAIL_EMISSION_PER_10_MIN / L2_BLOCKS_PER_10_MIN.
#[test]
fn test_tail_block_reward_derivation() {
    assert_eq!(
        TAIL_BLOCK_REWARD,
        TAIL_EMISSION_PER_10_MIN / L2_BLOCKS_PER_10_MIN
    );
}

/// HALVINGS_BEFORE_TAIL is 4.
#[test]
fn test_halvings_before_tail() {
    assert_eq!(HALVINGS_BEFORE_TAIL, 4u64);
}

/// INITIAL_EPOCH_REWARD is 32_000_000_000_000 (spec-declared value).
#[test]
fn test_initial_epoch_reward() {
    assert_eq!(INITIAL_EPOCH_REWARD, 32_000_000_000_000u64);
}

/// TAIL_BLOCK_REWARD < INITIAL_BLOCK_REWARD.
/// Enforced at compile time via `const _: () = assert!(...)`.
const _: () = assert!(TAIL_BLOCK_REWARD < INITIAL_BLOCK_REWARD);

#[test]
fn test_tail_less_than_initial() {
    // Materialize constants to avoid `clippy::assertions_on_constants`.
    let tail = TAIL_BLOCK_REWARD;
    let initial = INITIAL_BLOCK_REWARD;
    assert!(tail < initial);
}

/// HALVING_INTERVAL_BLOCKS is 94_608_000.
#[test]
fn test_halving_interval_blocks() {
    assert_eq!(HALVING_INTERVAL_BLOCKS, 94_608_000u64);
}

/// HALVING_INTERVAL_EPOCHS is 315_576.
#[test]
fn test_halving_interval_epochs() {
    assert_eq!(HALVING_INTERVAL_EPOCHS, 315_576u64);
}

/// MINIMUM_EPOCH_REWARD is 2_000_000_000_000.
#[test]
fn test_minimum_epoch_reward() {
    assert_eq!(MINIMUM_EPOCH_REWARD, 2_000_000_000_000u64);
}

/// EPOCH_FIRST_BLOCK_BONUS is 100_000_000_000.
#[test]
fn test_epoch_first_block_bonus() {
    assert_eq!(EPOCH_FIRST_BLOCK_BONUS, 100_000_000_000u64);
}
