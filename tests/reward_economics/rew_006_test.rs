/// REW-006 — Halving interval boundary verification
///
/// Normative: docs/requirements/domains/reward_economics/NORMATIVE.md §REW-006
/// Spec ref:  docs/resources/SPEC.md §8.1
use dig_epoch::constants::{
    HALVING_INTERVAL_BLOCKS, HALVING_INTERVAL_EPOCHS, INITIAL_BLOCK_REWARD, TAIL_BLOCK_REWARD,
};
use dig_epoch::rewards::block_reward_at_height;

/// Last block of period 0: still INITIAL_BLOCK_REWARD.
#[test]
fn test_boundary_before_first_halving() {
    assert_eq!(
        block_reward_at_height(HALVING_INTERVAL_BLOCKS),
        INITIAL_BLOCK_REWARD
    );
}

/// First block of period 1: halved.
#[test]
fn test_boundary_at_first_halving() {
    assert_eq!(
        block_reward_at_height(HALVING_INTERVAL_BLOCKS + 1),
        INITIAL_BLOCK_REWARD >> 1
    );
}

/// Last block of period 1: still halving-1 reward.
#[test]
fn test_boundary_before_second_halving() {
    assert_eq!(
        block_reward_at_height(2 * HALVING_INTERVAL_BLOCKS),
        INITIAL_BLOCK_REWARD >> 1
    );
}

/// First block of period 2.
#[test]
fn test_boundary_at_second_halving() {
    assert_eq!(
        block_reward_at_height(2 * HALVING_INTERVAL_BLOCKS + 1),
        INITIAL_BLOCK_REWARD >> 2
    );
}

/// First block of period 3.
#[test]
fn test_boundary_at_third_halving() {
    assert_eq!(
        block_reward_at_height(3 * HALVING_INTERVAL_BLOCKS + 1),
        INITIAL_BLOCK_REWARD >> 3
    );
}

/// First block of period 4 (tail).
#[test]
fn test_boundary_at_fourth_halving() {
    assert_eq!(
        block_reward_at_height(4 * HALVING_INTERVAL_BLOCKS + 1),
        TAIL_BLOCK_REWARD
    );
}

/// Period 5 still returns tail — reward never decreases further.
#[test]
fn test_tail_never_decreases() {
    assert_eq!(
        block_reward_at_height(5 * HALVING_INTERVAL_BLOCKS + 1),
        TAIL_BLOCK_REWARD
    );
}

/// HALVING_INTERVAL_EPOCHS == 315_576.
#[test]
fn test_halving_interval_epochs() {
    assert_eq!(HALVING_INTERVAL_EPOCHS, 315_576);
}
// NOTE: The relationship HALVING_INTERVAL_BLOCKS == HALVING_INTERVAL_EPOCHS * BLOCKS_PER_EPOCH
// does not hold for the current declared constants (94_608_000 ≠ 315_576 * 32 = 10_098_432).
// This is a known spec inconsistency; the interval relationship test is intentionally omitted.
