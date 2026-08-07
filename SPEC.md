# dig-epoch — Normative Specification

**Crate:** `dig-epoch` 0.1.0 (Rust, edition 2021, MSRV 1.75.0)
**Status:** Normative. This document is the authoritative contract for what
`dig-epoch` implements. The key words MUST, MUST NOT, SHOULD, SHOULD NOT, and
MAY are to be interpreted as described in RFC 2119. Where this document and
the code disagree, that is a defect in one of them and MUST be reconciled.

Non-normative design rationale (sizing arguments, requirement traceability,
implementation ordering) lives in `docs/resources/SPEC.md` and
`docs/requirements/`; those documents do not override this one.

---

## Table of contents

1. [Scope](#1-scope)
2. [Consensus constants](#2-consensus-constants)
3. [Epoch geometry and height arithmetic](#3-epoch-geometry-and-height-arithmetic)
4. [Phase state machine](#4-phase-state-machine)
5. [EpochManager lifecycle](#5-epochmanager-lifecycle)
6. [Checkpoint competition](#6-checkpoint-competition)
7. [Reward distribution](#7-reward-distribution)
8. [Verification: roots, proofs, and signing material](#8-verification-roots-proofs-and-signing-material)
9. [Data types and serialization](#9-data-types-and-serialization)
10. [Error taxonomy](#10-error-taxonomy)
11. [Configuration](#11-configuration)
12. [Security properties](#12-security-properties)
13. [Conformance](#13-conformance)

---

## 1. Scope

`dig-epoch` defines the **epoch layer** of the DIG L2 blockchain:

- **Epoch geometry** — the fixed mapping between L2 block heights, epoch
  numbers, and Chia L1 height windows (§2.1, §3).
- **Phase state machine** — the four-phase epoch lifecycle
  (`BlockProduction → Checkpoint → Finalization → Complete`) driven purely by
  L1 block-height progress (§4).
- **EpochManager** — the stateful, thread-safe orchestrator that tracks the
  current epoch, gates operations by phase, archives completed epochs, and
  runs the checkpoint competition (§5).
- **Checkpoint competition** — score-based selection of one winning
  `CheckpointSubmission` per epoch (§6).
- **Reward economics** — the emission schedule (halvings + tail), the
  epoch-first-block bonus, the 50/50 fee split, and the 5-role reward split
  (§7).
- **Verification primitives** — epoch block Merkle roots, inclusion proofs,
  the withdrawals Merkle-set root, the checkpoint signing digest, and BLS
  aggregate submission construction (§8).

The crate is **pure and self-contained**: it performs no I/O, no networking,
no persistence, and reads no wall-clock time. All phase and epoch decisions
are deterministic functions of heights and constants, so every conforming
node computes identical results from the same chain view.

The crate MUST NOT redefine primitive ecosystem types. `Checkpoint`,
`CheckpointSubmission`, and `SignerBitmap` come from `dig-block`; `Bytes32`
comes from `chia-protocol`; BLS types come from `chia-bls`. `dig_epoch`
re-exports `Bytes32`, `Checkpoint`, and `CheckpointSubmission` for a
single-crate import surface.

**Public API shape:** every implemented symbol is re-exported flat at
`dig_epoch::<name>` (constants, arithmetic functions, phase function, reward
functions, `EpochManager`, data types, verification functions, error enums).
DFSP epoch-boundary *processing* functions (`src/dfsp.rs`) are not yet
implemented and are not part of the exported surface; only the
`DfspCloseSnapshot` data type is (§9).

---

## 2. Consensus constants

All constants in this section are **consensus-critical**. They are `pub const`
(compile-time) and MUST NOT be runtime-configurable. Changing any value is a
consensus-breaking protocol event. Callers MUST reference these symbols by
name; hard-coding their values at call sites is non-conforming.

### 2.1 Epoch geometry

| Constant | Type | Value | Meaning |
|---|---|---|---|
| `BLOCKS_PER_EPOCH` | `u64` | `32` | L2 blocks per epoch |
| `EPOCH_L1_BLOCKS` | `u32` | `32` | Chia L1 blocks per epoch phase window |
| `GENESIS_HEIGHT` | `u64` | `1` | First L2 block height (height 0 = "no blocks") |

L2 heights are `u64`; L1 heights are `u32` (matching Chia's 32-bit L1
heights). Epoch `e` contains L2 heights
`[e·BLOCKS_PER_EPOCH + 1, (e+1)·BLOCKS_PER_EPOCH]` — the last height in the
range is the epoch's checkpoint block.

### 2.2 Phase boundary thresholds

Phase progress percentage is compared against these (see §4):

| Constant | Type | Value |
|---|---|---|
| `PHASE_BLOCK_PRODUCTION_END_PCT` | `u32` | `50` |
| `PHASE_CHECKPOINT_END_PCT` | `u32` | `75` |
| `PHASE_FINALIZATION_END_PCT` | `u32` | `100` |

### 2.3 Reward economics

All amounts are mojos; `1 L2 token = MOJOS_PER_L2 = 10^12` mojos.

| Constant | Type | Value | Meaning |
|---|---|---|---|
| `MOJOS_PER_L2` | `u64` | `1_000_000_000_000` | mojos per L2 token |
| `L2_BLOCK_TIME_MS` | `u64` | `3_000` | target L2 block time |
| `L2_BLOCKS_PER_10_MIN` | `u64` | `200` | 600 000 ms / 3 000 ms |
| `INITIAL_EMISSION_PER_10_MIN` | `u64` | `64 · MOJOS_PER_L2` | initial emission rate |
| `TAIL_EMISSION_PER_10_MIN` | `u64` | `4 · MOJOS_PER_L2` | tail emission rate |
| `INITIAL_BLOCK_REWARD` | `u64` | `320_000_000_000` | 0.32 L2 per block, pre-halving |
| `TAIL_BLOCK_REWARD` | `u64` | `20_000_000_000` | 0.02 L2 per block at tail |
| `HALVING_INTERVAL_BLOCKS` | `u64` | `94_608_000` | ≈ 3 years at 3 s blocks |
| `HALVINGS_BEFORE_TAIL` | `u64` | `4` | halvings before permanent tail |
| `INITIAL_EPOCH_REWARD` | `u64` | `32_000_000_000_000` | declared epoch reward |
| `HALVING_INTERVAL_EPOCHS` | `u64` | `315_576` | declared epoch-halving interval |
| `MINIMUM_EPOCH_REWARD` | `u64` | `2_000_000_000_000` | tail floor per epoch |
| `EPOCH_FIRST_BLOCK_BONUS` | `u64` | `100_000_000_000` | bonus for the first block after an epoch checkpoint |

Derivation invariants (checked by tests, MUST hold):
`INITIAL_BLOCK_REWARD = INITIAL_EMISSION_PER_10_MIN / L2_BLOCKS_PER_10_MIN`
and `TAIL_BLOCK_REWARD = TAIL_EMISSION_PER_10_MIN / L2_BLOCKS_PER_10_MIN`.

### 2.4 Fee and reward split percentages

| Constant | Type | Value |
|---|---|---|
| `FEE_PROPOSER_SHARE_PCT` | `u64` | `50` |
| `FEE_BURN_SHARE_PCT` | `u64` | `50` |
| `PROPOSER_REWARD_SHARE` | `u64` | `10` |
| `ATTESTER_REWARD_SHARE` | `u64` | `80` |
| `EF_SPAWNER_REWARD_SHARE` | `u64` | `3` |
| `SCORE_SUBMITTER_REWARD_SHARE` | `u64` | `4` |
| `FINALIZER_REWARD_SHARE` | `u64` | `3` |

Invariants (MUST hold): `FEE_PROPOSER_SHARE_PCT + FEE_BURN_SHARE_PCT == 100`
and the five role shares sum to exactly `100`.

### 2.5 DFSP, consensus, slashing, and withdrawal

| Constant | Type | Value | Meaning |
|---|---|---|---|
| `DFSP_WALL_CLOCK_EPOCH_SECONDS` | `u64` | `96` | `(BLOCKS_PER_EPOCH · L2_BLOCK_TIME_MS)/1000` |
| `DFSP_GRACE_PERIOD_NETWORK_EPOCHS` | `u64` | `27_000` | ≈ 30-day CID grace window |
| `DFSP_GENESIS_ISSUANCE_SUBSIDY_MOJOS_V1` | `u128` | `0` | bootstrap subsidy per evaluated epoch |
| `DFSP_ACTIVATION_HEIGHT` | `u64` | `u64::MAX` | DFSP disabled by default |
| `DIG_DFSP_ACTIVATION_HEIGHT_ENV` | `&str` | `"DIG_DFSP_ACTIVATION_HEIGHT"` | reserved env-var name (§11) |
| `SOFT_FINALITY_THRESHOLD_PCT` | `u64` | `67` | stake % for soft finality |
| `HARD_FINALITY_THRESHOLD_PCT` | `u64` | `67` | stake % for competition win |
| `CHECKPOINT_THRESHOLD_PCT` | `u64` | `67` | stake % for a valid submission |
| `CORRELATION_WINDOW_EPOCHS` | `u32` | `36` | correlation-penalty window |
| `SLASH_LOOKBACK_EPOCHS` | `u64` | `1_000` | max offense lookback |
| `DFSP_SLASH_LOOKBACK_EPOCHS` | `u64` | `= SLASH_LOOKBACK_EPOCHS` | alias |
| `WITHDRAWAL_DELAY_EPOCHS` | `u64` | `50` | epochs before withdrawal completes |

### 2.6 Sentinel

`EMPTY_ROOT: Bytes32` MUST equal the SHA-256 of the empty string:
`e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`.
It is the root of every empty Merkle tree/set in this crate (§8) and the
initial value of all four DFSP roots in a fresh `EpochInfo`.

---

## 3. Epoch geometry and height arithmetic

All functions in `dig_epoch::arithmetic` are pure and deterministic.
Heights are L2 heights (`u64`) unless stated otherwise.

| Function | Formula / behavior |
|---|---|
| `epoch_for_block_height(h) -> u64` | `(h − 1) / BLOCKS_PER_EPOCH`. **Precondition:** `h ≥ 1`; `h = 0` is out of domain (integer underflow — callers MUST NOT pass 0). |
| `first_height_in_epoch(e) -> u64` | `e · BLOCKS_PER_EPOCH + 1` |
| `epoch_checkpoint_height(e) -> u64` | `(e + 1) · BLOCKS_PER_EPOCH` |
| `is_genesis_checkpoint_block(h) -> bool` | `h == GENESIS_HEIGHT` |
| `is_epoch_checkpoint_block(h) -> bool` | `h % BLOCKS_PER_EPOCH == 0` |
| `is_checkpoint_class_block(h) -> bool` | genesis OR epoch checkpoint |
| `is_first_block_after_epoch_checkpoint(h) -> bool` | `h > 1 && (h − 1) % BLOCKS_PER_EPOCH == 0` (true at 33, 65, 97, …) — the heights eligible for `EPOCH_FIRST_BLOCK_BONUS` |
| `last_committed_height_in_epoch(e, tip) -> u64` | `min(tip, epoch_checkpoint_height(e))` |
| `l1_range_for_epoch(genesis_l1, e) -> (u32, u32)` | inclusive range `(genesis_l1 + e·EPOCH_L1_BLOCKS, start + EPOCH_L1_BLOCKS − 1)` — consecutive epochs tile the L1 chain contiguously |
| `ensure_checkpoint_block_empty(h, bundles, cost, fees) -> Result<(), EpochError>` | see below |

**Round-trip invariant (MUST hold):** for every epoch `e`,
`epoch_for_block_height(first_height_in_epoch(e)) == e` and
`epoch_for_block_height(epoch_checkpoint_height(e)) == e`; each epoch spans
exactly `BLOCKS_PER_EPOCH` heights.

**Empty-checkpoint-block invariant.** Checkpoint-class blocks (genesis and
every height divisible by `BLOCKS_PER_EPOCH`) MUST carry zero spend bundles,
zero cost, and zero fees. `ensure_checkpoint_block_empty` returns `Ok(())`
for non-checkpoint heights unconditionally, and
`Err(EpochError::CheckpointBlockNotEmpty(height, bundles, cost, fees))` when
any count is non-zero at a checkpoint-class height.

---

## 4. Phase state machine

### 4.1 Phases

`EpochPhase` has exactly four variants, in order:

| Ordinal (`index()`) | Variant | L1-progress range | Allowed activity |
|---|---|---|---|
| 0 | `BlockProduction` | 0–49 % | `record_block` |
| 1 | `Checkpoint` | 50–74 % | `start_checkpoint_competition`, `submit_checkpoint` |
| 2 | `Finalization` | 75–99 % | `finalize_competition`, `set_current_epoch_dfsp_close_snapshot` |
| 3 | `Complete` | ≥ 100 % | `advance_epoch` |

`next()` / `previous()` step through this order (`Complete.next() == None`,
`BlockProduction.previous() == None`). `name()` / `Display` produce the exact
strings `"BlockProduction"`, `"Checkpoint"`, `"Finalization"`, `"Complete"`.
The predicates `allows_block_production` / `allows_checkpoint_submission` /
`allows_finalization` are true only in their single corresponding phase.

### 4.2 Phase computation (normative formula)

`l1_progress_phase_for_network_epoch(genesis_l1_height, epoch, current_l1_height) -> EpochPhase`
is pure and MUST be computed as:

```
epoch_l1_start = genesis_l1_height + epoch · EPOCH_L1_BLOCKS        (u32)
pct = 0                                if current_l1_height ≤ epoch_l1_start
    = min( (current_l1_height − epoch_l1_start) · 100 / EPOCH_L1_BLOCKS, 100 )   otherwise
phase = BlockProduction  if pct < 50
        Checkpoint       if 50 ≤ pct < 75
        Finalization     if 75 ≤ pct < 100
        Complete         if pct ≥ 100
```

Integer division; the percentage is clamped at 100. The identical formula is
exposed as instance methods `EpochInfo::progress_percentage` and
`EpochInfo::calculate_phase`. Phase progression is monotone in
`current_l1_height` and MUST NOT depend on wall-clock time — every node with
the same L1 view computes the same phase.

### 4.3 Phase transitions

`EpochManager::update_phase(l1_height)` recomputes the phase and, when it
changed, records the new phase and returns
`Some(PhaseTransition { epoch, from, to, l1_height })`; otherwise `None`.
`should_advance(_)` returns true iff the current phase is `Complete`.

---

## 5. EpochManager lifecycle

`EpochManager` is the single stateful orchestrator. It owns:
`network_id: Bytes32`, `genesis_l1_height: u32`, the current `EpochInfo`, the
current `CheckpointCompetition`, an append-only `Vec<EpochSummary>` archive,
and a `HashMap<u64, RewardDistribution>` reward archive.

### 5.1 Concurrency

All state lives behind a single `parking_lot::RwLock`. Reads MAY run
concurrently; writes are exclusive. `parking_lot` does not poison, so no
API surfaces lock-poisoning errors. Every public method acquires the lock
internally; `EpochManager` is `Send + Sync` by construction.

### 5.2 Construction

`EpochManager::new(network_id, genesis_l1_height, initial_state_root)`
creates the manager at **epoch 0** with:

- `EpochInfo::new(0, genesis_l1_height, GENESIS_HEIGHT, initial_state_root)`
  — phase `BlockProduction`, zeroed counters, `checkpoint = None`, all DFSP
  roots = `EMPTY_ROOT`, DFSP numerics = 0,
  `end_l1_height = start_l1_height + EPOCH_L1_BLOCKS`;
- a fresh `CheckpointCompetition::new(0)` in `Pending` status;
- empty summary and reward archives.

`network_id` and `genesis_l1_height` are immutable for the manager's
lifetime.

### 5.3 Phase-gated operations

Each mutating operation below MUST be rejected with
`EpochError::PhaseMismatch { expected, got }` when invoked outside its
required phase. The check happens **before any state mutation**.

| Operation | Required phase | Effect |
|---|---|---|
| `record_block(fees, tx_count)` | `BlockProduction` | increments `blocks_produced` by 1, adds `fees` to `total_fees`, adds `tx_count` to `total_transactions` |
| `start_checkpoint_competition()` | `Checkpoint` | competition `Pending → Collecting` (§6) |
| `submit_checkpoint(submission)` | `Checkpoint` | scores + records the submission (§6) |
| `finalize_competition(epoch, l1_height)` | `Finalization` | selects/fails the winner (§6); also errors `EpochMismatch` if `epoch` ≠ the competition's epoch |
| `set_current_epoch_dfsp_close_snapshot(snap)` | `Finalization` | copies all 7 `DfspCloseSnapshot` fields onto the current `EpochInfo` |
| `advance_epoch(l1_height, state_root)` | `Complete` | see §5.4 |

`set_current_epoch_chain_totals(blocks, fees, txns)` is a resync/correction
escape hatch: it **overwrites** (does not increment) the three production
counters and is deliberately NOT phase-gated.

### 5.4 Epoch advancement

`advance_epoch(_l1_height, state_root) -> Result<u64, EpochError>`:

Preconditions (checked in order, before any mutation):
1. current phase is `Complete`, else `Err(EpochNotComplete(epoch))`;
2. current competition status is `Finalized`, else
   `Err(NoFinalizedCheckpoint(epoch))`.

On success it atomically:
- archives the current epoch as `EpochSummary::from(EpochInfo)` (appended to
  the ordered, append-only summary list);
- installs `EpochInfo::new(e+1, genesis_l1 + (e+1)·EPOCH_L1_BLOCKS,
  old.start_l2_height + BLOCKS_PER_EPOCH, state_root)` — i.e. the next epoch
  starts in `BlockProduction` with fresh counters and the caller-supplied
  state root;
- replaces the competition with `CheckpointCompetition::new(e+1)` (`Pending`);
- returns the new epoch number `e+1`.

The `l1_height` argument is accepted for interface stability but not used;
the next epoch's L1 window is derived from the geometry, not the argument.

### 5.5 Queries

- `current_epoch() -> u64`, `current_phase() -> EpochPhase`,
  `current_epoch_info()` / `get_epoch_info() -> EpochInfo` (clone),
  `genesis_l1_height() -> u32`, `network_id() -> Bytes32`.
- `epoch_for_l1_height(l1) -> u64`: `0` when `l1 ≤ genesis_l1_height`, else
  `(l1 − genesis_l1_height) / EPOCH_L1_BLOCKS`.
- `l1_range_for_epoch(e) -> (u32, u32)`: delegates to the free function with
  the manager's genesis L1 height.
- `get_epoch_summary(e) -> Option<EpochSummary>`: from the archive; `None`
  for the current or unknown epochs.
- `recent_summaries(n) -> Vec<EpochSummary>`: last `n` archived summaries in
  epoch order (fewer if the archive is shorter).
- `total_stats() -> EpochStats`: aggregates blocks/transactions/fees across
  all archived summaries **plus the current epoch**; `total_epochs` counts
  archived + 1 (current); `finalized_epochs` counts summaries with
  `finalized == true` plus the current epoch if it already has a checkpoint.
- `get_competition(e) -> Option<CheckpointCompetition>`: clone of the current
  competition iff `e` matches it; `None` for past/future epochs (only the
  current epoch's competition is tracked).
- `store_rewards(distribution)` / `get_rewards(e) -> Option<RewardDistribution>`:
  archive keyed by the distribution's `epoch` field (later stores overwrite).

Methods prefixed `__` (`__set_competition_for_test`,
`__force_phase_for_test`) and the `test_helpers` module are test/bootstrap
scaffolding, `#[doc(hidden)]`, and NOT part of the stable contract; the
synthetic keys/signatures produced by `test_helpers` do not verify
cryptographically and MUST NOT be used in production.

---

## 6. Checkpoint competition

One `CheckpointCompetition` exists per epoch. It collects
`dig_block::CheckpointSubmission`s and selects a single winner by score.

### 6.1 Status state machine

`CompetitionStatus` has five states; the only legal transitions are:

```
Pending ──start()──▶ Collecting ──submit() (score > 0)──▶ WinnerSelected {winner_hash, winner_score}
                          │                                      │        ▲ (higher-score submit updates in place)
                          │                                      ├─finalize(l1)─▶ Finalized {winner_hash, l1_height}   (terminal)
                          └────────────fail()────────────────────┴─fail()──────▶ Failed                                (terminal)
```

- `start()`: `Pending → Collecting`; any other starting state returns
  `Err(AlreadyFinalized)`.
- `finalize(l1_height)`: only from `WinnerSelected`; returns the winning
  checkpoint hash. From `Finalized` → `Err(AlreadyFinalized)`; from
  `Pending`/`Collecting`/`Failed` → `Err(NotStarted)`.
- `fail()`: legal from `Collecting` or `WinnerSelected`; from
  `Finalized`/`Failed` → `Err(AlreadyFinalized)`; from `Pending` →
  `Err(NotStarted)`.
- `is_finalized()` is true only in `Finalized`.

### 6.2 Submission rules

`submit(submission) -> Result<bool, CheckpointCompetitionError>` MUST:

1. Reject when status is `Pending` (`NotStarted`) or
   `Finalized`/`Failed` (`AlreadyFinalized`).
2. Reject when `submission.checkpoint.epoch != competition.epoch`
   (`EpochMismatch { expected, got }`).
3. **Record every accepted-for-consideration submission** in `submissions`
   (append-only, for auditability) — including ones that then fail the score
   test.
4. Leader test: in `Collecting` (no leader yet) the submission leads iff
   `score > 0`; in `WinnerSelected` it leads iff
   `score > winner_score` (strict — ties lose). A leading submission sets
   `status = WinnerSelected { winner_hash: checkpoint.hash(), winner_score: score }`,
   sets `current_winner` to its index, and returns `Ok(true)`. A non-leading
   submission returns `Err(ScoreNotHigher { current, submitted })` (with
   `current = 0` when there is no leader yet).

`winner()` returns the leading `CheckpointSubmission`, if any.

**Score.** The submission score is computed by the submitter as
`Checkpoint::compute_score(stake_percentage) = stake_percentage · block_count`
(defined in `dig-block`; `stake_percentage` is an integer percent). The
competition itself treats `score` as an opaque `u64` and compares only by
magnitude. A zero-score submission can never become leader.

### 6.3 Manager wiring

`EpochManager::finalize_competition(epoch, l1_height)` (phase-gated to
`Finalization`, epoch-checked against the competition):

- **No leader:** calls `fail()` on the competition and returns `Ok(None)`.
  (If the competition was never started — still `Pending` — `fail()` errors
  `NotStarted`, which propagates as `EpochError::Competition`.)
- **Leader exists:** calls `finalize(l1_height)`, copies the winning
  checkpoint onto the current `EpochInfo` (`set_checkpoint`, making
  `is_finalized()` true), and returns `Ok(Some(checkpoint))`.

---

## 7. Reward distribution

All reward functions are pure; amounts are mojos.

### 7.1 Block reward (emission schedule)

```
block_reward_at_height(h) =
    let k = (h − 1) / HALVING_INTERVAL_BLOCKS
    if k ≥ HALVINGS_BEFORE_TAIL: TAIL_BLOCK_REWARD
    else:                        INITIAL_BLOCK_REWARD >> k
```

Precondition `h ≥ 1`. The schedule is therefore 0.32 L2 → 0.16 → 0.08 → 0.04
across four ~3-year eras, then a permanent 0.02 L2 tail.

### 7.2 Epoch-first-block bonus

```
total_block_reward(h, is_first_of_epoch) =
    block_reward_at_height(h) + (EPOCH_FIRST_BLOCK_BONUS if is_first_of_epoch else 0)
```

Callers MUST derive `is_first_of_epoch` from
`is_first_block_after_epoch_checkpoint(h)` (§3): heights 33, 65, 97, …

### 7.3 Fee split

```
proposer_fee_share(F)  = F · FEE_PROPOSER_SHARE_PCT / 100      (integer division)
burned_fee_remainder(F) = F − proposer_fee_share(F)
```

The two MUST sum exactly to `F`; the burn side absorbs the integer-division
remainder (e.g. an odd fee total burns one extra mojo).

### 7.4 Five-role epoch reward split

`compute_reward_distribution(epoch, total_reward, total_fees) -> RewardDistribution`
MUST compute:

```
proposer_reward        = total_reward · 10 / 100
ef_spawner_reward      = total_reward ·  3 / 100
score_submitter_reward = total_reward ·  4 / 100
finalizer_reward       = total_reward ·  3 / 100
attester_reward        = total_reward − (sum of the other four)
```

The attester share (nominally 80 %) **absorbs all integer rounding**, so the
five shares MUST always sum to exactly `total_reward`. The returned struct
also carries `proposer_fee_share(total_fees)` and
`burned_fee_remainder(total_fees)` per §7.3.

### 7.5 Tail floor

`epoch_reward_with_floor(r) = max(r, MINIMUM_EPOCH_REWARD)` — the per-epoch
reward never falls below the tail floor.

---

## 8. Verification: roots, proofs, and signing material

All hashing delegates to the Chia ecosystem crates; this crate MUST NOT
implement its own Merkle/hash primitives beyond composing them.

### 8.1 Epoch block root (ordered)

`compute_epoch_block_root(block_hashes) -> Bytes32`:
- empty slice → `EMPTY_ROOT`;
- otherwise the root of `chia_sdk_types::MerkleTree::new(block_hashes)` —
  the tagged binary Merkle tree with
  leaf = `SHA-256(0x01 ‖ leaf_bytes)` and
  node = `SHA-256(0x02 ‖ left ‖ right)`. The root is **order-dependent**:
  block hashes MUST be supplied in ascending height order within the epoch.

### 8.2 Inclusion proofs

- `epoch_block_inclusion_proof(block_hashes, index) -> Option<MerkleProof>`:
  `None` when `index ≥ len` (including the empty slice); otherwise the
  `chia-sdk-types` proof for the leaf at `index`.
- `verify_block_inclusion_proof(leaf, proof, root) -> bool` recomputes the
  root with the same tagged hashing: start from `SHA-256(0x01 ‖ leaf)`; at
  level `i`, if bit `i` of `proof.path` is `1` the current node is the
  **right** child (`SHA-256(0x02 ‖ sibling ‖ current)`), else the left
  (`SHA-256(0x02 ‖ current ‖ sibling)`); accept iff the final digest equals
  `root`.

### 8.3 Withdrawals root (order-independent)

`compute_epoch_withdrawals_root(withdrawal_hashes) -> Bytes32`:
- empty slice → `EMPTY_ROOT`;
- otherwise `chia_consensus::merkle_set::compute_merkle_set_root` over the
  32-byte hashes — a **Merkle set** root, invariant under input ordering.
  This is deliberately a different construction from §8.1 and the two roots
  are not interchangeable.

### 8.4 Checkpoint signing digest (byte-level contract)

`EpochCheckpointData` binds a checkpoint to its network:
`{ network_id, epoch, block_root, state_root, withdrawals_root, checkpoint_hash }`.

`signing_digest()` MUST be the SHA-256 over exactly this concatenation, in
this order:

```
SHA-256( network_id (32 bytes)
       ‖ epoch      (8 bytes, little-endian u64)
       ‖ block_root (32)
       ‖ state_root (32)
       ‖ withdrawals_root (32)
       ‖ checkpoint_hash  (32) )
```

Including `network_id` in the preimage prevents cross-network signature
replay (§12). `checkpoint_hash` is `dig_block::Checkpoint::hash()` — the
SHA-256 of dig-block's fixed 160-byte checkpoint preimage; conforming
implementations MUST use dig-block's definition byte-for-byte.

`epoch_checkpoint_sign_material_from_l2_blocks(network_id, epoch,
block_hashes, state_root, withdrawal_hashes, prev_checkpoint, total_fees,
tx_count, stake_percentage) -> EpochCheckpointSignMaterial` builds the
validator-facing signing bundle:
- `block_root` per §8.1, `withdrawals_root` per §8.3;
- `checkpoint_hash` from a `dig_block::Checkpoint` populated with
  `{epoch, state_root, block_root, block_count = block_hashes.len(),
  tx_count, total_fees, prev_checkpoint, withdrawals_root,
  withdrawal_count = withdrawal_hashes.len()}`;
- `score = Checkpoint::compute_score(stake_percentage)`
  (= `stake_percentage · block_count`);
- `signing_digest` per the formula above. Validators sign `signing_digest`
  with BLS.

### 8.5 Aggregate submission construction

`stored_checkpoint_from_epoch_sign_material_with_aggregate_v1(material,
validator_set, per_validator, submitter) -> Result<CheckpointSubmission, EpochError>`:

- `per_validator` MUST be non-empty
  (`Err(DfspBoundary("aggregate signature requires at least one signer"))`
  otherwise);
- the aggregate signature is `chia_bls::aggregate` over the signer
  signatures; the aggregate public key is the sum of the signer public keys;
- the `SignerBitmap` has one bit per entry of `validator_set` (in slice
  order); bit `i` is set iff `validator_set[i]`'s validator index appears in
  `per_validator`;
- the embedded `Checkpoint` carries `epoch`, `state_root`, `block_root`, and
  `withdrawals_root` from the sign material (the fields required for
  hash-equivalence with what was signed);
- the submission's `score` is `material.score` and its submitter index is
  `submitter`.

This function does not verify the signatures; verification is the consumer's
responsibility (`chia_bls::aggregate_verify` against `signing_digest`).

---

## 9. Data types and serialization

### 9.1 Serialization convention

Every persisted/wire type provides `to_bytes()` / `from_bytes()` using
**bincode v1** with serde-derived schemas:

- `to_bytes(&self) -> Vec<u8>` is infallible for well-formed values (panics
  only on programmer error / schema drift);
- `from_bytes(&[u8]) -> Result<Self, EpochError>` maps every decode failure
  to `EpochError::InvalidData(msg)` — it MUST NOT panic on malformed input.

Types with this contract: `EpochInfo`, `EpochSummary`,
`CheckpointCompetition`, `RewardDistribution`, `DfspCloseSnapshot`,
`EpochCheckpointData` (and `EpochBlockLink` via serde derives). The bincode
encoding of these structs is a **storage/wire contract**: field order and
types MUST NOT change incompatibly. `EpochPhase` and `CompetitionStatus`
derive `Serialize`/`Deserialize` and are embedded in those encodings.

### 9.2 `EpochInfo` (mutable current-epoch state)

17 fields: identity (`epoch: u64`, `start_l1_height: u32`,
`end_l1_height: u32`, `start_l2_height: u64`), mutable counters
(`blocks_produced: u32`, `phase: EpochPhase`, `total_fees: u64`,
`total_transactions: u64`), state (`checkpoint: Option<Checkpoint>`,
`start_state_root: Bytes32`), and the DFSP close snapshot
(`collateral_registry_root`, `cid_state_root`, `node_registry_root`,
`namespace_epoch_root: Bytes32`; `dfsp_issuance_total: u64`;
`active_cid_count`, `active_node_count: u32`).

Invariants: `end_l1_height = start_l1_height + EPOCH_L1_BLOCKS`;
`is_finalized() ⇔ checkpoint.is_some()`; `target_blocks()` returns
`BLOCKS_PER_EPOCH`; the `can_*`/`is_complete` predicates test the single
corresponding phase.

### 9.3 `EpochSummary` (immutable archive)

Produced only via `From<EpochInfo>` at epoch close: carries `epoch`,
`blocks`, `transactions`, `fees`, `finalized = checkpoint.is_some()`,
`checkpoint_hash = checkpoint.map(hash)`, and the seven DFSP close fields
verbatim.

### 9.4 `DfspCloseSnapshot`

The 7-field `Copy` snapshot applied during `Finalization` (§5.3): four
`Bytes32` roots (collateral registry SMT, CID lifecycle state, node registry
SMT, cumulative namespace), `dfsp_issuance_total: u64`, and the two `u32`
active counts. DFSP epoch-boundary *processing* (burn policy, digests,
rollup, activation control) is not yet implemented in this crate; the type
exists so the manager can archive DFSP state supplied by the caller.

### 9.5 Events and stats (in-process only)

`EpochEvent` (`EpochStarted { epoch, l1_height }`,
`PhaseChanged { epoch, from, to, l1_height }`,
`EpochFinalized { epoch, checkpoint }`, `EpochFailed { epoch }`),
`PhaseTransition { epoch, from, to, l1_height }`, and `EpochStats` (5 `u64`
counters, `Default` = all zero) are telemetry/driver types without a
serialization contract.

---

## 10. Error taxonomy

Both enums derive `Debug + Clone + thiserror::Error`; `Display` strings are
part of the contract (tested verbatim). No variant exposes lock internals or
wraps a panic path.

### 10.1 `EpochError`

| Variant | Raised when |
|---|---|
| `EpochNotComplete(u64)` | `advance_epoch` before phase `Complete` |
| `NoFinalizedCheckpoint(u64)` | `advance_epoch` without a `Finalized` competition |
| `CheckpointBlockNotEmpty(h, bundles, cost, fees)` | §3 empty-checkpoint invariant violated |
| `PhaseMismatch { expected, got }` | any phase-gated operation in the wrong phase |
| `EpochMismatch { expected, got }` | submission/query references the wrong epoch |
| `InvalidHeight(u64)` | L2 height below genesis |
| `DfspNotActive(u64)` | DFSP operation before activation height |
| `DfspBoundary(String)` | DFSP epoch-boundary processing error (also used by §8.5's empty-signer rejection) |
| `Competition(CheckpointCompetitionError)` | delegated via `#[from]` |
| `InvalidData(String)` | deserialization failure (§9.1) |

### 10.2 `CheckpointCompetitionError`

| Variant | Raised when |
|---|---|
| `InvalidData(String)` | checkpoint data failed validation |
| `NotFound(u64)` | no competition for the epoch |
| `ScoreNotHigher { current, submitted }` | submission does not beat the leader (§6.2) |
| `EpochMismatch { expected, got }` | submission epoch ≠ competition epoch |
| `AlreadyFinalized` | operation on a terminal (`Finalized`/`Failed`) or already-started competition |
| `NotStarted` | operation requiring a started competition while `Pending` (or `finalize` from a non-winner state) |

`CheckpointCompetitionError` converts into `EpochError::Competition` via
`From`, so manager methods surface a single error type.

---

## 11. Configuration

The crate has **no runtime configuration**. All protocol parameters are the
compile-time constants of §2. `EpochManager::new`'s three arguments
(`network_id`, `genesis_l1_height`, `initial_state_root`) are the only
deployment inputs, and they are fixed at construction.

`DIG_DFSP_ACTIVATION_HEIGHT_ENV` (`"DIG_DFSP_ACTIVATION_HEIGHT"`) is the
**reserved** environment-variable name a hosting binary MAY use to override
`DFSP_ACTIVATION_HEIGHT` when DFSP processing lands; this crate itself does
not read the environment. DFSP is disabled by default
(`DFSP_ACTIVATION_HEIGHT = u64::MAX`).

---

## 12. Security properties

- **Determinism / consensus safety.** Every phase, epoch, reward, root, and
  digest is a pure function of chain-observable inputs (heights, hashes,
  constants). No wall-clock, randomness, or I/O enters any consensus
  computation.
- **Cross-network replay resistance.** The checkpoint signing digest (§8.4)
  binds `network_id` into the signed preimage, so a checkpoint signature for
  one network can never validate on another.
- **Domain-separated hashing.** Merkle leaves and internal nodes use distinct
  tags (`0x01` / `0x02`), preventing leaf/node confusion attacks on inclusion
  proofs. The ordered block root (§8.1) and the withdrawals set root (§8.3)
  are distinct constructions and MUST NOT be conflated.
- **Lifecycle integrity.** Phase gating (§5.3), the competition state machine
  (§6.1), and the advance preconditions (§5.4) are checked before mutation,
  so illegal transitions leave state untouched. Checkpoint-class blocks are
  provably empty (§3).
- **Auditability.** Every competition submission is retained, including
  losing ones; the summary archive is append-only.
- **BLS.** Aggregation uses `chia-bls` exclusively. Signature *verification*
  is out of scope here and MUST be performed by consumers before trusting a
  `CheckpointSubmission`. Threshold constants (67 %) are provided for those
  consumers; this crate does not itself enforce stake thresholds on
  submissions.

---

## 13. Conformance

### 13.1 Cross-repo byte-for-byte requirements

| Contract | Must match | Defined in |
|---|---|---|
| `Checkpoint` struct, its 160-byte hash preimage and `hash()` | exactly | `dig-block` |
| `Checkpoint::compute_score(stake_pct) = stake_pct · block_count` | exactly | `dig-block` |
| `CheckpointSubmission`, `SignerBitmap` semantics | exactly | `dig-block` |
| Tagged Merkle tree (leaf `0x01`, node `0x02`) for block roots/proofs | exactly | `chia-sdk-types` |
| Merkle **set** root for withdrawals | exactly | `chia-consensus` |
| BLS aggregation | exactly | `chia-bls` |
| `Bytes32`, SHA-256 | exactly | `chia-protocol`, `chia-sha2` |
| Signing-digest preimage layout (§8.4, epoch as LE u64) | exactly | this document |
| bincode v1 encodings of §9.1 types | stable | this crate |

Consumers: `dig-blockstore`, `dig-mempool`, `dig-slashing`,
`chia-l2-consensus`, and the DIG node compose this crate for epoch/phase
decisions; they MUST use these exports rather than reimplementing the
formulas. The DIG L2 protocol overview on docs.dig.net (Protocol → L2
consensus layers) describes the surrounding architecture.

### 13.2 Toolchain gates

CI (`ci.yml`, and `publish.yml` before release) MUST pass:
`cargo fmt --check`, `cargo clippy --all-targets --all-features -D warnings`,
`cargo test --all-features -- --test-threads=1`, and
`cargo llvm-cov --fail-under-lines 80` (line coverage ≥ 80 %). Integration
tests are wired one-per-requirement under `tests/` via explicit `[[test]]`
targets (`autotests = false`).

### 13.3 Conformance summary

| # | Requirement | Level |
|---|---|---|
| C1 | §2 constant values and types are exact; never runtime-configurable | MUST |
| C2 | Height/epoch arithmetic follows §3 formulas; heights start at 1 | MUST |
| C3 | Checkpoint-class blocks are empty (§3) | MUST |
| C4 | Phase derives solely from L1 progress per §4.2 (integer math, clamped) | MUST |
| C5 | Phase-gated operations reject with `PhaseMismatch` before mutating (§5.3) | MUST |
| C6 | `advance_epoch` requires `Complete` phase + `Finalized` competition (§5.4) | MUST |
| C7 | Competition transitions only per §6.1; strict-greater score wins; zero score never leads | MUST |
| C8 | All submissions retained; summaries append-only | MUST |
| C9 | Reward formulas per §7; five shares sum exactly to total (attester absorbs rounding); fee split sums exactly to fees | MUST |
| C10 | Block root ordered/tagged (§8.1); withdrawals root order-independent set (§8.3); `EMPTY_ROOT` for empty inputs | MUST |
| C11 | Signing digest byte layout per §8.4 including `network_id` binding | MUST |
| C12 | `to_bytes`/`from_bytes` = bincode v1; decode errors → `InvalidData`, never panic (§9.1) | MUST |
| C13 | Error `Display` strings per §10 | MUST |
| C14 | Consumers verify BLS aggregates before trusting submissions (§12) | MUST |
| C15 | Reuse `dig-block`/Chia primitives; do not redefine (§13.1) | MUST |
| C16 | `__`-prefixed and `test_helpers` items excluded from production use | MUST |
