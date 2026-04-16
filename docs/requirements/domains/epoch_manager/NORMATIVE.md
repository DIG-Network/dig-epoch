# Epoch Manager - Normative Requirements

| Field       | Value          |
|-------------|----------------|
| Domain      | epoch_manager  |
| Prefix      | MGR            |
| Total Items | 8              |
| Status      | Draft          |
| Spec        | [SPEC.md](../../../resources/SPEC.md) |

## Requirements

### MGR-001: EpochManager Struct

`EpochManager` MUST use `parking_lot::RwLock<EpochManagerInner>` for interior mutability. The inner state MUST contain:

- `network_id`: network identifier
- `genesis_l1_height`: the L1 block height at genesis
- `current_epoch`: `EpochInfo` — the current epoch's state
- `competition`: `CheckpointCompetition` — the current epoch's checkpoint competition
- `summaries`: `Vec<EpochSummary>` — history of completed epochs

Concurrent reads MUST be allowed with exclusive writes.

**Spec Reference:** Section 6.1

### MGR-002: record_block

`record_block(&self, fees: u64, tx_count: u64) -> Result<()>` MUST:

- Increment `blocks_produced` in the current `EpochInfo`.
- Add `fees` to `total_fees` in the current `EpochInfo`.
- Add `tx_count` to `total_transactions` in the current `EpochInfo`.
- Require the current phase to be `BlockProduction`. If not, MUST return `EpochError::PhaseMismatch`.

**Spec Reference:** Section 6.4

### MGR-003: set_current_epoch_chain_totals

`set_current_epoch_chain_totals(&self, blocks: u32, fees: u64, txns: u64)` MUST overwrite the following fields in the current `EpochInfo`:

- `blocks_produced` = `blocks`
- `total_fees` = `fees`
- `total_transactions` = `txns`

This is used for resync/correction without incrementing. It sets absolute values rather than adding to existing totals.

**Spec Reference:** Section 6.4

### MGR-004: advance_epoch

`advance_epoch(&self, l1_height: u32, state_root: Bytes32) -> Result<u64, EpochError>` MUST:

1. Verify the current phase is `Complete`. If not, MUST return `EpochError::EpochNotComplete`.
2. Verify a checkpoint is set (finalized). If not, MUST return `EpochError::NoFinalizedCheckpoint`.
3. Archive the current `EpochInfo` as an `EpochSummary`.
4. Create a new `EpochInfo` for `epoch + 1` with the provided `state_root`.
5. Reset the `CheckpointCompetition`.
6. Return the new epoch number (`u64`).

**Spec Reference:** Section 6.7

### MGR-005: Query Methods

`EpochManager` MUST provide the following query methods:

- `get_epoch_info() -> EpochInfo` — Returns a clone of the current epoch info.
- `get_epoch_summary(epoch: u64) -> Option<EpochSummary>` — Returns the summary for a specific completed epoch from history.
- `recent_summaries(n: usize) -> Vec<EpochSummary>` — Returns the last `n` summaries from the tail of the history.
- `total_stats() -> EpochStats` — Returns aggregate statistics across all epochs.
- `get_rewards(epoch: u64) -> Option<RewardDistribution>` — Returns the reward distribution for a specific epoch.

**Spec Reference:** Section 6.9

### MGR-006: DFSP Close Snapshot

`set_current_epoch_dfsp_close_snapshot(&self, snapshot: DfspCloseSnapshot) -> Result<()>` MUST set the DFSP close data on the current `EpochInfo` before `advance_epoch()` is called. MUST require the current phase to be `Finalization`. If not, MUST return an appropriate error.

**Spec Reference:** Section 6.6

### MGR-007: Epoch History Management

Completed epochs MUST be archived as `EpochSummary` in the `summaries` vec. Summaries MUST be ordered by epoch number. `recent_summaries` returns from the tail of the vec. History is append-only within a session (summaries are never removed or reordered).

**Spec Reference:** Section 6.7

### MGR-008: Core Instance Methods and Accessors

`EpochManager` MUST provide the following core instance methods:

- `current_epoch(&self) -> u64` — Returns the current epoch number.
- `current_epoch_info(&self) -> EpochInfo` — Returns a clone of the current epoch's full state.
- `current_phase(&self) -> EpochPhase` — Returns the current phase of the current epoch.
- `genesis_l1_height(&self) -> u32` — Returns the network's genesis L1 height.
- `network_id(&self) -> &Bytes32` — Returns the network ID.
- `epoch_for_l1_height(&self, l1_height: u32) -> u64` — Maps an L1 height to its epoch number.
- `l1_range_for_epoch(&self, epoch: u64) -> (u32, u32)` — Returns (start, end) L1 heights for an epoch.
- `store_rewards(&self, distribution: RewardDistribution)` — Archives per-epoch reward distribution, keyed by epoch number.

These methods provide essential accessor and utility functionality on the EpochManager instance.

**Spec Reference:** Sections 6.3, 6.8, 6.9
