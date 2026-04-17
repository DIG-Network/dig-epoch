# dig-epoch

DIG L2 epoch geometry, phase machine, manager, checkpoint competition, reward
economics, verification, and serialization — as a single Rust crate.

- **Canonical spec:** [`docs/resources/SPEC.md`](docs/resources/SPEC.md)
- **Requirements tree:** [`docs/requirements/README.md`](docs/requirements/README.md)
- **Implementation checklist:** [`docs/requirements/IMPLEMENTATION_ORDER.md`](docs/requirements/IMPLEMENTATION_ORDER.md)
- **Project notes:** [`CLAUDE.md`](CLAUDE.md)

This README is the authoritative entry point for consumers of the public
API. Everything listed under `Public interface` below is reachable via
`use dig_epoch::<name>` — no submodule paths required.

---

## What this crate does

Given an L1-anchored blockchain with a fixed cadence, dig-epoch is the state
machine that partitions time into **epochs** and drives each epoch through
four phases:

```
BlockProduction (0–50%)  →  Checkpoint (50–75%)  →  Finalization (75–100%)  →  Complete
```

Each epoch has:

- An L1 window of `EPOCH_L1_BLOCKS` heights (genesis + `epoch * EPOCH_L1_BLOCKS`).
- An L2 window of `BLOCKS_PER_EPOCH` blocks (first = `epoch * BLOCKS_PER_EPOCH + 1`).
- A `CheckpointCompetition` that collects submissions and selects a winner.
- A reward split (5 roles + 50/50 fee split) derived by the economic rules.
- A `DfspCloseSnapshot` of the storage-protocol state at epoch close.
- An `EpochSummary` archived at `advance_epoch`.

It does **not** redefine block types — `Checkpoint` / `CheckpointSubmission`
come from `dig-block`; `Bytes32` comes from `chia-protocol`. Both are
re-exported from `dig_epoch` for single-crate ergonomics.

---

## Installation

```toml
[dependencies]
dig-epoch = "0.1"
```

Transitive deps: `chia-protocol`, `chia-bls`, `chia-consensus`, `chia-sha2`,
`chia-sdk-types`, `chia-sdk-signer`, `clvm-utils`, `dig-block`,
`dig-constants`, `bincode` (v1), `serde`, `thiserror`, `parking_lot`.

Pins are intentional — DIG crates track `0.1`, Chia low-level crates track
`0.26`, Chia SDK crates track `0.30`. See `Cargo.toml` header for rationale.

---

## Quick start

```rust
use dig_epoch::{
    Bytes32, EpochManager, EpochPhase, DfspCloseSnapshot,
    compute_reward_distribution, epoch_reward_with_floor, total_block_reward,
    BLOCKS_PER_EPOCH,
};
use dig_epoch::test_helpers::mock_checkpoint_submission;

fn run() -> Result<(), dig_epoch::EpochError> {
    let network_id = Bytes32::new([0x01; 32]);
    let initial_state_root = Bytes32::new([0xAA; 32]);
    let m = EpochManager::new(network_id, /* genesis_l1 */ 100, initial_state_root);

    // Phase 1: produce blocks.
    for _ in 0..16 {
        m.record_block(/* fees */ 500, /* tx_count */ 2)?;
    }

    // Cross 50% → Checkpoint. Start the competition, submit, confirm winner.
    m.update_phase(116);
    m.start_checkpoint_competition()?;
    m.submit_checkpoint(mock_checkpoint_submission(0, 75, 16))?;

    // Cross 75% → Finalization. Stage DFSP snapshot + finalize the winner.
    m.update_phase(124);
    m.set_current_epoch_dfsp_close_snapshot(DfspCloseSnapshot {
        collateral_registry_root: Bytes32::new([0; 32]),
        cid_state_root: Bytes32::new([0; 32]),
        node_registry_root: Bytes32::new([0; 32]),
        namespace_epoch_root: Bytes32::new([0; 32]),
        dfsp_issuance_total: 0,
        active_cid_count: 0,
        active_node_count: 0,
    })?;
    m.finalize_competition(/* epoch */ 0, /* l1_height */ 132)?;

    // Cross 100% → Complete. Store rewards, then advance.
    m.update_phase(133);
    let total_fees = m.current_epoch_info().total_fees;
    let base: u64 = (1..=BLOCKS_PER_EPOCH).map(|h| total_block_reward(h, h == 1)).sum();
    let total_reward = epoch_reward_with_floor(base);
    m.store_rewards(compute_reward_distribution(0, total_reward, total_fees));
    let next = m.advance_epoch(133, Bytes32::new([0xFE; 32]))?;
    assert_eq!(next, 1);
    Ok(())
}
```

A full end-to-end run lives in
[`tests/integration/end_to_end_test.rs`](tests/integration/end_to_end_test.rs).

---

## Public interface

### Types

| Name                                   | Kind   | Role                                                                         |
|----------------------------------------|--------|------------------------------------------------------------------------------|
| `EpochManager`                         | struct | Stateful orchestrator. Owns current epoch + history + competition + rewards. |
| `EpochPhase`                           | enum   | `BlockProduction` / `Checkpoint` / `Finalization` / `Complete`.              |
| `PhaseTransition`                      | struct | `{ epoch, from, to, l1_height }` — emitted by `update_phase`.                |
| `EpochInfo`                            | struct | 17-field mutable state for the current epoch.                                |
| `EpochSummary`                         | struct | 13-field immutable archive of a completed epoch.                             |
| `DfspCloseSnapshot`                    | struct | 7-field DFSP state captured at epoch close.                                  |
| `EpochEvent`                           | enum   | `EpochStarted` / `PhaseChanged` / `EpochFinalized` / `EpochFailed`.          |
| `EpochStats`                           | struct | Aggregate stats across all epochs (total_epochs, blocks, fees, txns).        |
| `EpochBlockLink`                       | struct | Parent/child block-hash pair for epoch chains.                               |
| `CheckpointCompetition`                | struct | Per-epoch submission set + status.                                           |
| `CompetitionStatus`                    | enum   | `Pending` / `Collecting` / `WinnerSelected` / `Finalized` / `Failed`.        |
| `RewardDistribution`                   | struct | 8-field per-epoch reward allocation (5 roles + fee split).                   |
| `EpochCheckpointData`                  | struct | Network-bound checkpoint identity (network_id + roots + hash).               |
| `EpochCheckpointSignMaterial`          | struct | `EpochCheckpointData` + score + signing_digest for BLS.                      |
| `EpochError`                           | enum   | Primary error type (see **Errors** below).                                   |
| `CheckpointCompetitionError`           | enum   | Competition-specific errors; wraps into `EpochError::Competition`.           |
| `Bytes32` *(re-exported)*              | struct | From `chia-protocol`. 32-byte hash.                                          |
| `Checkpoint` *(re-exported)*           | struct | From `dig-block`. Epoch summary wire type.                                   |
| `CheckpointSubmission` *(re-exported)* | struct | From `dig-block`. Signed checkpoint + score + submitter.                     |

### `EpochManager` methods

Construction:

| Signature | Description |
|-----------|-------------|
| `new(network_id: Bytes32, genesis_l1_height: u32, initial_state_root: Bytes32) -> Self` | Starts at epoch 0, `BlockProduction`. Empty history/competitions/rewards. |

Accessors (read locks; concurrent-safe):

| Signature | Returns |
|-----------|---------|
| `current_epoch(&self) -> u64` | Current epoch number. |
| `current_epoch_info(&self) -> EpochInfo` | Clone of current state. |
| `current_phase(&self) -> EpochPhase` | Current phase. |
| `genesis_l1_height(&self) -> u32` | Immutable genesis L1 height. |
| `network_id(&self) -> Bytes32` | Immutable network ID. |
| `epoch_for_l1_height(&self, l1_height: u32) -> u64` | L1 → epoch mapping. |
| `l1_range_for_epoch(&self, epoch: u64) -> (u32, u32)` | Inclusive L1 window. |
| `competition(&self) -> CheckpointCompetition` | Clone of current competition. |
| `should_advance(&self, _l1_height: u32) -> bool` | `true` when phase is `Complete`. |

Writes (write locks):

| Signature | Effect |
|-----------|--------|
| `update_phase(&self, l1_height: u32) -> Option<PhaseTransition>` | Recalculates phase; `Some` on transition. |
| `record_block(&self, fees: u64, tx_count: u64) -> Result<(), EpochError>` | Increments counters. Requires `BlockProduction`. |
| `set_current_epoch_chain_totals(&self, blocks: u32, fees: u64, txns: u64)` | Overwrites counters (no phase gate). For resync. |
| `set_current_epoch_dfsp_close_snapshot(&self, snap: DfspCloseSnapshot) -> Result<(), EpochError>` | Applies DFSP close. Requires `Finalization`. |
| `advance_epoch(&self, _l1_height: u32, state_root: Bytes32) -> Result<u64, EpochError>` | Archives summary, creates `epoch+1`. Requires `Complete` + finalized competition. |

Checkpoint competition (SPEC §6.5):

| Signature | Effect |
|-----------|--------|
| `start_checkpoint_competition(&self) -> Result<(), EpochError>` | `Pending` → `Collecting`. Requires `Checkpoint` phase. |
| `submit_checkpoint(&self, submission: CheckpointSubmission) -> Result<bool, EpochError>` | Returns `true` when submission beats current leader. Records every submission. |
| `finalize_competition(&self, epoch: u64, l1_height: u32) -> Result<Option<Checkpoint>, EpochError>` | `WinnerSelected` → `Finalized`. Sets checkpoint on `EpochInfo`. `Ok(None)` if no submissions (→ `Failed`). |
| `get_competition(&self, epoch: u64) -> Option<CheckpointCompetition>` | Returns clone when `epoch` matches current competition. |

Query + history (SPEC §6.7, §6.9):

| Signature | Returns |
|-----------|---------|
| `get_epoch_info(&self) -> EpochInfo` | Alias for `current_epoch_info`. |
| `get_epoch_summary(&self, epoch: u64) -> Option<EpochSummary>` | Archived summary for completed epoch. |
| `recent_summaries(&self, n: usize) -> Vec<EpochSummary>` | Last `n` in ascending epoch order. |
| `total_stats(&self) -> EpochStats` | Aggregates across archived + current. |
| `get_rewards(&self, epoch: u64) -> Option<RewardDistribution>` | Looks up stored rewards. |
| `store_rewards(&self, distribution: RewardDistribution)` | Archives rewards keyed by `distribution.epoch`. |

### Free functions

**Height arithmetic** (`arithmetic`, HEA-*):

```rust
epoch_for_block_height(height: u64) -> u64
first_height_in_epoch(epoch: u64) -> u64
epoch_checkpoint_height(epoch: u64) -> u64
last_committed_height_in_epoch(epoch: u64, tip_height: u64) -> u64
is_genesis_checkpoint_block(height: u64) -> bool
is_epoch_checkpoint_block(height: u64) -> bool
is_checkpoint_class_block(height: u64) -> bool
is_first_block_after_epoch_checkpoint(height: u64) -> bool
l1_range_for_epoch(genesis_l1_height: u32, epoch: u64) -> (u32, u32)
ensure_checkpoint_block_empty(height: u64, bundles: u32, cost: u64, fees: u64) -> Result<(), EpochError>
```

**Phase** (`phase`, PHS-001):

```rust
l1_progress_phase_for_network_epoch(
    genesis_l1_height: u32,
    network_epoch: u64,
    current_l1_height: u32,
) -> EpochPhase
```

**Reward economics** (`rewards`, REW-*):

```rust
block_reward_at_height(height: u64) -> u64
total_block_reward(height: u64, is_first_of_epoch: bool) -> u64
proposer_fee_share(total_fees: u64) -> u64
burned_fee_remainder(total_fees: u64) -> u64
compute_reward_distribution(epoch: u64, total_reward: u64, total_fees: u64) -> RewardDistribution
epoch_reward_with_floor(computed_epoch_reward: u64) -> u64
```

**Verification** (`verification`, VER-*):

```rust
compute_epoch_block_root(block_hashes: &[Bytes32]) -> Bytes32
epoch_block_inclusion_proof(block_hashes: &[Bytes32], index: usize) -> Option<MerkleProof>
verify_block_inclusion_proof(leaf: Bytes32, proof: &MerkleProof, root: Bytes32) -> bool
compute_epoch_withdrawals_root(withdrawal_hashes: &[Bytes32]) -> Bytes32
epoch_checkpoint_sign_material_from_l2_blocks(
    network_id: Bytes32,
    epoch: u64,
    block_hashes: &[Bytes32],
    state_root: Bytes32,
    withdrawal_hashes: &[Bytes32],
    prev_checkpoint: Bytes32,
    total_fees: u64,
    tx_count: u64,
    stake_percentage: u64,
) -> EpochCheckpointSignMaterial
stored_checkpoint_from_epoch_sign_material_with_aggregate_v1(
    material: &EpochCheckpointSignMaterial,
    validator_set: &[(u32, PublicKey)],
    per_validator: &[(u32, PublicKey, Signature)],
    submitter: u32,
) -> Result<CheckpointSubmission, EpochError>
```

`MerkleProof` is `chia_sdk_types::MerkleProof`. `PublicKey` / `Signature`
are `chia_bls::PublicKey` / `chia_bls::Signature`.

### Test helpers (`test_helpers`)

Exposed as `pub mod` so integration tests can reuse deterministic fixtures.
**Not production-safe** — synthetic BLS signatures do not verify.

```rust
test_network_id() -> Bytes32
test_initial_state_root() -> Bytes32
TEST_GENESIS_L1_HEIGHT: u32 = 100
test_epoch_manager() -> EpochManager
advance_through_phases(&EpochManager) -> Vec<PhaseTransition>
mock_checkpoint_submission(epoch: u64, stake_percentage: u64, block_count: u32) -> CheckpointSubmission
build_n_block_epoch(&EpochManager, n: u32, fee_per_block: u64, tx_per_block: u64) -> (u64, u64)
```

### Constants

**Epoch geometry:** `BLOCKS_PER_EPOCH = 32`, `EPOCH_L1_BLOCKS = 32`,
`GENESIS_HEIGHT = 1`.

**Phase boundaries:** `PHASE_BLOCK_PRODUCTION_END_PCT = 50`,
`PHASE_CHECKPOINT_END_PCT = 75`, `PHASE_FINALIZATION_END_PCT = 100`.

**Reward economics:** `MOJOS_PER_L2`, `L2_BLOCK_TIME_MS`,
`L2_BLOCKS_PER_10_MIN`, `INITIAL_EMISSION_PER_10_MIN`,
`TAIL_EMISSION_PER_10_MIN`, `INITIAL_BLOCK_REWARD`, `TAIL_BLOCK_REWARD`,
`HALVING_INTERVAL_BLOCKS`, `HALVINGS_BEFORE_TAIL = 4`,
`INITIAL_EPOCH_REWARD`, `HALVING_INTERVAL_EPOCHS`, `MINIMUM_EPOCH_REWARD`,
`EPOCH_FIRST_BLOCK_BONUS`.

**Fee + reward split:** `FEE_PROPOSER_SHARE_PCT = 50`,
`FEE_BURN_SHARE_PCT = 50`, `PROPOSER_REWARD_SHARE = 10`,
`ATTESTER_REWARD_SHARE = 80`, `EF_SPAWNER_REWARD_SHARE = 3`,
`SCORE_SUBMITTER_REWARD_SHARE = 4`, `FINALIZER_REWARD_SHARE = 3`.

**DFSP / consensus / slashing:** `DFSP_WALL_CLOCK_EPOCH_SECONDS`,
`DFSP_GRACE_PERIOD_NETWORK_EPOCHS`,
`DFSP_GENESIS_ISSUANCE_SUBSIDY_MOJOS_V1`, `DFSP_ACTIVATION_HEIGHT`,
`DIG_DFSP_ACTIVATION_HEIGHT_ENV`, `SOFT_FINALITY_THRESHOLD_PCT`,
`HARD_FINALITY_THRESHOLD_PCT`, `CHECKPOINT_THRESHOLD_PCT`,
`CORRELATION_WINDOW_EPOCHS`, `SLASH_LOOKBACK_EPOCHS`,
`DFSP_SLASH_LOOKBACK_EPOCHS`, `WITHDRAWAL_DELAY_EPOCHS`.

**Sentinel:** `EMPTY_ROOT = SHA-256("")`.

---

## Inputs and outputs

### Canonical inputs

| Input | Where it enters | Validation |
|-------|-----------------|------------|
| `network_id: Bytes32` | `EpochManager::new` | Stored immutably. Binds all sign material to prevent cross-network replay. |
| `genesis_l1_height: u32` | `EpochManager::new` | Stored immutably. Defines epoch 0's L1 window. |
| `initial_state_root: Bytes32` | `EpochManager::new` | Used as `EpochInfo::start_state_root` for epoch 0. |
| `(fees, tx_count)` | `record_block` | Added to current epoch counters. Rejected unless `BlockProduction`. |
| `l1_height: u32` | `update_phase` | Drives phase recalculation via `(l1_now - start) * 100 / EPOCH_L1_BLOCKS`. |
| `CheckpointSubmission` | `submit_checkpoint` | Rejected unless `Checkpoint` phase + competition `Collecting`/`WinnerSelected` + `submission.epoch == competition.epoch` + `score > current`. |
| `DfspCloseSnapshot` | `set_current_epoch_dfsp_close_snapshot` | Copied field-wise onto `EpochInfo`. Requires `Finalization`. |
| `(epoch, l1_height)` | `finalize_competition` | Requires `Finalization` + `epoch == competition.epoch`. |
| `state_root: Bytes32` | `advance_epoch` | Initial `start_state_root` for `epoch+1`. Requires `Complete` + finalized competition. |
| `RewardDistribution` | `store_rewards` | Keyed by `distribution.epoch`. |
| `block_hashes: &[Bytes32]` | `compute_epoch_block_root`, `epoch_block_inclusion_proof` | Ordered. Empty → `EMPTY_ROOT`. |
| `withdrawal_hashes: &[Bytes32]` | `compute_epoch_withdrawals_root` | Order-independent (Merkle set). Empty → `EMPTY_ROOT`. |
| `(validator_set, per_validator, submitter)` | `stored_checkpoint_from_epoch_sign_material_with_aggregate_v1` | BLS-aggregated. `per_validator` non-empty. |

### Canonical outputs

| Output | Produced by | Shape |
|--------|-------------|-------|
| `EpochInfo` | `current_epoch_info` / `get_epoch_info` | Clone of current state (17 fields). |
| `EpochSummary` | `get_epoch_summary`, `recent_summaries`, appended on `advance_epoch` | Immutable 13-field archive. |
| `EpochStats` | `total_stats` | Aggregate over archived + current. |
| `EpochPhase` | `current_phase`, phase free function | One of four variants. |
| `PhaseTransition` | `update_phase` | `{ epoch, from, to, l1_height }` when phase changed. |
| `CheckpointCompetition` | `competition`, `get_competition` | Clone of current competition. |
| `Option<Checkpoint>` | `finalize_competition` | Winning checkpoint or `None` when no submissions. |
| `bool` | `submit_checkpoint` | `true` when submission became leader. |
| `u64` | `advance_epoch` | New epoch number. |
| `RewardDistribution` | `compute_reward_distribution`, `get_rewards` | 8-field split. Sum of role rewards == `total_reward`. |
| `Bytes32` | Merkle root functions | 32-byte canonical hash. |
| `MerkleProof` | `epoch_block_inclusion_proof` | Leaf-keyed proof; verify with `verify_block_inclusion_proof`. |
| `EpochCheckpointData` / `EpochCheckpointSignMaterial` | `epoch_checkpoint_sign_material_from_l2_blocks` | Ready for BLS signing. |
| `Vec<u8>` | `to_bytes()` on each serializable type | bincode-encoded. |
| `Result<T, EpochError>` | Every fallible method | See **Errors** below. |

### Invariants

- **Phase gating** — each mutating method requires a specific phase. Mismatches return `EpochError::PhaseMismatch { expected, got }`.
- **Append-only history** — `summaries` only grows (via `advance_epoch`), ordered by ascending epoch, with consecutive numbers.
- **Reward sum** — `compute_reward_distribution` guarantees `proposer + attester + ef_spawner + score_submitter + finalizer == total_reward` (attester absorbs rounding).
- **Fee sum** — `proposer_fee_share(f) + burned_fee_remainder(f) == f` for all `f: u64`.
- **Checkpoint-class blocks** — `ensure_checkpoint_block_empty` rejects non-zero bundles/cost/fees at checkpoint heights.
- **Merkle roots** — empty input → `EMPTY_ROOT` (SHA-256 of empty string). Block root is order-dependent; withdrawals root is order-independent.
- **Concurrency** — all mutations go through a single `parking_lot::RwLock<EpochManagerInner>`. Reads allow concurrency; writes are exclusive; no poisoning.
- **Network binding** — `EpochCheckpointData::signing_digest()` includes `network_id` to prevent cross-network replay.
- **Serialization** — `to_bytes()` is infallible; `from_bytes()` returns `Result<Self, EpochError>`. Round-trip preserves every field across all 6 serializable types (`EpochInfo`, `EpochSummary`, `DfspCloseSnapshot`, `CheckpointCompetition`, `RewardDistribution`, `EpochCheckpointData`).

---

## Errors

```rust
pub enum EpochError {
    EpochNotComplete(u64),
    NoFinalizedCheckpoint(u64),
    CheckpointBlockNotEmpty(u64, u32, u64, u64),   // (height, bundles, cost, fees)
    PhaseMismatch { expected: EpochPhase, got: EpochPhase },
    EpochMismatch { expected: u64, got: u64 },
    InvalidHeight(u64),
    DfspNotActive(u64),
    DfspBoundary(String),
    Competition(#[from] CheckpointCompetitionError),
    InvalidData(String),                           // bincode / malformed input
}

pub enum CheckpointCompetitionError {
    InvalidData(String),
    NotFound(u64),
    ScoreNotHigher { current: u64, submitted: u64 },
    EpochMismatch { expected: u64, got: u64 },
    AlreadyFinalized,
    NotStarted,
}
```

All variants implement `std::error::Error` via `thiserror`. `EpochError`
auto-converts from `CheckpointCompetitionError` via `?`.

---

## Epoch lifecycle diagram

```
┌──────────────────────────────────────────────────────────────────────────┐
│                            EpochManager (epoch N)                        │
│                                                                          │
│   BlockProduction  ──(l1 at 50%)──►  Checkpoint  ──(l1 at 75%)──►        │
│   │  record_block                    │  start_checkpoint_competition     │
│   │  set_chain_totals                │  submit_checkpoint × K            │
│                                      │                                   │
│                                      ▼                                   │
│                               Finalization  ──(l1 at 100%)──►  Complete  │
│                               │  set_dfsp_close_snapshot       │         │
│                               │  finalize_competition          │         │
│                                                                │         │
│                                                                ▼         │
│                                                    store_rewards         │
│                                                    advance_epoch ───────►│
│                                                    ╔═════════════════════╝
│                                                    ║  epoch N+1 starts
│                                                    ║  BlockProduction
└──────────────────────────────────────────────────────────────────────────┘
```

Every phase boundary is enforced server-side: calling the wrong method
returns `EpochError::PhaseMismatch`.

---

## Module layout

| Module | Purpose |
|--------|---------|
| `constants` | Compile-time epoch/phase/reward/DFSP constants (CON-*). |
| `types` | Data types grouped by concern (TYP-*, CKP-001, REW-007, VER-004). |
| `arithmetic` | Pure height ↔ epoch math (HEA-*). |
| `phase` | Stateless L1-progress phase calculation (PHS-001). |
| `rewards` | Reward economics (REW-*). |
| `manager` | `EpochManager` struct + methods (MGR-*). |
| `verification` | Merkle roots, inclusion proofs, sign material, BLS aggregate (VER-*). |
| `dfsp` | Root-level DFSP processing (DFS-*, currently deferred). |
| `error` | `EpochError` + `CheckpointCompetitionError` (ERR-*). |
| `test_helpers` | Deterministic test fixtures (STR-005). |

`types/` submodules: `epoch_phase`, `epoch_info`, `epoch_summary`, `dfsp`,
`events`, `checkpoint_competition`, `reward`, `verification`. Each is
`pub mod` so the full type can also be addressed by its fully-qualified
path (e.g. `dig_epoch::types::checkpoint_competition::CompetitionStatus`)
in addition to the flat `dig_epoch::CompetitionStatus`.

---

## Implementation scope

| Phase | Requirements | Status |
|-------|-------------|--------|
| 0. Crate structure | STR-001..005 | Complete |
| 1. Constants | CON-001..006 | Complete |
| 2. Error types | ERR-001..003 | Complete |
| 3. Epoch types | TYP-001..007 | Complete |
| 4. Height arithmetic | HEA-001..007 | Complete |
| 5. Phase machine | PHS-001..004 | Complete |
| 6. Reward economics | REW-001..007 | Complete |
| 7. Epoch manager | MGR-001..008 | Complete |
| 8. Checkpoint competition | CKP-001..005 | Complete |
| 9. Verification | VER-001..005 | Complete |
| 10. DFSP processing | DFS-001..008 | **Deferred** (not in current scope) |
| 11. Serialization | SER-001..003 | Complete |

See [`docs/requirements/IMPLEMENTATION_ORDER.md`](docs/requirements/IMPLEMENTATION_ORDER.md)
for the checklist and each domain's `TRACKING.yaml` for per-requirement
test pointers.

---

## Testing

```bash
cargo test              # 63 test suites, 470+ tests
cargo clippy --lib -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --lib
cargo fmt --check
```

Each requirement owns its own test file under `tests/<domain>/<id>_test.rs`
(e.g. `tests/epoch_manager/mgr_004_test.rs`). The full-lifecycle cohesion
test lives in `tests/integration/end_to_end_test.rs`.

---

## License

MIT.
