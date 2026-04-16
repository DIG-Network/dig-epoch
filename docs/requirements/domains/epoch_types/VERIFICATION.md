# Epoch Types - Verification Matrix

> **Domain:** epoch_types
> **Prefix:** TYP
> **Normative:** [NORMATIVE.md](./NORMATIVE.md)
> **Tracking:** [TRACKING.yaml](./TRACKING.yaml)

| ID | Status | Summary | Verification Approach |
|----|--------|---------|----------------------|
| TYP-001 | gap | EpochPhase and PhaseTransition | Unit test: construct each EpochPhase variant (BlockProduction, Checkpoint, Finalization, Complete). Verify Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize derives. Construct PhaseTransition with all fields, verify correct storage and Debug output. |
| TYP-002 | gap | EpochInfo Struct | Unit test: construct EpochInfo via new() with epoch, start_l1_height, start_l2_height, start_state_root. Verify BlockProduction phase, zeroed counters, EMPTY_ROOT for all DFSP roots, None checkpoint, end_l1_height = start_l1_height + EPOCH_L1_BLOCKS. Verify all 17 fields accessible and correctly typed. |
| TYP-003 | gap | EpochSummary Struct | Unit test: create EpochSummary from EpochInfo. Verify finalized = checkpoint.is_some(). Verify checkpoint_hash = checkpoint.map(c.hash()). Verify all 13 fields including 7 DFSP fields are correctly mapped from EpochInfo. |
| TYP-004 | gap | DfspCloseSnapshot Struct | Unit test: construct DfspCloseSnapshot with all 7 fields. Verify Clone and Copy semantics. Apply via set_current_epoch_dfsp_close_snapshot() and verify all fields appear in EpochInfo and survive into archived EpochSummary. |
| TYP-005 | gap | EpochEvent Enum | Unit test: construct all four EpochEvent variants (EpochStarted, PhaseChanged, EpochFinalized, EpochFailed). Verify pattern matching works. Verify Debug and Clone derives. Verify each variant carries its expected fields. |
| TYP-006 | gap | EpochBlockLink Struct | Unit test: construct EpochBlockLink with parent_hash and block_hash Bytes32 values. Verify Debug, Clone, Serialize, Deserialize derives. Verify round-trip serialization preserves both fields. |
| TYP-007 | gap | EpochStats Struct | Unit test: construct EpochStats with all 5 fields (total_epochs, finalized_epochs, total_blocks, total_transactions, total_fees). Verify Debug, Clone, Default derives. Verify Default zeroes all fields. Verify EpochManager::total_stats() returns correct aggregates. |
