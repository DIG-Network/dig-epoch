# Constants - Normative Requirements

> **Domain:** constants
> **Prefix:** CON
> **Spec reference:** [SPEC.md - Sections 2.1-2.6](../../../resources/SPEC.md)

## Requirements

### CON-001: Epoch Geometry

The crate MUST define the following epoch geometry constants:

- `BLOCKS_PER_EPOCH: u64 = 32` - L2 blocks per epoch. Each epoch spans exactly 32 committed heights. Epoch e contains heights `[e * BLOCKS_PER_EPOCH + 1, (e + 1) * BLOCKS_PER_EPOCH]`.
- `EPOCH_L1_BLOCKS: u32 = 32` - L1 blocks per epoch window (~10 minutes at 18-second Chia block time). Phase progression is computed as a percentage of this window.
- `GENESIS_HEIGHT: u64 = 1` - First L2 block height. The genesis block is at height 1, not 0.

These constants MUST be `pub const` and accessible from the crate root.

**Spec reference:** SPEC Section 2.1

### CON-002: Phase Boundaries

The crate MUST define the following phase boundary percentages:

- `PHASE_BLOCK_PRODUCTION_END_PCT: u32 = 50` - Block production phase ends at 50% of the L1 window.
- `PHASE_CHECKPOINT_END_PCT: u32 = 75` - Checkpoint submission phase ends at 75% of the L1 window.
- `PHASE_FINALIZATION_END_PCT: u32 = 100` - Finalization phase ends at 100% of the L1 window.

These percentages define the four-phase epoch lifecycle with asymmetric windows: BlockProduction (0-50%), Checkpoint (50-75%), Finalization (75-100%), Complete (>=100%).

**Spec reference:** SPEC Section 2.2

### CON-003: Reward Economics Constants

The crate MUST define the following reward economics constants:

- `MOJOS_PER_L2: u64 = 1_000_000_000_000` - 1 L2 token = 10^12 mojos.
- `L2_BLOCK_TIME_MS: u64 = 3_000` - L2 block time in milliseconds.
- `L2_BLOCKS_PER_10_MIN: u64 = 200` - L2 blocks per 10-minute window (600_000 / 3_000).
- `INITIAL_EMISSION_PER_10_MIN: u64 = 64 * MOJOS_PER_L2` - Initial emission rate: 64 L2 per 10 minutes.
- `TAIL_EMISSION_PER_10_MIN: u64 = 4 * MOJOS_PER_L2` - Tail emission rate: 4 L2 per 10 minutes.
- `INITIAL_BLOCK_REWARD: u64 = 320_000_000_000` - Per-block reward before any halving (0.32 L2). Derived: INITIAL_EMISSION_PER_10_MIN / L2_BLOCKS_PER_10_MIN.
- `TAIL_BLOCK_REWARD: u64 = 20_000_000_000` - Per-block reward at tail emission (0.02 L2). Derived: TAIL_EMISSION_PER_10_MIN / L2_BLOCKS_PER_10_MIN.
- `HALVING_INTERVAL_BLOCKS: u64 = 94_608_000` - Halving interval: ~3 years of blocks at 3-second block time.
- `HALVINGS_BEFORE_TAIL: u64 = 4` - Number of halvings before switching to tail emission.
- `INITIAL_EPOCH_REWARD: u64 = 32_000_000_000_000` - Initial epoch reward (sum of block rewards across one epoch, 32 * INITIAL_BLOCK_REWARD = 10.24 L2).
- `HALVING_INTERVAL_EPOCHS: u64 = 315_576` - Halving interval in epochs.
- `MINIMUM_EPOCH_REWARD: u64 = 2_000_000_000_000` - Minimum epoch reward (tail emission floor).
- `EPOCH_FIRST_BLOCK_BONUS: u64 = 100_000_000_000` - Bonus reward for the first block after an epoch checkpoint.

These constants are economic commitments that MUST be identical across all validators. They MUST NOT be runtime-configurable.

**Spec reference:** SPEC Section 2.3

### CON-004: Fee and Reward Distribution

The crate MUST define the following fee and reward distribution constants:

**Fee distribution:**
- `FEE_PROPOSER_SHARE_PCT: u64 = 50` - Proposer share of collected fees (percentage).
- `FEE_BURN_SHARE_PCT: u64 = 50` - Burn share of collected fees (percentage).

**Epoch reward distribution (shares MUST sum to 100):**
- `PROPOSER_REWARD_SHARE: u64 = 10` - Proposer share of epoch reward.
- `ATTESTER_REWARD_SHARE: u64 = 80` - Attester share of epoch reward.
- `EF_SPAWNER_REWARD_SHARE: u64 = 3` - EF spawner share of epoch reward.
- `SCORE_SUBMITTER_REWARD_SHARE: u64 = 4` - Score submitter share of epoch reward.
- `FINALIZER_REWARD_SHARE: u64 = 3` - Finalizer share of epoch reward.

The five epoch reward distribution shares (10 + 80 + 3 + 4 + 3) MUST sum to exactly 100.

**Spec reference:** SPEC Section 2.3

### CON-005: DFSP, Consensus, Slashing Constants

The crate MUST define the following DFSP, consensus threshold, and slashing/withdrawal constants:

**DFSP epoch parameters:**
- `DFSP_WALL_CLOCK_EPOCH_SECONDS: u64 = 96` - Wall-clock seconds per DFSP accounting epoch. Derived: (32 blocks * 3000ms) / 1000.
- `DFSP_GRACE_PERIOD_NETWORK_EPOCHS: u64 = 27_000` - Network epochs a CID may remain in Grace state before expiring. Derived from 30-day grace window: (30 * 24 * 3600) / 96.
- `DFSP_GENESIS_ISSUANCE_SUBSIDY_MOJOS_V1: u128 = 0` - Bootstrap genesis issuance subsidy per evaluated epoch (mojos).
- `DFSP_ACTIVATION_HEIGHT: u64 = u64::MAX` - DFSP activation height (default: disabled).
- `DIG_DFSP_ACTIVATION_HEIGHT_ENV: &str = "DIG_DFSP_ACTIVATION_HEIGHT"` - Environment variable name for DFSP activation height override.

**Consensus thresholds:**
- `SOFT_FINALITY_THRESHOLD_PCT: u64 = 67` - Stake percentage required for soft finality.
- `HARD_FINALITY_THRESHOLD_PCT: u64 = 67` - Stake percentage required for a checkpoint to win the competition.
- `CHECKPOINT_THRESHOLD_PCT: u64 = 67` - Stake percentage required for a valid checkpoint submission.

**Slashing and withdrawal:**
- `CORRELATION_WINDOW_EPOCHS: u32 = 36` - Epochs to track for correlation penalty calculation.
- `SLASH_LOOKBACK_EPOCHS: u64 = 1_000` - Maximum lookback for slashable offenses (in epochs).
- `DFSP_SLASH_LOOKBACK_EPOCHS: u64 = SLASH_LOOKBACK_EPOCHS` - DFSP slashing evidence lookback (same as general slashing).
- `WITHDRAWAL_DELAY_EPOCHS: u64 = 50` - Epochs before a withdrawal completes.

**Spec reference:** SPEC Sections 2.4, 2.5, 2.6

### CON-006: Sentinel Constants

The crate MUST define the following sentinel constants:

- `EMPTY_ROOT: Bytes32` - SHA-256 hash of the empty string (`e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`). Used as the default root for empty Merkle trees and as the initial value for DFSP roots in `EpochInfo::new()`.

This constant MUST be `pub const` and accessible from the crate root.

**Spec reference:** SPEC Sections 7.1, 7.3, 14.1
