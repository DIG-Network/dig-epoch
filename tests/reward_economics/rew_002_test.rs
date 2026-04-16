/// REW-002 — total_block_reward with epoch-first-block bonus
///
/// Normative: docs/requirements/domains/reward_economics/NORMATIVE.md §REW-002
/// Spec ref:  docs/resources/SPEC.md §8.2
use dig_epoch::constants::{EPOCH_FIRST_BLOCK_BONUS, INITIAL_BLOCK_REWARD, TAIL_BLOCK_REWARD};
use dig_epoch::rewards::{block_reward_at_height, total_block_reward};

#[test]
fn test_reward_no_bonus() {
    let h = 1;
    assert_eq!(total_block_reward(h, false), block_reward_at_height(h));
}

#[test]
fn test_reward_with_bonus() {
    let h = 1;
    assert_eq!(
        total_block_reward(h, true),
        block_reward_at_height(h) + EPOCH_FIRST_BLOCK_BONUS
    );
}

/// Bonus during first halving period.
#[test]
fn test_bonus_at_halving_0() {
    assert_eq!(
        total_block_reward(1, true),
        INITIAL_BLOCK_REWARD + EPOCH_FIRST_BLOCK_BONUS
    );
}

/// Bonus during tail emission.
#[test]
fn test_bonus_at_tail() {
    assert_eq!(
        total_block_reward(u64::MAX / 2, true),
        TAIL_BLOCK_REWARD + EPOCH_FIRST_BLOCK_BONUS
    );
}

/// EPOCH_FIRST_BLOCK_BONUS == 0.1 L2 in mojos (100_000_000_000).
#[test]
fn test_bonus_value() {
    assert_eq!(EPOCH_FIRST_BLOCK_BONUS, 100_000_000_000);
}

/// No overflow when adding bonus to maximum base reward.
#[test]
fn test_no_overflow() {
    let _result = total_block_reward(1, true);
}
