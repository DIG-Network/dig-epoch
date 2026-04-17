//! # `test_helpers` — reusable test fixtures (STR-005)
//!
//! Deterministic helpers for building [`EpochManager`] fixtures, driving
//! phase progression, and generating mock submissions. Exposed as a
//! public module so integration tests under `tests/` can use them.
//!
//! **Not for production use.** Signatures and public keys produced by
//! `mock_checkpoint_submission` are synthetic defaults and do not verify.
//!
//! **Spec reference:** [`SPEC.md` §14](../../docs/resources/SPEC.md) / STR-005.

use chia_protocol::Bytes32;
use dig_block::{Checkpoint, CheckpointSubmission, PublicKey, Signature, SignerBitmap};

use crate::constants::{BLOCKS_PER_EPOCH, EPOCH_L1_BLOCKS};
use crate::manager::EpochManager;
use crate::types::epoch_phase::PhaseTransition;

/// Deterministic test `network_id` — all bytes 0x01.
pub fn test_network_id() -> Bytes32 {
    Bytes32::new([0x01u8; 32])
}

/// Deterministic test initial state root — all bytes 0xAA.
pub fn test_initial_state_root() -> Bytes32 {
    Bytes32::new([0xAAu8; 32])
}

/// Default test genesis L1 height. Leaves room for pre-genesis L1 blocks.
pub const TEST_GENESIS_L1_HEIGHT: u32 = 100;

/// Creates an [`EpochManager`] with fixed, deterministic test parameters.
pub fn test_epoch_manager() -> EpochManager {
    EpochManager::new(
        test_network_id(),
        TEST_GENESIS_L1_HEIGHT,
        test_initial_state_root(),
    )
}

/// Advances `manager` through all four phases by calling
/// [`EpochManager::update_phase`] at the L1 heights that cross each
/// boundary. Returns the sequence of observed transitions (expected length 3).
pub fn advance_through_phases(manager: &EpochManager) -> Vec<PhaseTransition> {
    let start = manager.current_epoch_info().start_l1_height;
    let epoch_len = EPOCH_L1_BLOCKS;
    // 50%, 75%, 100% of the window.
    let checkpoint_start = start + epoch_len / 2;
    let finalization_start = start + (epoch_len * 3) / 4;
    let complete_start = start + epoch_len;
    let mut transitions = Vec::new();
    for h in [checkpoint_start, finalization_start, complete_start] {
        if let Some(t) = manager.update_phase(h) {
            transitions.push(t);
        }
    }
    transitions
}

/// Creates a mock [`CheckpointSubmission`] with the given epoch and scoring
/// parameters. Score = `stake_percentage * block_count` per CKP-004.
///
/// Uses default/synthetic signatures and public keys — not valid under BLS
/// verification.
pub fn mock_checkpoint_submission(
    epoch: u64,
    stake_percentage: u64,
    block_count: u32,
) -> CheckpointSubmission {
    let mut cp = Checkpoint::new();
    cp.epoch = epoch;
    cp.block_count = block_count;
    // Distinguish the checkpoint by score so hashes don't collide across
    // submissions within a test.
    cp.state_root = Bytes32::new([(stake_percentage & 0xFF) as u8; 32]);
    let score = stake_percentage * u64::from(block_count);
    CheckpointSubmission::new(
        cp,
        SignerBitmap::new(0),
        Signature::default(),
        PublicKey::default(),
        score,
        0,
    )
}

/// Records `n` blocks into the current epoch on `manager` via
/// [`EpochManager::record_block`]. Panics if `n > BLOCKS_PER_EPOCH`.
///
/// Returns `(total_fees, total_txns)` accumulated.
pub fn build_n_block_epoch(
    manager: &EpochManager,
    n: u32,
    fee_per_block: u64,
    tx_per_block: u64,
) -> (u64, u64) {
    assert!(
        u64::from(n) <= BLOCKS_PER_EPOCH,
        "build_n_block_epoch: n {n} exceeds BLOCKS_PER_EPOCH ({BLOCKS_PER_EPOCH})"
    );
    for _ in 0..n {
        manager
            .record_block(fee_per_block, tx_per_block)
            .expect("test manager should be in BlockProduction at start");
    }
    (u64::from(n) * fee_per_block, u64::from(n) * tx_per_block)
}
