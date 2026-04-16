# Epoch Types - Normative Requirements

> **Domain:** epoch_types
> **Prefix:** TYP
> **Spec reference:** [SPEC.md - Sections 3.2-3.7, 3.13](../../../resources/SPEC.md)

## Requirements

### TYP-001: EpochPhase and PhaseTransition

EpochPhase MUST define the following variants: BlockProduction, Checkpoint, Finalization, Complete. EpochPhase MUST derive Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize.

PhaseTransition MUST contain the following fields:

- `epoch: u64` - the epoch number in which the transition occurred
- `from: EpochPhase` - the phase being transitioned from
- `to: EpochPhase` - the phase being transitioned to
- `l1_height: u32` - the L1 height at which the transition occurred

PhaseTransition MUST derive Debug, Clone.

EpochPhase MUST implement the following methods:

- `index(&self) -> usize` — Returns the phase index: BlockProduction=0, Checkpoint=1, Finalization=2, Complete=3.
- `next(&self) -> Option<EpochPhase>` — Returns the next phase: BlockProduction->Some(Checkpoint), Checkpoint->Some(Finalization), Finalization->Some(Complete), Complete->None.
- `previous(&self) -> Option<EpochPhase>` — Returns the previous phase: Complete->Some(Finalization), Finalization->Some(Checkpoint), Checkpoint->Some(BlockProduction), BlockProduction->None.
- `name(&self) -> &'static str` — Returns the phase name: "BlockProduction", "Checkpoint", "Finalization", "Complete".
- `allows_block_production(&self) -> bool` — Returns true only for BlockProduction.
- `allows_checkpoint_submission(&self) -> bool` — Returns true only for Checkpoint.
- `allows_finalization(&self) -> bool` — Returns true only for Finalization.

**Spec reference:** SPEC Sections 3.2, 3.3

### TYP-002: EpochInfo Struct

EpochInfo MUST contain the following fields:

**Identity fields:**
- `epoch: u64` - epoch number (0-indexed)
- `start_l1_height: u32` - first L1 height in this epoch's window
- `end_l1_height: u32` - last L1 height in this epoch's window
- `start_l2_height: u64` - first L2 block height in this epoch

**Mutable counters:**
- `blocks_produced: u32` - L2 blocks recorded so far
- `phase: EpochPhase` - current phase
- `total_fees: u64` - accumulated fees (mojos)
- `total_transactions: u64` - accumulated transaction count

**State:**
- `checkpoint: Option<Checkpoint>` - winning checkpoint (set after finalization)
- `start_state_root: Bytes32` - CoinSet state root at epoch start

**DFSP close snapshot fields:**
- `collateral_registry_root: Bytes32` - collateral registry SMT root at close
- `cid_state_root: Bytes32` - CID lifecycle state root at close
- `node_registry_root: Bytes32` - node registry SMT root at close
- `namespace_epoch_root: Bytes32` - cumulative namespace root at close
- `dfsp_issuance_total: u64` - total DFSP issuance this epoch (mojos)
- `active_cid_count: u32` - active CIDs at close
- `active_node_count: u32` - active storage nodes at close

EpochInfo MUST derive Debug, Clone, Serialize, Deserialize.

EpochInfo holds mutable state for the current epoch. DFSP roots default to EMPTY_ROOT and counts default to 0 until `set_current_epoch_dfsp_close_snapshot()` is called.

EpochInfo MUST implement the following methods:

- `new(epoch: u64, start_l1_height: u32, start_l2_height: u64, start_state_root: Bytes32) -> Self` — Creates a new epoch with BlockProduction phase and zeroed counters. DFSP roots default to EMPTY_ROOT. end_l1_height = start_l1_height + EPOCH_L1_BLOCKS.
- `calculate_phase(&self, current_l1_height: u32) -> EpochPhase` — Deterministically computes phase from L1 progress percentage within the epoch window.
- `target_blocks(&self) -> u64` — Returns BLOCKS_PER_EPOCH.
- `can_produce_blocks(&self) -> bool` — True when phase is BlockProduction.
- `can_submit_checkpoint(&self) -> bool` — True when phase is Checkpoint.
- `is_complete(&self) -> bool` — True when phase is Complete.
- `is_finalized(&self) -> bool` — True when checkpoint.is_some().
- `record_block(&mut self, fees: u64, tx_count: u64)` — Increments blocks_produced, total_fees, total_transactions.
- `set_checkpoint(&mut self, checkpoint: Checkpoint)` — Sets the winning checkpoint.
- `progress_percentage(&self, current_l1_height: u32) -> u32` — Returns 0-100 based on L1 progress within the epoch window.

**Spec reference:** SPEC Section 3.4

### TYP-003: EpochSummary Struct

EpochSummary MUST contain the following fields:

- `epoch: u64` - epoch number
- `blocks: u32` - total L2 blocks produced in the epoch
- `transactions: u64` - total transaction count
- `fees: u64` - total fees collected (mojos)
- `finalized: bool` - whether the epoch was finalized with a checkpoint
- `checkpoint_hash: Option<Bytes32>` - hash of the finalized checkpoint, if any

**DFSP state at close:**
- `collateral_registry_root: Bytes32`
- `cid_state_root: Bytes32`
- `node_registry_root: Bytes32`
- `namespace_epoch_root: Bytes32`
- `dfsp_issuance_total: u64`
- `active_cid_count: u32`
- `active_node_count: u32`

EpochSummary MUST derive Debug, Clone, Serialize, Deserialize.

EpochSummary is an immutable archive of a completed epoch. It is created from EpochInfo when `advance_epoch()` is called. `finalized` MUST be `checkpoint.is_some()` from the source EpochInfo. `checkpoint_hash` MUST be `checkpoint.map(|c| c.hash())`.

**Spec reference:** SPEC Section 3.5

### TYP-004: DfspCloseSnapshot Struct

DfspCloseSnapshot MUST contain the following 7 fields:

- `collateral_registry_root: Bytes32` - collateral registry SMT root at close
- `cid_state_root: Bytes32` - CID lifecycle state root at close
- `node_registry_root: Bytes32` - node registry SMT root at close
- `namespace_epoch_root: Bytes32` - cumulative namespace root at close
- `dfsp_issuance_total: u64` - total DFSP issuance this epoch (mojos)
- `active_cid_count: u32` - active CIDs at close
- `active_node_count: u32` - active storage nodes at close

DfspCloseSnapshot MUST derive Debug, Clone, Copy.

DfspCloseSnapshot captures DFSP state at epoch close before archival. It is applied to the current EpochInfo via `set_current_epoch_dfsp_close_snapshot()` before `advance_epoch()` archives the epoch.

**Spec reference:** SPEC Section 3.6

### TYP-005: EpochEvent Enum

EpochEvent MUST define the following variants:

- `EpochStarted { epoch: u64, l1_height: u32 }` - emitted when a new epoch begins
- `PhaseChanged { epoch: u64, from: EpochPhase, to: EpochPhase, l1_height: u32 }` - emitted on phase transition
- `EpochFinalized { epoch: u64, checkpoint: Checkpoint }` - emitted when epoch finalization completes
- `EpochFailed { epoch: u64 }` - emitted when epoch finalization fails

EpochEvent MUST derive Debug, Clone.

EpochEvent variants are emitted by the EpochManager for downstream notification (telemetry, logging, driver coordination).

**Spec reference:** SPEC Section 3.7

### TYP-006: EpochBlockLink Struct

EpochBlockLink MUST contain the following fields:

- `parent_hash: Bytes32` - hash of the parent block
- `block_hash: Bytes32` - hash of the current block

EpochBlockLink MUST derive Debug, Clone, Serialize, Deserialize.

EpochBlockLink records the parent-to-child relationship between consecutive blocks within an epoch. It is used for block continuity validation in L1 finalization payloads.

**Spec reference:** SPEC Section 3.13

### TYP-007: EpochStats Struct

EpochStats MUST contain the following 5 fields:

- `total_epochs: u64` - total number of epochs managed
- `finalized_epochs: u64` - number of epochs that were finalized with a checkpoint
- `total_blocks: u64` - total L2 blocks across all epochs
- `total_transactions: u64` - total transaction count across all epochs
- `total_fees: u64` - total fees collected across all epochs (mojos)

EpochStats MUST derive Debug, Clone, Default.

EpochStats provides aggregate statistics across all epochs managed by the EpochManager. The Default implementation MUST zero all fields.

**Spec reference:** SPEC Section 3.8
