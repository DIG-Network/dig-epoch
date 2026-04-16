# Reward Economics - Normative Requirements

> **Domain:** reward_economics
> **Prefix:** REW
> **Spec reference:** [SPEC.md - Sections 8.1, 8.2, 8.3, 8.4](../../../resources/SPEC.md)

## Requirements

### REW-001: block_reward_at_height with Halving

`block_reward_at_height(height: u64) -> u64` MUST:

- Compute `halvings = (height - 1) / HALVING_INTERVAL_BLOCKS`.
- If `halvings >= HALVINGS_BEFORE_TAIL`, return `TAIL_BLOCK_REWARD`.
- Else return `INITIAL_BLOCK_REWARD >> halvings`.
- All values MUST be in mojos.

**Spec reference:** SPEC Section 8.1

### REW-002: total_block_reward with Bonus

`total_block_reward(height: u64, is_first_of_epoch: bool) -> u64` MUST return:

- `block_reward_at_height(height) + (if is_first_of_epoch { EPOCH_FIRST_BLOCK_BONUS } else { 0 })`.
- The first block after a checkpoint MUST receive an additional `0.1 L2` bonus (`EPOCH_FIRST_BLOCK_BONUS`).

**Spec reference:** SPEC Section 8.2

### REW-003: Fee Distribution

`proposer_fee_share(total_fees: u64) -> u64` MUST return `total_fees * FEE_PROPOSER_SHARE_PCT / 100`.

`burned_fee_remainder(total_fees: u64) -> u64` MUST return `total_fees - proposer_fee_share(total_fees)`.

Fee distribution is a 50/50 split between proposer and burn.

**Spec reference:** SPEC Section 8.3

### REW-004: Epoch Reward Distribution (5-Role Split)

`compute_reward_distribution(epoch: u64, total_reward: u64, total_fees: u64) -> RewardDistribution` MUST split `total_reward` as follows:

- 10% proposer
- 80% attester
- 3% EF spawner
- 4% score submitter
- 3% finalizer

`RewardDistribution` MUST also contain the fee split: `proposer_fee_share` and `burned_fees`.

**Spec reference:** SPEC Section 8.4

### REW-005: Tail Emission Floor

Epoch reward MUST NOT fall below `MINIMUM_EPOCH_REWARD` (2 L2).

After all halvings, the per-block reward is `TAIL_BLOCK_REWARD` (0.02 L2), giving `32 * 0.02 = 0.64 L2` per epoch, but the floor ensures a minimum of 2 L2 per epoch.

**Spec reference:** SPEC Section 8.1

### REW-006: Halving Boundaries

Halving MUST occur at `HALVING_INTERVAL_BLOCKS` intervals (~94.6M blocks, ~3 years at 3s block time).

4 halvings total: `0.32 -> 0.16 -> 0.08 -> 0.04 -> 0.02` (tail emission).

`HALVING_INTERVAL_EPOCHS = 315_576`.

**Spec reference:** SPEC Section 8.1

### REW-007: RewardDistribution Struct

RewardDistribution MUST contain the following 10 fields:

- `epoch: u64` - the epoch number
- `total_reward: u64` - base epoch reward (from halving schedule)
- `proposer_reward: u64` - 10% of total_reward
- `attester_reward: u64` - 80% of total_reward
- `ef_spawner_reward: u64` - 3% of total_reward
- `score_submitter_reward: u64` - 4% of total_reward
- `finalizer_reward: u64` - 3% of total_reward
- `total_fees: u64` - fees collected in this epoch
- `proposer_fee_share: u64` - 50% of total_fees
- `burned_fees: u64` - 50% of total_fees

RewardDistribution MUST derive Debug, Clone, Serialize, Deserialize.

The five reward shares (proposer_reward + attester_reward + ef_spawner_reward + score_submitter_reward + finalizer_reward) MUST equal total_reward (accounting for integer division rounding). The two fee shares (proposer_fee_share + burned_fees) MUST equal total_fees.

**Spec reference:** SPEC Section 3.12
