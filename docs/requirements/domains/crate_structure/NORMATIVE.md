# Crate Structure - Normative Requirements

> **Domain:** crate_structure
> **Prefix:** STR
> **Spec reference:** [SPEC.md - Sections 1.2, 6.2, 12, 13, 14](../../../resources/SPEC.md)

## Requirements

### STR-001: Cargo.toml Dependencies

The crate's Cargo.toml MUST include the following dependencies with the specified versions:

- `dig-block` = "0.1"
- `dig-constants` = "0.1"
- `chia-protocol` = "0.26"
- `chia-bls` = "0.26"
- `chia-consensus` = "0.26"
- `chia-sdk-types` = "0.30"
- `chia-sdk-signer` = "0.30"
- `chia-sha2` = "0.26"
- `clvm-utils` = "0.26"
- `bincode`
- `serde` (with `derive` feature)
- `thiserror`
- `parking_lot`

Each dependency serves a specific, non-redundant purpose as defined in SPEC Section 1.2. No dependency MAY be omitted without losing required functionality.

**Spec reference:** SPEC Section 1.2

### STR-002: Module Hierarchy

The crate MUST organize source files into the following module hierarchy:

- `constants.rs` - All epoch constants (geometry, phases, rewards, DFSP, thresholds, slashing)
- `types/` module directory containing:
  - `epoch_phase.rs` - `EpochPhase` enum and `PhaseTransition` struct
  - `epoch_info.rs` - `EpochInfo` mutable epoch state
  - `epoch_summary.rs` - `EpochSummary` immutable archive
  - `dfsp.rs` - All DFSP epoch-boundary types (`DfspCloseSnapshot`, `DfspExecutionStageV1`, `DfspEpochBurnContextV1`, `DfspEpochBurnPolicyV1`, `DfspEpochBurnPolicyScheduleEntryV1`, `DfspEpochStorageProofEvaluationContextV1`, `DfspEpochIssuancePreviewV1`)
  - `events.rs` - `EpochEvent` enum and `EpochStats` struct
  - `checkpoint_competition.rs` - `CheckpointCompetition`, `CompetitionStatus`
  - `reward.rs` - `RewardDistribution` struct
  - `verification.rs` - `EpochCheckpointData`, `EpochBlockLink`
- `arithmetic.rs` - Height-epoch mapping functions (`epoch_for_block_height`, `first_height_in_epoch`, `epoch_checkpoint_height`, etc.)
- `phase.rs` - Phase calculation function (`l1_progress_phase_for_network_epoch`)
- `rewards.rs` - Reward computation functions (`block_reward_at_height`, `total_block_reward`, `proposer_fee_share`, `burned_fee_remainder`, `compute_reward_distribution`)
- `manager.rs` - `EpochManager` struct and all methods
- `verification.rs` - Verification functions (`compute_epoch_block_root`, `epoch_block_inclusion_proof`, `compute_epoch_withdrawals_root`, checkpoint signing material)
- `dfsp.rs` - DFSP epoch-boundary functions (burn policy, operations digest, commitment digest, namespace rollup, tail roots, activation control)
- `error.rs` - `EpochError` and `CheckpointCompetitionError`

**Spec reference:** SPEC Section 13

### STR-003: Public Re-exports

`lib.rs` MUST re-export all public API surface items:

- `EpochManager` from `manager.rs`
- All type structs and enums: `EpochPhase`, `PhaseTransition`, `EpochInfo`, `EpochSummary`, `DfspCloseSnapshot`, `EpochEvent`, `EpochStats`, `CheckpointCompetition`, `CompetitionStatus`, `RewardDistribution`, `EpochCheckpointData`, `EpochBlockLink`, and all DFSP types
- All constants from `constants.rs`
- All free functions from `arithmetic.rs`, `phase.rs`, `rewards.rs`, `verification.rs`, and `dfsp.rs`
- Error types: `EpochError`, `CheckpointCompetitionError`

Consumers MUST be able to access all public items via `use dig_epoch::*` without navigating internal module paths.

**Spec reference:** SPEC Section 12

### STR-004: EpochManager Constructor

`EpochManager::new(network_id: Bytes32, genesis_l1_height: u32, initial_state_root: Bytes32) -> Self` MUST:

- Initialize the current epoch to epoch 0 via `EpochInfo::new(0, genesis_l1_height, GENESIS_HEIGHT, initial_state_root)`
- Set the initial phase to `BlockProduction`
- Initialize empty history, competitions, and rewards maps
- Store `network_id` and `genesis_l1_height` as immutable fields

The constructor SHOULD NOT accept any additional parameters. All runtime behavior MUST be derived from these three inputs plus the compile-time constants.

**Spec reference:** SPEC Section 6.2

### STR-005: Test Infrastructure

The crate MUST include test helper utilities:

- A helper function to create a test `EpochManager` with deterministic parameters (fixed network_id, genesis_l1_height, initial_state_root)
- A helper function to advance an `EpochManager` through all four phases by simulating L1 height progression
- A helper function to create mock `CheckpointSubmission` values with configurable score parameters (stake percentage, block count)
- A helper function to build an N-block epoch chain by calling `record_block()` N times with specified fee and transaction count values

These helpers SHOULD be defined in a `test_helpers` module gated behind `#[cfg(test)]` or in integration test files.

**Spec reference:** SPEC Section 14
