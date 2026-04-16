//! # `rewards` — block reward and distribution computation
//!
//! **Introduced by:** `STR-002` — Module hierarchy (SPEC §13).
//!
//! **Owners:** REW-001–006 (Phase 6).
//!
//! **Spec reference:** [`SPEC.md` §8](../../docs/resources/SPEC.md)

/// Sentinel marker proving the module exists and is reachable at
/// `dig_epoch::rewards::STR_002_MODULE_PRESENT`.
#[doc(hidden)]
pub const STR_002_MODULE_PRESENT: () = ();

use crate::constants::{
    EPOCH_FIRST_BLOCK_BONUS, FEE_PROPOSER_SHARE_PCT, HALVINGS_BEFORE_TAIL, HALVING_INTERVAL_BLOCKS,
    INITIAL_BLOCK_REWARD, MINIMUM_EPOCH_REWARD, TAIL_BLOCK_REWARD,
};
use crate::types::reward::RewardDistribution;

// -----------------------------------------------------------------------------
// REW-001 — block_reward_at_height
// -----------------------------------------------------------------------------

/// Returns the per-block reward for a given L2 block height.
///
/// Uses 4-halving schedule: INITIAL_BLOCK_REWARD halved every HALVING_INTERVAL_BLOCKS.
/// After HALVINGS_BEFORE_TAIL halvings, permanently returns TAIL_BLOCK_REWARD.
pub fn block_reward_at_height(height: u64) -> u64 {
    let halvings = (height - 1) / HALVING_INTERVAL_BLOCKS;
    if halvings >= HALVINGS_BEFORE_TAIL {
        TAIL_BLOCK_REWARD
    } else {
        INITIAL_BLOCK_REWARD >> halvings
    }
}

// -----------------------------------------------------------------------------
// REW-002 — total_block_reward
// -----------------------------------------------------------------------------

/// Returns the total reward for a block, including the optional epoch-first bonus.
pub fn total_block_reward(height: u64, is_first_of_epoch: bool) -> u64 {
    block_reward_at_height(height)
        + if is_first_of_epoch {
            EPOCH_FIRST_BLOCK_BONUS
        } else {
            0
        }
}

// -----------------------------------------------------------------------------
// REW-003 — fee distribution
// -----------------------------------------------------------------------------

/// Returns the proposer's share of transaction fees.
pub fn proposer_fee_share(total_fees: u64) -> u64 {
    total_fees * FEE_PROPOSER_SHARE_PCT / 100
}

/// Returns the burned portion of transaction fees (total − proposer share).
pub fn burned_fee_remainder(total_fees: u64) -> u64 {
    total_fees - proposer_fee_share(total_fees)
}

// -----------------------------------------------------------------------------
// REW-004 — compute_reward_distribution
// -----------------------------------------------------------------------------

/// Splits the epoch reward across 5 roles and combines with fee distribution.
///
/// The attester share absorbs any integer rounding to guarantee the 5 shares
/// always sum exactly to `total_reward`.
pub fn compute_reward_distribution(
    epoch: u64,
    total_reward: u64,
    total_fees: u64,
) -> RewardDistribution {
    let proposer_reward = total_reward * 10 / 100;
    let ef_spawner_reward = total_reward * 3 / 100;
    let score_submitter_reward = total_reward * 4 / 100;
    let finalizer_reward = total_reward * 3 / 100;
    let attester_reward = total_reward
        - proposer_reward
        - ef_spawner_reward
        - score_submitter_reward
        - finalizer_reward;
    RewardDistribution {
        epoch,
        proposer_reward,
        attester_reward,
        ef_spawner_reward,
        score_submitter_reward,
        finalizer_reward,
        proposer_fee_share: proposer_fee_share(total_fees),
        burned_fees: burned_fee_remainder(total_fees),
    }
}

// -----------------------------------------------------------------------------
// REW-005 — epoch_reward_with_floor
// -----------------------------------------------------------------------------

/// Applies the tail emission floor: epoch reward never falls below MINIMUM_EPOCH_REWARD.
pub fn epoch_reward_with_floor(computed_epoch_reward: u64) -> u64 {
    computed_epoch_reward.max(MINIMUM_EPOCH_REWARD)
}
