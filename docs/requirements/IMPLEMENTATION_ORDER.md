# Implementation Order

Phased checklist for dig-epoch requirements. Work top-to-bottom within each phase.
After completing a requirement: write tests, verify they pass, update TRACKING.yaml, VERIFICATION.md, and check off here.

**A requirement is NOT complete until comprehensive tests verify it.**

---

## Phase 0: Crate Structure & Foundation

- [x] STR-001 — Cargo.toml with DIG/Chia crate dependencies and metadata
- [x] STR-002 — Module hierarchy matching SPEC Section 13
- [ ] STR-003 — Public re-exports (EpochManager, all types, constants, functions)
- [ ] STR-004 — EpochManager constructor (new with network_id, genesis_l1_height, initial_state_root)
- [ ] STR-005 — Test infrastructure (test EpochManager, test blocks, helpers)

## Phase 1: Constants

- [x] CON-001 — Epoch geometry constants (BLOCKS_PER_EPOCH, EPOCH_L1_BLOCKS, GENESIS_HEIGHT)
- [x] CON-002 — Phase boundary constants (50%, 75%, 100% thresholds)
- [ ] CON-003 — Reward economics constants (MOJOS_PER_L2, INITIAL_BLOCK_REWARD, halvings, tail, bonus)
- [ ] CON-004 — Fee and reward distribution percentages (5-role split)
- [ ] CON-005 — DFSP, consensus, slashing, and withdrawal constants
- [ ] CON-006 — Sentinel constants (EMPTY_ROOT)

## Phase 2: Error Types

- [ ] ERR-001 — EpochError enum with all variants
- [ ] ERR-002 — CheckpointCompetitionError enum
- [ ] ERR-003 — Error From conversions and Display messages

## Phase 3: Epoch Types

- [ ] TYP-001 — EpochPhase enum and PhaseTransition struct
- [ ] TYP-002 — EpochInfo struct with all fields
- [ ] TYP-003 — EpochSummary struct (immutable archive)
- [ ] TYP-004 — DfspCloseSnapshot struct
- [ ] TYP-005 — EpochEvent enum
- [ ] TYP-006 — EpochBlockLink struct

## Phase 4: Height-Epoch Arithmetic

- [ ] HEA-001 — epoch_for_block_height(h) -> u64
- [ ] HEA-002 — first_height_in_epoch(e) and epoch_checkpoint_height(e)
- [ ] HEA-003 — is_genesis_checkpoint_block(h), is_epoch_checkpoint_block(h), is_checkpoint_class_block(h), and ensure_checkpoint_block_empty()
- [ ] HEA-004 — l1_range_for_epoch(genesis_l1_height, epoch) -> (u32, u32)
- [ ] HEA-005 — Height-epoch round-trip identity property

## Phase 5: Phase State Machine

- [ ] PHS-001 — l1_progress_phase_for_network_epoch() free function
- [ ] PHS-002 — EpochManager phase tracking (current_phase, update_phase)
- [ ] PHS-003 — Phase transition events and should_advance()
- [ ] PHS-004 — Phase boundary enforcement (PhaseMismatch errors)

## Phase 6: Reward Economics

- [ ] REW-001 — block_reward_at_height() with halving schedule
- [ ] REW-002 — total_block_reward() with epoch-first-block bonus
- [ ] REW-003 — proposer_fee_share() and burned_fee_remainder()
- [ ] REW-004 — compute_reward_distribution() with 5-role split
- [ ] REW-005 — Tail emission floor (MINIMUM_EPOCH_REWARD)
- [ ] REW-006 — Halving interval boundary verification

## Phase 7: Epoch Manager

- [ ] MGR-001 — EpochManager struct with interior mutability (RwLock)
- [ ] MGR-002 — record_block(fees, tx_count)
- [ ] MGR-003 — set_current_epoch_chain_totals(blocks, fees, txns)
- [ ] MGR-004 — advance_epoch(l1_height, state_root)
- [ ] MGR-005 — Query methods (get_epoch_info, get_epoch_summary, recent_summaries, total_stats)
- [ ] MGR-006 — set_current_epoch_dfsp_close_snapshot()
- [ ] MGR-007 — Epoch history management (summaries storage)

## Phase 8: Checkpoint Competition

- [ ] CKP-001 — CheckpointCompetition struct and CompetitionStatus enum
- [ ] CKP-002 — start_checkpoint_competition()
- [ ] CKP-003 — submit_checkpoint() with score comparison
- [ ] CKP-004 — finalize_competition() and get_competition()
- [ ] CKP-005 — Competition lifecycle (Pending → Collecting → WinnerSelected → Finalized/Failed)

## Phase 9: Verification

- [ ] VER-001 — compute_epoch_block_root() via chia-sdk-types::MerkleTree
- [ ] VER-002 — epoch_block_inclusion_proof() via MerkleProof
- [ ] VER-003 — compute_epoch_withdrawals_root() via chia-consensus::compute_merkle_set_root
- [ ] VER-004 — EpochCheckpointData and EpochCheckpointSignMaterial
- [ ] VER-005 — stored_checkpoint_from_epoch_sign_material_with_aggregate_v1()

## Phase 10: DFSP Processing

- [ ] DFS-001 — DfspEpochBurnPolicyV1 and burn context
- [ ] DFS-002 — Storage proof evaluation context and issuance preview
- [ ] DFS-003 — Epoch boundary finalize preview and staged outputs
- [ ] DFS-004 — Finalize roots commitment digest computation
- [ ] DFS-005 — DFSP activation control (is_dfsp_active_at_height, dfsp_activation_height_for_network)
- [ ] DFS-006 — DFSP namespace rollup and tail roots computation
- [ ] DFS-007 — Parse burn policy schedule from string configuration
- [ ] DFS-008 — Storage proof evaluation step (apply_epoch_storage_proof_evaluation_step_v1)

## Phase 11: Serialization

- [ ] SER-001 — Bincode serialization for all epoch types
- [ ] SER-002 — to_bytes/from_bytes conventions
- [ ] SER-003 — Round-trip integrity for all serializable types

---

## Summary

| Phase | Domain(s) | Requirements |
|-------|-----------|-------------|
| 0 | Crate Structure | STR-001 — STR-005 (5) |
| 1 | Constants | CON-001 — CON-006 (6) |
| 2 | Error Types | ERR-001 — ERR-003 (3) |
| 3 | Epoch Types | TYP-001 — TYP-006 (6) |
| 4 | Height-Epoch Arithmetic | HEA-001 — HEA-005 (5) |
| 5 | Phase State Machine | PHS-001 — PHS-004 (4) |
| 6 | Reward Economics | REW-001 — REW-006 (6) |
| 7 | Epoch Manager | MGR-001 — MGR-007 (7) |
| 8 | Checkpoint Competition | CKP-001 — CKP-005 (5) |
| 9 | Verification | VER-001 — VER-005 (5) |
| 10 | DFSP Processing | DFS-001 — DFS-008 (8) |
| 11 | Serialization | SER-001 — SER-003 (3) |
| **Total** | | **63** |
