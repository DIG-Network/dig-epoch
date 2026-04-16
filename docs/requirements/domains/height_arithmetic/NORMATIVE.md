# Height Arithmetic - Normative Requirements

> **Domain:** height_arithmetic
> **Prefix:** HEA
> **Spec reference:** [SPEC.md - Sections 5.1-5.4, 14.10](../../../resources/SPEC.md)

## Requirements

### HEA-001: epoch_for_block_height

`epoch_for_block_height(height: u64) -> u64` MUST return `(height - 1) / BLOCKS_PER_EPOCH` when `height >= GENESIS_HEIGHT` (height >= 1). The function MUST correctly map: height 1 to epoch 0, height 32 to epoch 0, height 33 to epoch 1, height 64 to epoch 1, height 65 to epoch 2. The function is a stateless pure function with no side effects.

**Spec reference:** SPEC Section 5.1

### HEA-002: first_height and checkpoint_height

`first_height_in_epoch(epoch: u64) -> u64` MUST return `epoch * BLOCKS_PER_EPOCH + 1`. This is the first L2 block height in the given epoch.

`epoch_checkpoint_height(epoch: u64) -> u64` MUST return `(epoch + 1) * BLOCKS_PER_EPOCH`. This is the last L2 block height in the given epoch (the checkpoint block).

These two functions define the inclusive range of block heights belonging to each epoch: `[first_height_in_epoch(e), epoch_checkpoint_height(e)]`.

**Spec reference:** SPEC Section 5.1

### HEA-003: Checkpoint Block Detection

The crate MUST expose the following three functions as separate public functions (not just as internal helpers):

`is_genesis_checkpoint_block(height: u64) -> bool` MUST return `height == GENESIS_HEIGHT` (true at height 1). MUST be a separate public function.

`is_epoch_checkpoint_block(height: u64) -> bool` MUST return `height % BLOCKS_PER_EPOCH == 0` (true at heights 32, 64, 96, ...). MUST be a separate public function.

`is_checkpoint_class_block(height: u64) -> bool` MUST return `is_genesis_checkpoint_block(height) || is_epoch_checkpoint_block(height)`. This combines genesis checkpoint detection and epoch checkpoint detection. MUST be a separate public function.

`ensure_checkpoint_block_empty(height: u64, spend_bundle_count: u32, total_cost: u64, total_fees: u64) -> Result<(), EpochError>` MUST return `Ok(())` for non-checkpoint heights regardless of parameter values. At checkpoint-class heights, it MUST return `Err(EpochError::CheckpointBlockNotEmpty)` if any of spend_bundle_count, total_cost, or total_fees is non-zero. This enforces the empty-checkpoint-block invariant.

**Spec reference:** SPEC Sections 5.2, 5.3

### HEA-004: L1 Range for Epoch

`l1_range_for_epoch(genesis_l1_height: u32, epoch: u64) -> (u32, u32)` MUST return `(genesis_l1_height + epoch * EPOCH_L1_BLOCKS, genesis_l1_height + (epoch + 1) * EPOCH_L1_BLOCKS - 1)`. The returned tuple represents the inclusive L1 height range for the given epoch's L1 window.

**Spec reference:** SPEC Section 5.4

### HEA-005: Round-Trip Identity

For all valid heights h >= 1: `first_height_in_epoch(epoch_for_block_height(h)) <= h <= epoch_checkpoint_height(epoch_for_block_height(h))`. This is a property test requirement that validates the internal consistency of the height-epoch conversion functions. Every valid L2 height must fall within the range defined by its epoch's first height and checkpoint height.

**Spec reference:** SPEC Section 14.10

### HEA-006: last_committed_height_in_epoch

`last_committed_height_in_epoch(epoch: u64, tip_height: u64) -> u64` MUST return `min(tip_height, epoch_checkpoint_height(epoch))`. This caps the last L2 height included in an epoch's checkpoint at the epoch checkpoint height, even if the chain tip is higher. This prevents namespace rollup from bleeding into the next epoch (DL-CKP-001).

**Spec reference:** SPEC Section 5.5

### HEA-007: is_first_block_after_epoch_checkpoint

`is_first_block_after_epoch_checkpoint(height: u64) -> bool` MUST return `height > 1 && (height - 1) % BLOCKS_PER_EPOCH == 0`. This returns true at heights 33, 65, 97, etc. -- the first block of each epoch after epoch 0. This is used to determine when the epoch-opening bonus applies.

**Spec reference:** SPEC Section 5.2
