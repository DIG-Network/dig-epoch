/// REW-001 — block_reward_at_height with halving schedule
///
/// Normative: docs/requirements/domains/reward_economics/NORMATIVE.md §REW-001
/// Spec ref:  docs/resources/SPEC.md §8.1
use dig_epoch::constants::{
    HALVINGS_BEFORE_TAIL, HALVING_INTERVAL_BLOCKS, INITIAL_BLOCK_REWARD, MOJOS_PER_L2,
    TAIL_BLOCK_REWARD,
};
use dig_epoch::rewards::block_reward_at_height;

#[test]
fn test_reward_at_height_1() {
    assert_eq!(block_reward_at_height(1), INITIAL_BLOCK_REWARD);
}

/// Last block before first halving still returns INITIAL_BLOCK_REWARD.
#[test]
fn test_reward_before_first_halving() {
    assert_eq!(
        block_reward_at_height(HALVING_INTERVAL_BLOCKS),
        INITIAL_BLOCK_REWARD
    );
}

/// First block of halving 1 returns INITIAL_BLOCK_REWARD >> 1.
#[test]
fn test_reward_at_first_halving() {
    assert_eq!(
        block_reward_at_height(HALVING_INTERVAL_BLOCKS + 1),
        INITIAL_BLOCK_REWARD >> 1
    );
}

/// First block of halving 2.
#[test]
fn test_reward_at_second_halving() {
    assert_eq!(
        block_reward_at_height(2 * HALVING_INTERVAL_BLOCKS + 1),
        INITIAL_BLOCK_REWARD >> 2
    );
}

/// First block of halving 3.
#[test]
fn test_reward_at_third_halving() {
    assert_eq!(
        block_reward_at_height(3 * HALVING_INTERVAL_BLOCKS + 1),
        INITIAL_BLOCK_REWARD >> 3
    );
}

/// At and beyond 4th halving: tail emission.
#[test]
fn test_reward_at_tail() {
    assert_eq!(
        block_reward_at_height(HALVINGS_BEFORE_TAIL as u64 * HALVING_INTERVAL_BLOCKS + 1),
        TAIL_BLOCK_REWARD
    );
}

/// Far future: still tail.
#[test]
fn test_reward_far_future() {
    assert_eq!(block_reward_at_height(u64::MAX / 2), TAIL_BLOCK_REWARD);
}

/// INITIAL_BLOCK_REWARD == 0.32 L2 in mojos.
#[test]
fn test_values_in_mojos() {
    // 0.32 L2 = 32 * MOJOS_PER_L2 / 100
    assert_eq!(INITIAL_BLOCK_REWARD, 32 * MOJOS_PER_L2 / 100);
}
