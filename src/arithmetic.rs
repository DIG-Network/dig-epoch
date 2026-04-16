//! # `arithmetic` — pure height-to-epoch mapping functions
//!
//! **Introduced by:** `STR-002` — Module hierarchy (SPEC §13).
//!
//! **Owners:** HEA-001 through HEA-004 (Phase 4).
//!
//! **Spec reference:** [`SPEC.md` §5](../../docs/resources/SPEC.md)

/// Sentinel marker proving the module exists and is reachable at
/// `dig_epoch::arithmetic::STR_002_MODULE_PRESENT`.
#[doc(hidden)]
pub const STR_002_MODULE_PRESENT: () = ();

use crate::constants::{BLOCKS_PER_EPOCH, EPOCH_L1_BLOCKS, GENESIS_HEIGHT};
use crate::error::EpochError;

// -----------------------------------------------------------------------------
// HEA-001 — epoch_for_block_height
// -----------------------------------------------------------------------------

/// Maps an L2 block height to its epoch number.
///
/// Formula: `(height - 1) / BLOCKS_PER_EPOCH`
///
/// Height 1 is genesis (epoch 0). Requires `height >= 1`.
pub fn epoch_for_block_height(height: u64) -> u64 {
    (height - 1) / BLOCKS_PER_EPOCH
}

// -----------------------------------------------------------------------------
// HEA-002 — first_height_in_epoch / epoch_checkpoint_height
// -----------------------------------------------------------------------------

/// Returns the first L2 block height in the given epoch.
///
/// Formula: `epoch * BLOCKS_PER_EPOCH + 1`
pub fn first_height_in_epoch(epoch: u64) -> u64 {
    epoch * BLOCKS_PER_EPOCH + 1
}

/// Returns the checkpoint (last) L2 block height in the given epoch.
///
/// Formula: `(epoch + 1) * BLOCKS_PER_EPOCH`
pub fn epoch_checkpoint_height(epoch: u64) -> u64 {
    (epoch + 1) * BLOCKS_PER_EPOCH
}

// -----------------------------------------------------------------------------
// HEA-003 — Checkpoint block detection
// -----------------------------------------------------------------------------

/// Returns true if `height` is the genesis checkpoint block (height == GENESIS_HEIGHT).
pub fn is_genesis_checkpoint_block(height: u64) -> bool {
    height == GENESIS_HEIGHT
}

/// Returns true if `height` is an epoch checkpoint block (divisible by BLOCKS_PER_EPOCH).
pub fn is_epoch_checkpoint_block(height: u64) -> bool {
    height % BLOCKS_PER_EPOCH == 0
}

/// Returns true if `height` is genesis or an epoch checkpoint block.
pub fn is_checkpoint_class_block(height: u64) -> bool {
    is_genesis_checkpoint_block(height) || is_epoch_checkpoint_block(height)
}

/// Enforces the empty-checkpoint-block invariant.
///
/// Returns `Ok(())` for non-checkpoint heights regardless of parameters.
/// Returns `Err(EpochError::CheckpointBlockNotEmpty)` if any count is non-zero
/// at a checkpoint-class height.
pub fn ensure_checkpoint_block_empty(
    height: u64,
    spend_bundle_count: u32,
    total_cost: u64,
    total_fees: u64,
) -> Result<(), EpochError> {
    if is_checkpoint_class_block(height)
        && (spend_bundle_count != 0 || total_cost != 0 || total_fees != 0)
    {
        return Err(EpochError::CheckpointBlockNotEmpty(
            height,
            spend_bundle_count,
            total_cost,
            total_fees,
        ));
    }
    Ok(())
}

// -----------------------------------------------------------------------------
// HEA-004 — l1_range_for_epoch
// -----------------------------------------------------------------------------

/// Returns the `(start_l1_height, end_l1_height)` for a given epoch.
///
/// The range is inclusive. Each epoch's L1 window is `EPOCH_L1_BLOCKS` wide.
pub fn l1_range_for_epoch(genesis_l1_height: u32, epoch: u64) -> (u32, u32) {
    let start = genesis_l1_height + (epoch as u32 * EPOCH_L1_BLOCKS);
    let end = start + EPOCH_L1_BLOCKS - 1;
    (start, end)
}

// -----------------------------------------------------------------------------
// HEA-006 — last_committed_height_in_epoch
// -----------------------------------------------------------------------------

/// Returns the last L2 height included in this epoch's checkpoint.
///
/// Caps at `epoch_checkpoint_height(epoch)` even if `tip_height` is higher.
pub fn last_committed_height_in_epoch(epoch: u64, tip_height: u64) -> u64 {
    std::cmp::min(tip_height, epoch_checkpoint_height(epoch))
}

// -----------------------------------------------------------------------------
// HEA-007 — is_first_block_after_epoch_checkpoint
// -----------------------------------------------------------------------------

/// Returns true if `height` is the first block after an epoch checkpoint.
///
/// True at h=33, 65, 97, … — each epoch's opening block after epoch 0.
pub fn is_first_block_after_epoch_checkpoint(height: u64) -> bool {
    height > 1 && (height - 1) % BLOCKS_PER_EPOCH == 0
}
