/// REW-005 — Tail emission floor (epoch_reward_with_floor)
///
/// Normative: docs/requirements/domains/reward_economics/NORMATIVE.md §REW-005
/// Spec ref:  docs/resources/SPEC.md §8.1
use dig_epoch::constants::{
    BLOCKS_PER_EPOCH, MINIMUM_EPOCH_REWARD, MOJOS_PER_L2, TAIL_BLOCK_REWARD,
};
use dig_epoch::rewards::epoch_reward_with_floor;

/// At tail emission, raw=32*0.02L2=0.64L2 → raised to MINIMUM_EPOCH_REWARD.
#[test]
fn test_floor_at_tail() {
    let raw_tail_epoch = BLOCKS_PER_EPOCH * TAIL_BLOCK_REWARD;
    assert!(raw_tail_epoch < MINIMUM_EPOCH_REWARD);
    assert_eq!(
        epoch_reward_with_floor(raw_tail_epoch),
        MINIMUM_EPOCH_REWARD
    );
}

/// Epoch reward above floor: returns as-is.
#[test]
fn test_floor_not_applied_above() {
    let above = 10 * MOJOS_PER_L2;
    assert_eq!(epoch_reward_with_floor(above), above);
}

/// Exactly at floor: no change.
#[test]
fn test_floor_at_boundary() {
    assert_eq!(
        epoch_reward_with_floor(MINIMUM_EPOCH_REWARD),
        MINIMUM_EPOCH_REWARD
    );
}

/// Below floor: raised to floor.
#[test]
fn test_floor_below() {
    let below = MOJOS_PER_L2; // 1 L2 < 2 L2
    assert_eq!(epoch_reward_with_floor(below), MINIMUM_EPOCH_REWARD);
}

/// Zero: raised to floor.
#[test]
fn test_floor_zero() {
    assert_eq!(epoch_reward_with_floor(0), MINIMUM_EPOCH_REWARD);
}

/// MINIMUM_EPOCH_REWARD == 2 L2 in mojos.
#[test]
fn test_minimum_epoch_reward_value() {
    assert_eq!(MINIMUM_EPOCH_REWARD, 2 * MOJOS_PER_L2);
}

/// 32 * TAIL_BLOCK_REWARD equals 0.64 L2 in mojos.
#[test]
fn test_tail_epoch_computation() {
    let tail_per_epoch = BLOCKS_PER_EPOCH * TAIL_BLOCK_REWARD;
    // 0.64 L2 = 64 * MOJOS_PER_L2 / 100
    assert_eq!(tail_per_epoch, 64 * MOJOS_PER_L2 / 100);
}
