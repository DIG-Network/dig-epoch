# Epoch Manager - Verification Matrix

| Field       | Value          |
|-------------|----------------|
| Domain      | epoch_manager  |
| Prefix      | MGR            |
| Total Items | 8              |
| Status      | Draft          |
| Spec        | [SPEC.md](../../../resources/SPEC.md) |

| ID      | Status  | Summary                              | Verification Approach                                                        |
|---------|---------|--------------------------------------|------------------------------------------------------------------------------|
| MGR-001 | Pending | EpochManager Struct                  | Verify parking_lot::RwLock usage; test concurrent reads succeed; test exclusive write blocks concurrent writes; verify all inner fields present |
| MGR-002 | Pending | record_block                         | Unit test increments blocks_produced, total_fees, total_transactions; test PhaseMismatch when not in BlockProduction; test multiple sequential calls accumulate correctly |
| MGR-003 | Pending | set_current_epoch_chain_totals       | Unit test overwrites blocks_produced, total_fees, total_transactions to exact values; verify does not add to existing totals; test idempotency |
| MGR-004 | Pending | advance_epoch                        | Test EpochNotComplete when phase != Complete; test NoFinalizedCheckpoint when no checkpoint; verify EpochSummary archived; verify new EpochInfo for epoch+1 with state_root; verify competition reset; verify returns new epoch number (u64) |
| MGR-005 | Pending | Query Methods                        | Unit test get_epoch_info() returns clone; test get_epoch_summary() returns correct epoch; test recent_summaries(n) returns last n; test total_stats() aggregates; test get_rewards() |
| MGR-006 | Pending | DFSP Close Snapshot                  | Unit test sets DfspCloseSnapshot on current EpochInfo; test requires Finalization phase; test error when called in wrong phase |
| MGR-007 | Pending | Epoch History Management             | Verify summaries ordered by epoch; verify append-only behavior; test recent_summaries returns from tail; advance multiple epochs and verify history integrity |
| MGR-008 | gap | Core Instance Methods and Accessors | Unit test: verify current_epoch() returns current epoch number. Verify current_epoch_info() returns clone of current EpochInfo. Verify current_phase() returns current phase. Verify genesis_l1_height() returns genesis L1 height. Verify network_id() returns network ID. Verify epoch_for_l1_height() maps L1 height to epoch. Verify l1_range_for_epoch() returns correct L1 range. Verify store_rewards() archives distribution and get_rewards() retrieves it. |
