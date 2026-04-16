# Constants - Verification Matrix

> **Domain:** constants
> **Prefix:** CON
> **Normative:** [NORMATIVE.md](./NORMATIVE.md)
> **Tracking:** [TRACKING.yaml](./TRACKING.yaml)

| ID | Status | Summary | Verification Approach |
|----|--------|---------|----------------------|
| CON-001 | verified | Epoch Geometry | Unit test: verify BLOCKS_PER_EPOCH == 32, EPOCH_L1_BLOCKS == 32, GENESIS_HEIGHT == 1. Verify all three are publicly accessible from the crate root. **Evidence:** `tests/constants/con_001_test.rs` (7/7 passing, 2026-04-16) proves the three constants have the spec-mandated types (`u64`/`u32`/`u64`) and values (32/32/1) and that height-to-epoch arithmetic places heights 1, 32, 33, 64 in the correct epochs. |
| CON-002 | verified | Phase Boundaries | Unit test: verify PHASE_BLOCK_PRODUCTION_END_PCT == 50, PHASE_CHECKPOINT_END_PCT == 75, PHASE_FINALIZATION_END_PCT == 100. Verify boundaries are monotonically increasing and partition the 0-100% range into three intervals. **Evidence:** `tests/constants/con_002_test.rs` (7/7 passing, 2026-04-16) proves the three constants have the spec-mandated `u32` types and values (50/75/100), are strictly ascending, and produce the asymmetric window widths (50/25/25 pp). |
| CON-003 | verified | Reward Economics Constants | Unit test: verify each constant matches its specified value. Cross-check derived relationships: INITIAL_EPOCH_REWARD == BLOCKS_PER_EPOCH * INITIAL_BLOCK_REWARD, INITIAL_BLOCK_REWARD == MOJOS_PER_L2 * 64 / 200 (from emission rate derivation). Verify TAIL_BLOCK_REWARD < INITIAL_BLOCK_REWARD. Verify HALVINGS_BEFORE_TAIL == 4. **Evidence:** `tests/constants/con_003_test.rs` (14/14 passing, 2026-04-16) covers all 13 constants' values and the four emission derivation relationships. |
| CON-004 | verified | Fee and Reward Distribution | Unit test: verify each constant matches its specified value. Cross-check: FEE_PROPOSER_SHARE_PCT + FEE_BURN_SHARE_PCT == 100. Cross-check: PROPOSER_REWARD_SHARE + ATTESTER_REWARD_SHARE + EF_SPAWNER_REWARD_SHARE + SCORE_SUBMITTER_REWARD_SHARE + FINALIZER_REWARD_SHARE == 100. **Evidence:** `tests/constants/con_004_test.rs` (9/9 passing, 2026-04-16) covers all 7 constants' values plus both sum-to-100 invariants. |
| CON-005 | gap | DFSP, Consensus, Slashing Constants | Unit test: verify each constant matches its specified value. Cross-check DFSP derivations: DFSP_WALL_CLOCK_EPOCH_SECONDS == (BLOCKS_PER_EPOCH * 3000) / 1000. Verify DFSP_ACTIVATION_HEIGHT == u64::MAX (disabled by default). Verify all consensus thresholds are 67%. Verify DFSP_GENESIS_ISSUANCE_SUBSIDY_MOJOS_V1 is u128. Verify CORRELATION_WINDOW_EPOCHS is u32. Verify DIG_DFSP_ACTIVATION_HEIGHT_ENV and DFSP_SLASH_LOOKBACK_EPOCHS are present. |
| CON-006 | gap | Sentinel Constants | Unit test: verify EMPTY_ROOT matches SHA-256 of the empty string. Compute SHA-256 of `b""` at runtime and compare. Verify EMPTY_ROOT is publicly accessible from the crate root. |
