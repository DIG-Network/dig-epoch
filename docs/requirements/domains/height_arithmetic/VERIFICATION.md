# Height Arithmetic - Verification Matrix

> **Domain:** height_arithmetic
> **Prefix:** HEA
> **Normative:** [NORMATIVE.md](./NORMATIVE.md)
> **Tracking:** [TRACKING.yaml](./TRACKING.yaml)

| ID | Status | Summary | Verification Approach |
|----|--------|---------|----------------------|
| HEA-001 | gap | epoch_for_block_height | Unit test: verify h=1 returns 0, h=32 returns 0, h=33 returns 1, h=64 returns 1, h=65 returns 2. Verify large heights (h=1,000,000) return correct epoch. Verify formula (h-1)/BLOCKS_PER_EPOCH is used. |
| HEA-002 | gap | first_height and checkpoint_height | Unit test: verify first_height_in_epoch(0)=1, first_height_in_epoch(1)=33, first_height_in_epoch(2)=65. Verify epoch_checkpoint_height(0)=32, epoch_checkpoint_height(1)=64, epoch_checkpoint_height(2)=96. Verify inverse: epoch_for_block_height(first_height_in_epoch(e))==e and epoch_for_block_height(epoch_checkpoint_height(e))==e for multiple epochs. |
| HEA-003 | gap | Checkpoint Block Detection | Unit test: verify is_checkpoint_class_block(1)=true (genesis), is_checkpoint_class_block(32)=true, is_checkpoint_class_block(64)=true, is_checkpoint_class_block(2)=false, is_checkpoint_class_block(33)=false. Verify ensure_checkpoint_block_empty passes at checkpoint with zeros, rejects with non-zero bundles/cost/fees, passes at non-checkpoint regardless. |
| HEA-004 | gap | L1 Range for Epoch | Unit test: verify genesis=100, e=0 returns (100, 131), e=1 returns (132, 163), e=2 returns (164, 195). Verify returned range width equals EPOCH_L1_BLOCKS. |
| HEA-005 | gap | Round-Trip Identity | Property test: for random h in 1..1,000,000, verify first_height_in_epoch(epoch_for_block_height(h)) <= h <= epoch_checkpoint_height(epoch_for_block_height(h)). Verify boundary values: first and last heights of multiple epochs. |
| HEA-006 | gap | last_committed_height_in_epoch | Unit test: verify returns min(tip_height, epoch_checkpoint_height(epoch)). Test tip below checkpoint height, at checkpoint height, and above checkpoint height. Verify prevents bleed into next epoch. |
| HEA-007 | gap | is_first_block_after_epoch_checkpoint | Unit test: verify returns true at h=33, h=65, h=97. Verify returns false at h=1, h=2, h=32, h=34. Verify formula h > 1 && (h - 1) % BLOCKS_PER_EPOCH == 0. |
