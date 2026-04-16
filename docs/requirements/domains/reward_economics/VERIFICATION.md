# Reward Economics - Verification Matrix

> **Domain:** reward_economics
> **Prefix:** REW
> **Normative:** [NORMATIVE.md](./NORMATIVE.md)
> **Tracking:** [TRACKING.yaml](./TRACKING.yaml)

| ID | Status | Summary | Verification Approach |
|----|--------|---------|----------------------|
| REW-001 | gap | block_reward_at_height with Halving | Unit test: verify block_reward_at_height returns INITIAL_BLOCK_REWARD at height 1, correct halved values at each halving boundary, TAIL_BLOCK_REWARD after HALVINGS_BEFORE_TAIL halvings. Verify computation halvings = (height - 1) / HALVING_INTERVAL_BLOCKS. Verify all values in mojos. |
| REW-002 | gap | total_block_reward with Bonus | Unit test: verify total_block_reward returns block_reward_at_height(height) for non-first blocks. Verify EPOCH_FIRST_BLOCK_BONUS (0.1 L2) added when is_first_of_epoch is true. Test boundary at epoch transitions. |
| REW-003 | gap | Fee Distribution | Unit test: verify proposer_fee_share returns total_fees * FEE_PROPOSER_SHARE_PCT / 100. Verify burned_fee_remainder returns total_fees - proposer_fee_share. Verify 50/50 split. Test with zero fees, odd fees (rounding), and large fee values. |
| REW-004 | gap | Epoch Reward Distribution (5-Role Split) | Unit test: verify compute_reward_distribution splits total_reward into 10% proposer, 80% attester, 3% EF spawner, 4% score submitter, 3% finalizer. Verify RewardDistribution contains fee split fields. Verify all shares sum to total_reward (accounting for rounding). |
| REW-005 | gap | Tail Emission Floor | Unit test: verify epoch reward never falls below MINIMUM_EPOCH_REWARD (2 L2). Verify at tail emission (0.02 L2/block * 32 blocks = 0.64 L2) the floor of 2 L2 applies. Test heights well beyond all halvings. |
| REW-006 | gap | Halving Boundaries | Unit test: verify halving occurs exactly at HALVING_INTERVAL_BLOCKS boundaries. Verify 4-step halving schedule: 0.32 -> 0.16 -> 0.08 -> 0.04 -> 0.02. Verify HALVING_INTERVAL_EPOCHS = 315_576. Test reward at each halving boundary and one block before/after. |
| REW-007 | gap | RewardDistribution Struct | Unit test: construct RewardDistribution with all 10 fields. Verify derives (Debug, Clone, Serialize, Deserialize). Verify five reward shares sum to total_reward. Verify two fee shares sum to total_fees. Verify compute_reward_distribution() produces correct RewardDistribution. |
