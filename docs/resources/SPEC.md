# dig-epoch Specification

**Version:** 0.1.0
**Status:** Draft
**Date:** 2026-04-14

## 1. Overview

`dig-epoch` is a self-contained Rust crate that owns three concerns for the DIG Network L2 blockchain: **defining the epoch format**, **managing epoch lifecycle**, and **computing epoch economics**. It covers the full epoch lifecycle from creation through phase progression, checkpoint competition, DFSP epoch-boundary processing, reward distribution, and finalization. The crate provides everything needed to determine what epoch a block belongs to, what phase an epoch is in, how checkpoint competitions resolve, how rewards are distributed, and how DFSP state transitions at epoch boundaries — all without accessing external storage or network.

The crate **does** own:
- **Epoch format** — Type definitions for `EpochInfo`, `EpochSummary`, `EpochPhase`, `PhaseTransition`, `DfspCloseSnapshot`, `EpochEvent`, `EpochStats`, `RewardDistribution`, and all supporting types.
- **Epoch management** — `EpochManager` that tracks the current epoch, manages phase transitions driven by L1 progress, records block production, orchestrates checkpoint competitions, applies DFSP close snapshots, distributes rewards, and advances to the next epoch. The manager enforces lifecycle invariants so transitions are valid by construction.
- **Epoch phase state machine** — Four-phase progression (`BlockProduction` → `Checkpoint` → `Finalization` → `Complete`) driven by L1 block height as a percentage of the epoch's L1 window. Phase calculation is deterministic and stateless given the genesis L1 height, epoch number, and current L1 height.
- **Height–epoch arithmetic** — Pure functions mapping L2 block heights to epoch numbers, computing epoch checkpoint heights, detecting checkpoint-class blocks, and enforcing the empty-checkpoint-block invariant. All functions are stateless.
- **Checkpoint competition** — `CheckpointCompetition` managing the lifecycle of competing checkpoint submissions within an epoch (`Pending` → `Collecting` → `WinnerSelected` → `Finalized` or `Failed`). Score-based winner selection where `score = stake_percentage × block_count`.
- **Epoch verification** — `compute_epoch_block_root()` for Merkle root computation over an epoch's blocks, and `EpochCheckpointData` for checkpoint verification material.
- **Reward economics** — Block reward computation with halving schedule (4 halvings to tail emissions), epoch-opening bonus, fee distribution (proposer/burn split), and five-way epoch reward distribution (proposer, attester, EF spawner, score submitter, finalizer).
- **DFSP epoch-boundary processing** — Deterministic seven-stage execution pipeline (`EpochBurn` → `CollateralAndCid` → `NodeRegistry` → `Proofs` → `Namespace` → `FinalizeRoots`) with burn policy, issuance preview, reassignment batching, and root commitment computation.
- **Epoch constants** — `BLOCKS_PER_EPOCH`, `EPOCH_L1_BLOCKS`, phase percentage boundaries, reward parameters, DFSP epoch parameters, consensus thresholds, slashing lookback.
- **Error types** — `EpochError`, `CheckpointCompetitionError`.
- **Serialization** — `to_bytes()` / `from_bytes()` via bincode for all epoch types.

The crate does **not** own:
- **Block format** (L2BlockHeader, L2Block, AttestedBlock, Checkpoint, CheckpointSubmission) — owned by `dig-block`. However, this crate references `Checkpoint` and `CheckpointSubmission` from `dig-block` for competition management and epoch finalization.
- **Block production and validation** (BlockBuilder, validation pipeline, CLVM execution) — owned by `dig-block` and `dig-clvm`.
- **Block storage** (persisting blocks to disk, indexing by height/hash) — handled by the chain store layer.
- **Global state management** (CoinSet database, rollback, state root computation) — owned by `dig-coinstore`.
- **Transaction pool** (fee prioritization, conflict detection, CPFP) — owned by `dig-mempool`.
- **Networking** (block gossip, peer sync, checkpoint relay) — owned by `dig-gossip`.
- **Validator set management** (validator registration, stake tracking, activation/deactivation) — handled by the consensus layer. Validator stakes are injected into checkpoint scoring by the caller.
- **L1 coin interactions** (puzzle spends, singleton management, on-chain finalization transactions) — handled by the L1 driver layer.
- **Network-level constants** (genesis challenge, network ID) — owned by `dig-constants`.

**Hard boundary:** The crate operates on **epochs as a self-contained lifecycle unit**. It defines what an epoch is, how it progresses through phases, how L2 heights map to epochs, how checkpoint competitions resolve, how rewards distribute, and how DFSP epoch-boundary operations are staged. External state required for epoch operations (current L1 height, block hashes, validator stakes, DFSP registry state) is injected through function parameters and traits, not stored persistently. The crate never reads from a database, never makes network calls, and never executes CLVM.

### 1.1 Design Principles

- **Epoch-scoped lifecycle**: Every function operates on epochs and the mapping between blocks and epochs. The `EpochManager` maintains the current epoch's state and a history of completed epochs, but all state transitions are driven by explicit calls with injected external data (L1 heights, block fees, checkpoint submissions).
- **L1-driven phase progression**: Epoch phases are deterministically computed from L1 block height progress within the epoch's L1 window. Given the same genesis L1 height, epoch number, and current L1 height, `calculate_phase()` always returns the same result. No wall-clock time is used.
- **Stateless arithmetic**: Height–epoch conversion functions (`epoch_for_block_height`, `first_height_in_epoch`, `epoch_checkpoint_height`, etc.) are pure functions with no side effects. They can be called from any context without an `EpochManager` instance.
- **Score-based competition**: Checkpoint competition uses a deterministic scoring function (`stake_percentage × block_count`) with highest-score-wins semantics. The competition lifecycle is a simple state machine with well-defined transitions.
- **Deterministic DFSP staging**: DFSP epoch-boundary processing follows a fixed seven-stage execution order. Each stage's inputs and outputs are fully determined by the previous stage and the epoch's block data. The staging order ensures that burn transitions, registry deltas, proof evaluations, namespace updates, and root commitments are applied in a consistent, reproducible sequence.
- **Reward economics are protocol law**: Block rewards, halving schedule, fee splits, and epoch reward distribution ratios are defined as constants. The functions that compute them are pure — no configuration, no overrides, no runtime flags (except DFSP activation height for the DFSP subsystem).
- **Existing crates, not custom abstractions**: If an existing crate already provides it, dig-epoch uses it directly rather than reimplementing. **DIG crate:** Checkpoint and CheckpointSubmission types come from `dig-block` (not redefined). **Chia crates:** `Bytes32` from `chia-protocol` (not a custom hash type), `Signature`/`PublicKey`/`aggregate()`/`aggregate_verify()` from `chia-bls` (not a custom BLS library), `MerkleTree`/`MerkleProof` from `chia-sdk-types` (not a custom Merkle tree — epoch block root computation uses this directly), `Sha256` from `chia-sha2` (not a generic sha2 crate), `tree_hash()`/`TreeHash` from `clvm-utils` (not a custom tree hasher), and `compute_merkle_set_root()` from `chia-consensus` (not a custom Merkle set). This ensures bit-for-bit compatibility with the Chia ecosystem, reduces maintenance burden, and inherits bug fixes automatically.
- **Two audiences, one crate**: Both the stateless arithmetic functions (used by block proposers, validators, light clients) and the stateful `EpochManager` (used by full nodes) live in the same crate because they share constants, types, and the epoch model. The stateless functions have no dependency on `EpochManager`.

### 1.2 Crate Dependencies

The crate maximally reuses the Chia Rust ecosystem to avoid reimplementing production-hardened primitives. The principle is: **if a Chia crate already provides it, use it — don't rewrite it.**

| Crate | Version | Purpose |
|-------|---------|---------|
| `dig-block` | 0.1 | `Checkpoint`, `CheckpointSubmission`, `SignerBitmap`, `L2BlockHeader` types. `dig-epoch` manages competitions over these types and reads block headers for DFSP root extraction, but does not define them. |
| `dig-constants` | 0.10 | Network-level constants: `NetworkConstants`, genesis challenge, network ID. Declared and linked, but not yet consumed: `dig-constants` 0.10 sits on the chia **0.36** cohort while this crate sits on **0.26/0.30**, so its chia types are distinct types from this crate's and MUST NOT be used until both cohorts converge. |
| `chia-protocol` | 0.26 | Core protocol types: `Bytes32`. The universal 32-byte hash type used for hashes, Merkle roots, coin IDs, block hashes — everywhere in the epoch crate. |
| `chia-bls` | 0.26 | BLS12-381 cryptography: `Signature`, `PublicKey`, `SecretKey`. Functions: `sign()`, `verify()`, `aggregate()`, `aggregate_verify()`. Used for checkpoint aggregate signature verification in competition scoring, and for `EpochCheckpointData` signing digest verification. |
| `chia-consensus` | 0.26 | **Merkle set construction.** `compute_merkle_set_root()` — computes an order-independent Merkle set root identically to Chia L1. Used for DFSP namespace rollup and withdrawal root computation. `validate_merkle_proof()` — used for Merkle inclusion proof verification. |
| `chia-sdk-types` | 0.30 | **Merkle tree construction.** `MerkleTree::new(&[Bytes32])` — balanced binary Merkle tree used directly for `compute_epoch_block_root()`. `MerkleProof` — inclusion proof generation and verification for epoch block Merkle proofs. `MAINNET_CONSTANTS` / `TESTNET11_CONSTANTS` — network consensus constants for validation. |
| `chia-sdk-signer` | 0.30 | **Aggregate signature constants.** `AggSigConstants` — domain separation constants for checkpoint signing digest construction. |
| `chia-sha2` | 0.26 | SHA-256 implementation (`Sha256` hasher). Used for `EpochCheckpointData::hash()`, operations digest, commitment digest, and domain-prefixed hashing where the Chia `MerkleTree` is not applicable. Same implementation used internally by `Coin::coin_id()` and throughout the Chia ecosystem. |
| `clvm-utils` | 0.26 | `tree_hash()`, `TreeHash`. Used for puzzle hash verification in checkpoint data and DFSP commitment digest computation where CLVM tree hashing semantics apply. |
| `bincode` | — | Compact binary serialization for all epoch types (`to_bytes()` / `from_bytes()`). |
| `serde` | — | Serialization/deserialization framework. All epoch types derive `Serialize` + `Deserialize`. |
| `thiserror` | — | Error type derivation for `EpochError`, `CheckpointCompetitionError`. |
| `parking_lot` | — | `RwLock` for interior mutability in `EpochManager`. |

**Key types used from the Chia ecosystem:**

| Type | From Crate | Usage in dig-epoch |
|------|-----------|-------------------|
| `Bytes32` | chia-protocol | Hashes, coin IDs, Merkle roots, block hashes, epoch checkpoint hashes — everywhere. |
| `Signature` | chia-bls | Checkpoint aggregate signatures in `CheckpointSubmission` (from `dig-block`). Verified via `aggregate_verify()` during competition validation. |
| `PublicKey` | chia-bls | Checkpoint aggregate public key in `CheckpointSubmission` (from `dig-block`). |
| `aggregate()` | chia-bls | Combines multiple BLS signatures into one. Used when building checkpoint aggregate signatures from per-validator signatures. |
| `aggregate_verify()` | chia-bls | Verifies aggregate BLS signatures against multiple public key/message pairs. Used to validate checkpoint submissions in competition. |
| `MerkleTree` | chia-sdk-types | Balanced binary Merkle tree. `MerkleTree::new(&block_hashes).root()` is used directly for `compute_epoch_block_root()`. `MerkleTree::proof(leaf)` generates inclusion proofs for individual blocks. |
| `MerkleProof` | chia-sdk-types | Inclusion proof for a leaf in a `MerkleTree`. Used to prove that a specific block hash is part of an epoch's block root. Fields: `path: u32`, `proof: Vec<Bytes32>`. |
| `compute_merkle_set_root()` | chia-consensus | Order-independent Merkle set root. Used for withdrawal root computation (set of withdrawal requests, order doesn't matter). |
| `validate_merkle_proof()` | chia-consensus | Validates a Merkle inclusion proof against a root. Used for epoch block root proof verification by light clients. |
| `AggSigConstants` | chia-sdk-signer | Domain separation constants for aggregate signature message construction. Used in checkpoint signing digest to ensure domain separation. |
| `Sha256` | chia-sha2 | SHA-256 hasher for `EpochCheckpointData::hash()`, operations digest, commitment digest. |
| `TreeHash` | clvm-utils | CLVM tree hash type. Used for DFSP commitment digest where CLVM tree hashing semantics apply. |
| `MAINNET_CONSTANTS` | chia-sdk-types | Network consensus constants. Referenced for validation parameter defaults. |

### 1.3 Design Decisions

| # | Decision | Rationale |
|---|----------|-----------|
| 1 | Epoch phases driven by L1 height, not wall-clock time | L1 blocks are the objective, consensus-observable clock. Wall-clock time varies between nodes and is subject to drift. By anchoring phases to L1 progress, all validators agree on the current phase given the same L1 view. |
| 2 | `BLOCKS_PER_EPOCH = 32` and `EPOCH_L1_BLOCKS = 32` | 32 L2 blocks per epoch is small enough for fast finality (~96 seconds at 3s block time) yet large enough to amortize checkpoint overhead. 32 L1 blocks (~10 minutes at 18s Chia block time) provides a comfortable window for the three-phase lifecycle. |
| 3 | Height-1 is genesis, not height-0 | The genesis block at height 1 serves as the epoch-0 checkpoint. This avoids ambiguity with a "height 0" that could be confused with "no blocks". `epoch_for_block_height(h) = (h - 1) / BLOCKS_PER_EPOCH`. |
| 4 | Checkpoint blocks must be empty | The last block of each epoch (at `epoch_checkpoint_height`) carries zero SpendBundles, zero cost, and zero fees. This provides a clean state boundary for epoch summarization — the checkpoint reflects the state root after all transactions, without interleaving new transactions with the checkpoint. |
| 5 | Four-phase epoch lifecycle with asymmetric windows | BlockProduction (0–50%), Checkpoint (50–75%), Finalization (75–100%), Complete. The asymmetric split gives block production the largest window, checkpointing a moderate window, and finalization the smallest (since it is computationally lighter). |
| 6 | `EpochManager` uses interior mutability | The manager is shared across driver components (block proposer, checkpoint submitter, phase tracker). `RwLock` allows concurrent reads (current epoch, phase queries) with exclusive writes (advance epoch, record block). This matches the access pattern of a multi-threaded validator. |
| 7 | Competition state machine is independent of `EpochPhase` | `CheckpointCompetition` has its own status enum (`Pending` → `Collecting` → `WinnerSelected` → `Finalized` / `Failed`) that advances independently of the epoch phase. This separation allows competition to fail or be retried without corrupting epoch phase tracking. |
| 8 | Reward constants are compile-time, not configurable | Reward parameters (halving interval, initial emission, tail emission, distribution shares) are economic commitments that must be identical across all validators. Making them runtime-configurable would invite accidental divergence. The only runtime flag is DFSP activation height. |
| 9 | DFSP epoch-boundary uses staged execution | Seven discrete stages ensure that each domain (burn, collateral, node registry, proofs, namespace, roots) is processed in order with well-defined inputs/outputs. This prevents ordering bugs where, for example, a namespace update depends on a node registry change that hasn't been applied yet. |
| 10 | Epoch history stored as summaries, not full `EpochInfo` | Once an epoch is finalized, the full `EpochInfo` (with mutable phase tracking) is compressed to an immutable `EpochSummary`. This reduces memory usage for long-running validators and makes the distinction between "current mutable epoch" and "historical immutable epochs" explicit. |
| 11 | `DfspCloseSnapshot` applied before `advance_epoch()` | DFSP roots and issuance totals from the closing epoch's final state are captured in a snapshot and applied to the current `EpochInfo` before archival. This ensures the `EpochSummary` records accurate DFSP state as of epoch close. |
| 12 | Separate `EpochError` and `CheckpointCompetitionError` | Epoch lifecycle errors (invalid advance, phase mismatch) are distinct from competition errors (score not higher, already finalized). Callers can pattern-match on the error domain without parsing strings. |
| 13 | `CompetitionStatus` instead of reusing `dig-block::CheckpointStatus` | `dig-block` defines `CheckpointStatus` for a single checkpoint's L1 submission state. `CompetitionStatus` tracks the multi-submission competition lifecycle (Collecting, WinnerSelected, etc.). Different state machines with different semantics deserve different types. |
| 14 | `EpochBlockLink` lives in `dig-epoch`, not the L1 driver | Block continuity within an epoch is an epoch-level concern. The L1 driver consumes `EpochBlockLink` for finalization payloads, but the type itself describes the epoch's internal block chain — it belongs where epochs are defined. |
| 15 | Maximal reuse of Chia and DIG crates — no custom reimplementations | If an existing crate already provides a type, function, or algorithm, dig-epoch uses it directly rather than reimplementing. **DIG crate:** Checkpoint and CheckpointSubmission from `dig-block` (the same types the rest of the DIG ecosystem uses — not a separate epoch-specific checkpoint). **Chia crates:** Merkle tree from `chia-sdk-types::MerkleTree` (not a custom balanced tree — `compute_epoch_block_root()` calls `MerkleTree::new(&hashes).root()` directly), Merkle set root from `chia-consensus::compute_merkle_set_root()` (not a custom set root — used for withdrawal roots), BLS signature aggregation and verification from `chia-bls::aggregate()` / `aggregate_verify()` (not a custom BLS wrapper — checkpoint signatures use this directly), domain separation from `chia-sdk-signer::AggSigConstants` (not custom domain bytes), SHA-256 from `chia-sha2::Sha256` (not a generic sha2 crate), tree hashing from `clvm-utils::tree_hash()` (not a custom tree hash). This ensures bit-for-bit compatibility with the Chia ecosystem, reduces maintenance burden, and inherits upstream fixes automatically. |

## 2. Constants

### 2.1 Epoch Geometry

```rust
/// L2 blocks per epoch — each epoch spans exactly this many committed heights.
/// Epoch e contains heights [e × BLOCKS_PER_EPOCH + 1, (e + 1) × BLOCKS_PER_EPOCH].
pub const BLOCKS_PER_EPOCH: u64 = 32;

/// L1 blocks per epoch window (~10 minutes at 18s Chia block time).
/// Phase progression is computed as a percentage of this window.
pub const EPOCH_L1_BLOCKS: u32 = 32;

/// First L2 block height. The genesis block is at height 1, not 0.
pub const GENESIS_HEIGHT: u64 = 1;
```

### 2.2 Phase Boundaries

```rust
/// Block production phase ends at 50% of the L1 window.
pub const PHASE_BLOCK_PRODUCTION_END_PCT: u32 = 50;

/// Checkpoint submission phase ends at 75% of the L1 window.
pub const PHASE_CHECKPOINT_END_PCT: u32 = 75;

/// Finalization phase ends at 100% of the L1 window.
pub const PHASE_FINALIZATION_END_PCT: u32 = 100;
```

### 2.3 Reward Economics

```rust
/// 1 L2 token = 10^12 mojos.
pub const MOJOS_PER_L2: u64 = 1_000_000_000_000;

/// L2 block time in milliseconds.
pub const L2_BLOCK_TIME_MS: u64 = 3_000;

/// L2 blocks per 10-minute window.
pub const L2_BLOCKS_PER_10_MIN: u64 = 200; // 600_000 / 3_000

/// Initial emission rate: 64 L2 per 10 minutes.
pub const INITIAL_EMISSION_PER_10_MIN: u64 = 64 * MOJOS_PER_L2;

/// Tail emission rate: 4 L2 per 10 minutes.
pub const TAIL_EMISSION_PER_10_MIN: u64 = 4 * MOJOS_PER_L2;

/// Per-block reward before any halving (0.32 L2).
pub const INITIAL_BLOCK_REWARD: u64 = INITIAL_EMISSION_PER_10_MIN / L2_BLOCKS_PER_10_MIN;

/// Per-block reward at tail emission (0.02 L2).
pub const TAIL_BLOCK_REWARD: u64 = TAIL_EMISSION_PER_10_MIN / L2_BLOCKS_PER_10_MIN;

/// Halving interval: ~3 years of blocks at 3s block time.
pub const HALVING_INTERVAL_BLOCKS: u64 = 94_608_000;

/// Number of halvings before switching to tail emission.
pub const HALVINGS_BEFORE_TAIL: u64 = 4;

/// Initial epoch reward (sum of block rewards across one epoch).
pub const INITIAL_EPOCH_REWARD: u64 = 32_000_000_000_000;

/// Halving interval in epochs.
pub const HALVING_INTERVAL_EPOCHS: u64 = 315_576;

/// Minimum epoch reward (tail emission floor).
pub const MINIMUM_EPOCH_REWARD: u64 = 2_000_000_000_000;

/// Bonus reward for the first block after an epoch checkpoint.
pub const EPOCH_FIRST_BLOCK_BONUS: u64 = 100_000_000_000;

/// Proposer share of collected fees (percentage).
pub const FEE_PROPOSER_SHARE_PCT: u64 = 50;

/// Burn share of collected fees (percentage).
pub const FEE_BURN_SHARE_PCT: u64 = 50;

/// ── Epoch reward distribution shares (must sum to 100) ──

/// Proposer share of epoch reward.
pub const PROPOSER_REWARD_SHARE: u64 = 10;

/// Attester share of epoch reward.
pub const ATTESTER_REWARD_SHARE: u64 = 80;

/// EF spawner share of epoch reward.
pub const EF_SPAWNER_REWARD_SHARE: u64 = 3;

/// Score submitter share of epoch reward.
pub const SCORE_SUBMITTER_REWARD_SHARE: u64 = 4;

/// Finalizer share of epoch reward.
pub const FINALIZER_REWARD_SHARE: u64 = 3;
```

### 2.4 DFSP Epoch Parameters

```rust
/// Wall-clock seconds represented by one DFSP accounting epoch.
/// Derived: (32 blocks × 3000ms) / 1000 = 96 seconds.
pub const DFSP_WALL_CLOCK_EPOCH_SECONDS: u64 =
    (BLOCKS_PER_EPOCH * L2_BLOCK_TIME_MS) / 1000;

/// Network epochs a CID may remain in Grace state before expiring.
/// Derived from a 30-day grace window: (30 × 24 × 3600) / 96 = 27_000 epochs.
pub const DFSP_GRACE_PERIOD_NETWORK_EPOCHS: u64 =
    (30 * 24 * 3600) / DFSP_WALL_CLOCK_EPOCH_SECONDS;

/// Bootstrap genesis issuance subsidy per evaluated epoch (mojos).
pub const DFSP_GENESIS_ISSUANCE_SUBSIDY_MOJOS_V1: u128 = 0;

/// DFSP activation height (default: disabled). Overridden at runtime via
/// the DIG_DFSP_ACTIVATION_HEIGHT environment variable.
pub const DFSP_ACTIVATION_HEIGHT: u64 = u64::MAX;

/// Environment variable name for DFSP activation height override.
pub const DIG_DFSP_ACTIVATION_HEIGHT_ENV: &str = "DIG_DFSP_ACTIVATION_HEIGHT";
```

### 2.5 Consensus Thresholds

```rust
/// Stake percentage required for soft finality (attestation threshold).
pub const SOFT_FINALITY_THRESHOLD_PCT: u64 = 67;

/// Stake percentage required for a checkpoint to win the competition.
pub const HARD_FINALITY_THRESHOLD_PCT: u64 = 67;

/// Stake percentage required for a valid checkpoint submission.
pub const CHECKPOINT_THRESHOLD_PCT: u64 = 67;
```

### 2.6 Slashing & Withdrawal

```rust
/// Epochs to track for correlation penalty calculation.
pub const CORRELATION_WINDOW_EPOCHS: u32 = 36;

/// Maximum lookback for slashable offenses (in epochs).
pub const SLASH_LOOKBACK_EPOCHS: u64 = 1_000;

/// DFSP slashing evidence lookback (same as general slashing).
pub const DFSP_SLASH_LOOKBACK_EPOCHS: u64 = SLASH_LOOKBACK_EPOCHS;

/// Epochs before a withdrawal completes.
pub const WITHDRAWAL_DELAY_EPOCHS: u64 = 50;
```

## 3. Data Model

### 3.1 Primitive Types

| Type | Definition | Usage |
|------|-----------|-------|
| `Bytes32` | `[u8; 32]` (from `chia-protocol`) | Hashes, roots, coin IDs — everywhere. |
| `Signature` | BLS12-381 signature (from `chia-bls`) | Checkpoint aggregate signatures in competition. |
| `PublicKey` | BLS12-381 public key (from `chia-bls`) | Checkpoint aggregate public key in competition. |
| `Checkpoint` | Epoch summary (from `dig-block`) | Referenced in `EpochInfo`, `CheckpointCompetition`, `EpochEvent`. |
| `CheckpointSubmission` | Signed checkpoint (from `dig-block`) | Submitted to `CheckpointCompetition` for scoring. |
| `SignerBitmap` | Validator participation bitmap (from `dig-block`) | Referenced via `CheckpointSubmission`. |
| `L2BlockHeader` | Block header (from `dig-block`) | Read-only for DFSP root extraction at epoch boundary. |

### 3.2 EpochPhase

The epoch phase determines what operations are permitted during the current L1 window position.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EpochPhase {
    /// Block production (0–50% of L1 window).
    /// Validators propose and attest L2 blocks.
    BlockProduction,

    /// Checkpoint submission (50–75% of L1 window).
    /// Validators submit competing checkpoint summaries.
    Checkpoint,

    /// Finalization (75–100% of L1 window).
    /// Winning checkpoint is selected, rewards are computed.
    Finalization,

    /// Epoch is complete (>= 100% of L1 window).
    /// No further operations; epoch is ready to advance.
    Complete,
}
```

**Phase permissions:**

| Phase | `allows_block_production()` | `allows_checkpoint_submission()` | `allows_finalization()` |
|-------|---------------------------|--------------------------------|------------------------|
| `BlockProduction` | `true` | `false` | `false` |
| `Checkpoint` | `false` | `true` | `false` |
| `Finalization` | `false` | `false` | `true` |
| `Complete` | `false` | `false` | `false` |

**Phase ordering:**

| Method | Returns |
|--------|---------|
| `index()` | `BlockProduction` = 0, `Checkpoint` = 1, `Finalization` = 2, `Complete` = 3 |
| `next()` | `BlockProduction` → `Some(Checkpoint)` → `Some(Finalization)` → `Some(Complete)` → `None` |
| `previous()` | `Complete` → `Some(Finalization)` → `Some(Checkpoint)` → `Some(BlockProduction)` → `None` |
| `name()` | `"BlockProduction"`, `"Checkpoint"`, `"Finalization"`, `"Complete"` |

### 3.3 PhaseTransition

Records a phase change event for telemetry and downstream notification.

```rust
#[derive(Debug, Clone)]
pub struct PhaseTransition {
    pub epoch: u64,
    pub from: EpochPhase,
    pub to: EpochPhase,
    pub l1_height: u32,
}
```

### 3.4 EpochInfo

Mutable state for the current epoch. Tracks block production, fees, DFSP roots, and the winning checkpoint.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpochInfo {
    // ── Identity ──
    pub epoch: u64,                            // Epoch number (0-indexed)
    pub start_l1_height: u32,                  // First L1 height in this epoch's window
    pub end_l1_height: u32,                    // Last L1 height in this epoch's window
    pub start_l2_height: u64,                  // First L2 block height in this epoch

    // ── Mutable counters ──
    pub blocks_produced: u32,                  // L2 blocks recorded so far
    pub phase: EpochPhase,                     // Current phase
    pub total_fees: u64,                       // Accumulated fees (mojos)
    pub total_transactions: u64,               // Accumulated transaction count

    // ── State ──
    pub checkpoint: Option<Checkpoint>,        // Winning checkpoint (set after finalization)
    pub start_state_root: Bytes32,             // CoinSet state root at epoch start

    // ── DFSP close snapshot (DL-CKP-001) ──
    pub collateral_registry_root: Bytes32,     // Collateral registry SMT root at close
    pub cid_state_root: Bytes32,               // CID lifecycle state root at close
    pub node_registry_root: Bytes32,           // Node registry SMT root at close
    pub namespace_epoch_root: Bytes32,         // Cumulative namespace root at close
    pub dfsp_issuance_total: u64,              // Total DFSP issuance this epoch (mojos)
    pub active_cid_count: u32,                 // Active CIDs at close
    pub active_node_count: u32,                // Active storage nodes at close
}
```

**Field groups:**

| Group | Fields | Purpose |
|-------|--------|---------|
| Identity | `epoch`, `start_l1_height`, `end_l1_height`, `start_l2_height` | Locates the epoch in the chain. L1 range defines the phase window. |
| Mutable counters | `blocks_produced`, `phase`, `total_fees`, `total_transactions` | Accumulate as blocks arrive and L1 progresses. |
| State | `checkpoint`, `start_state_root` | CoinSet state root at epoch start and the finalized checkpoint (once selected). |
| DFSP close (DL-CKP-001) | `collateral_registry_root`, `cid_state_root`, `node_registry_root`, `namespace_epoch_root`, `dfsp_issuance_total`, `active_cid_count`, `active_node_count` | DFSP state snapshot applied before epoch archival. All default to `EMPTY_ROOT` / `0` until `set_current_epoch_dfsp_close_snapshot()` is called. |

**Methods:**

| Method | Signature | Description |
|--------|-----------|-------------|
| `new()` | `(epoch, start_l1_height, start_l2_height, start_state_root) -> Self` | Creates a new epoch with `BlockProduction` phase and zeroed counters. DFSP roots default to `EMPTY_ROOT`. `end_l1_height` = `start_l1_height + EPOCH_L1_BLOCKS`. |
| `calculate_phase()` | `(&self, current_l1_height: u32) -> EpochPhase` | Deterministically computes phase from L1 progress percentage. |
| `target_blocks()` | `(&self) -> u64` | Returns `BLOCKS_PER_EPOCH`. |
| `can_produce_blocks()` | `(&self) -> bool` | True when phase is `BlockProduction`. |
| `can_submit_checkpoint()` | `(&self) -> bool` | True when phase is `Checkpoint`. |
| `is_complete()` | `(&self) -> bool` | True when phase is `Complete`. |
| `is_finalized()` | `(&self) -> bool` | True when `checkpoint.is_some()`. |
| `record_block()` | `(&mut self, fees: u64, tx_count: u64)` | Increments `blocks_produced`, `total_fees`, `total_transactions`. |
| `set_checkpoint()` | `(&mut self, checkpoint: Checkpoint)` | Sets the winning checkpoint. |
| `progress_percentage()` | `(&self, current_l1_height: u32) -> u32` | Returns 0–100 based on L1 progress within the epoch window. |

### 3.5 EpochSummary

Immutable archive of a completed epoch. Created when `EpochManager::advance_epoch()` archives the current epoch.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpochSummary {
    pub epoch: u64,
    pub blocks: u32,
    pub transactions: u64,
    pub fees: u64,
    pub finalized: bool,
    pub checkpoint_hash: Option<Bytes32>,

    // ── DFSP state at close (DL-CKP-001) ──
    pub collateral_registry_root: Bytes32,
    pub cid_state_root: Bytes32,
    pub node_registry_root: Bytes32,
    pub namespace_epoch_root: Bytes32,
    pub dfsp_issuance_total: u64,
    pub active_cid_count: u32,
    pub active_node_count: u32,
}
```

### 3.6 DfspCloseSnapshot

Snapshot of DFSP state applied to the current `EpochInfo` before archival. Populated by the DFSP subsystem after epoch-boundary processing completes.

```rust
#[derive(Debug, Clone, Copy)]
pub struct DfspCloseSnapshot {
    pub collateral_registry_root: Bytes32,
    pub cid_state_root: Bytes32,
    pub node_registry_root: Bytes32,
    pub namespace_epoch_root: Bytes32,
    pub dfsp_issuance_total: u64,
    pub active_cid_count: u32,
    pub active_node_count: u32,
}
```

### 3.7 EpochEvent

Events emitted by the `EpochManager` for downstream notification (telemetry, logging, driver coordination).

```rust
#[derive(Debug, Clone)]
pub enum EpochEvent {
    EpochStarted {
        epoch: u64,
        l1_height: u32,
    },
    PhaseChanged {
        epoch: u64,
        from: EpochPhase,
        to: EpochPhase,
        l1_height: u32,
    },
    EpochFinalized {
        epoch: u64,
        checkpoint: Checkpoint,
    },
    EpochFailed {
        epoch: u64,
    },
}
```

### 3.8 EpochStats

Aggregate statistics across all epochs managed by the `EpochManager`.

```rust
#[derive(Debug, Clone, Default)]
pub struct EpochStats {
    pub total_epochs: u64,
    pub finalized_epochs: u64,
    pub total_blocks: u64,
    pub total_transactions: u64,
    pub total_fees: u64,
}
```

### 3.9 CheckpointCompetition

Manages competing checkpoint submissions for a single epoch. Tracks the competition lifecycle from opening through winner selection to finalization.

```rust
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CheckpointCompetition {
    pub epoch: u64,
    pub submissions: Vec<CheckpointSubmission>,
    pub status: CompetitionStatus,
    pub current_winner: Option<usize>, // index into submissions
}
```

**Methods:**

| Method | Signature | Description |
|--------|-----------|-------------|
| `new()` | `(epoch: u64) -> Self` | Creates competition in `Pending` status with empty submissions. |
| `start()` | `(&mut self)` | Transitions from `Pending` to `Collecting`. |
| `add_submission()` | `(&mut self, submission: CheckpointSubmission) -> bool` | Adds a submission. Returns `true` if it becomes the new leader (highest score). Updates `current_winner` index if the new submission's score exceeds the current leader. |
| `winner()` | `(&self) -> Option<&CheckpointSubmission>` | Returns the current highest-scoring submission, or `None` if no submissions. |
| `finalize()` | `(&mut self) -> Option<&CheckpointSubmission>` | Transitions to `WinnerSelected`, locks the winner. Returns the winning submission. |
| `confirm_finalization()` | `(&mut self, l1_height: u32)` | Transitions to `Finalized` with L1 confirmation height. |
| `fail()` | `(&mut self)` | Transitions to `Failed`. |
| `submission_count()` | `(&self) -> usize` | Number of submissions received. |
| `is_finalized()` | `(&self) -> bool` | True when status is `Finalized`. |

### 3.10 CompetitionStatus

Lifecycle state of a checkpoint competition. Distinct from `dig-block::CheckpointStatus` which tracks a single submission's L1 state.

```rust
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub enum CompetitionStatus {
    /// Competition created but not yet accepting submissions.
    #[default]
    Pending,

    /// Accepting checkpoint submissions; scoring in progress.
    Collecting,

    /// Best submission locked as winner; awaiting L1 confirmation.
    WinnerSelected {
        winner_hash: Bytes32,
        winner_score: u64,
    },

    /// Winner confirmed on L1; competition is permanently closed.
    Finalized {
        winner_hash: Bytes32,
        l1_height: u32,
    },

    /// Competition failed (no valid submissions, timeout, etc.).
    Failed,
}
```

**State transitions:**

```
Pending ──start()──► Collecting ──finalize()──► WinnerSelected
                                                      │
                                           confirm_finalization()
                                                      │
                                                      ▼
                                                  Finalized

        Any state ──fail()──► Failed
```

### 3.11 EpochCheckpointData

Verification material for an epoch's checkpoint, used to validate checkpoint authenticity against the network.

```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EpochCheckpointData {
    pub network_id: Bytes32,
    pub epoch: u64,
    pub block_root: Bytes32,
    pub state_root: Bytes32,
    pub withdrawals_root: Bytes32,
    pub checkpoint_hash: Bytes32,
}
```

**Methods:**

| Method | Signature | Description |
|--------|-----------|-------------|
| `from_checkpoint()` | `(network_id: Bytes32, checkpoint: &Checkpoint) -> Self` | Constructs from a checkpoint and network ID. |
| `hash()` | `(&self) -> Bytes32` | SHA-256 of `network_id \|\| epoch_le \|\| block_root \|\| state_root \|\| withdrawals_root \|\| checkpoint_hash`. |
| `test_coin_id()` | `(&self) -> Bytes32` | Derives a synthetic coin ID for L1 verification (SHA-256 of `hash()`). |

### 3.12 RewardDistribution

Per-epoch reward allocation across the five reward roles.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RewardDistribution {
    pub epoch: u64,
    pub total_reward: u64,            // Base epoch reward (from halving schedule)
    pub proposer_reward: u64,         // 10% of total_reward
    pub attester_reward: u64,         // 80% of total_reward
    pub ef_spawner_reward: u64,       // 3% of total_reward
    pub score_submitter_reward: u64,  // 4% of total_reward
    pub finalizer_reward: u64,        // 3% of total_reward
    pub total_fees: u64,              // Fees collected in this epoch
    pub proposer_fee_share: u64,      // 50% of total_fees
    pub burned_fees: u64,             // 50% of total_fees
}
```

### 3.13 EpochBlockLink

Block continuity proof within an epoch. Records the parent→child relationship between consecutive blocks. Used in L1 finalization payloads to prove that epoch blocks form a valid chain.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpochBlockLink {
    pub parent_hash: Bytes32,
    pub block_hash: Bytes32,
}
```

### 3.14 DFSP Epoch-Boundary Types

Types governing the deterministic seven-stage DFSP epoch-boundary execution pipeline.

#### 3.14.1 DfspExecutionStageV1

Execution stages run in the order listed. Each stage depends on the results of all preceding stages.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DfspExecutionStageV1 {
    /// Stage 2: Epoch-boundary burn transitions.
    /// Advances collateral records through grace/expired/removed states.
    EpochBurn,

    /// Stage 3: Collateral + CID-state deltas.
    /// Applies new collateral deposits, budget adjustments, CID lifecycle transitions.
    CollateralAndCid,

    /// Stage 4: Node registry deltas.
    /// Processes node join/leave/stake-change operations.
    NodeRegistry,

    /// Stage 5: Ingestion + retrieval proof deltas.
    /// Evaluates storage proofs submitted during the epoch.
    Proofs,

    /// Stage 6: Namespace pointer updates.
    /// Applies namespace assignment changes and computes cumulative root.
    Namespace,

    /// Stage 7: Finalize/commit four DFSP roots.
    /// Folds all delta results into final Merkle roots and commitment digest.
    FinalizeRoots,
}
```

#### 3.14.2 DfspEpochBurnContextV1

Deterministic burn transition context for epoch-boundary collateral decay.

```rust
#[derive(Debug, Clone)]
pub struct DfspEpochBurnContextV1 {
    /// Number of wall-clock epochs elapsed since the CID's last payment.
    pub elapsed_wall_clock_epochs: u64,
    /// Minimum bond threshold below which CIDs enter grace state.
    pub min_bond_threshold: u128,
}
```

#### 3.14.3 DfspEpochBurnPolicyV1

Protocol governance for epoch-boundary burn behavior.

```rust
#[derive(Debug, Clone)]
pub struct DfspEpochBurnPolicyV1 {
    /// Minimum bond threshold (mojos). CIDs below this enter grace.
    pub min_bond_threshold: u128,
    /// Wall-clock seconds per DFSP accounting epoch.
    pub wall_clock_epoch_seconds: u64,
}
```

#### 3.14.4 DfspEpochBurnPolicyScheduleEntryV1

Height-activated burn policy override. Allows burn parameters to change at specific L2 heights.

```rust
#[derive(Debug, Clone)]
pub struct DfspEpochBurnPolicyScheduleEntryV1 {
    /// L2 height at which this policy activates.
    pub start_height: u64,
    /// The burn policy in effect from start_height onward.
    pub policy: DfspEpochBurnPolicyV1,
}
```

#### 3.14.5 DfspEpochStorageProofEvaluationContextV1

Context for evaluating storage proofs submitted during an epoch (Stage 5).

```rust
#[derive(Debug, Clone)]
pub struct DfspEpochStorageProofEvaluationContextV1 {
    /// The epoch being evaluated.
    pub evaluated_epoch: u64,
    /// Node IDs that submitted valid retrieval proofs.
    pub proved_node_ids: Vec<NodeId>,
}
```

#### 3.14.6 DfspEpochIssuancePreviewV1

Preview of DFSP issuance rewards computed from retrieval proof shares (Stage 5 output).

```rust
#[derive(Debug, Clone)]
pub struct DfspEpochIssuancePreviewV1 {
    /// Per-node proof share counts.
    pub proof_shares_by_node: Vec<(NodeId, u64)>,
    /// Sum of all proof shares.
    pub total_proof_shares: u64,
    /// Total issuance pool available this epoch (mojos).
    pub epoch_total_issuance_pool_mojos: u128,
    /// Per-node issuance allocation (mojos).
    pub node_issuance_mojos: Vec<(NodeId, u128)>,
}
```

#### 3.14.7 DfspEpochBoundaryFinalizePreviewV1

Stage 6 output: finalize preview with reassignment plan, issuance, and operations digest.

```rust
#[derive(Debug, Clone)]
pub struct DfspEpochBoundaryFinalizePreviewV1 {
    /// The epoch being finalized.
    pub evaluated_epoch: u64,
    /// Fragment reassignment batch plan.
    pub reassignment_batch_plan: DfspEpochReassignmentBatchPlanPreviewV1,
    /// Issuance preview from proof evaluation.
    pub issuance_preview: DfspEpochIssuancePreviewV1,
    /// SHA-256 digest of all staged operations.
    pub operations_digest: Bytes32,
}
```

#### 3.14.8 DfspEpochBoundaryStagedOutputsV1

Complete staged outputs from the epoch-boundary pipeline. Captures all intermediate and final results.

```rust
#[derive(Debug, Clone)]
pub struct DfspEpochBoundaryStagedOutputsV1 {
    /// The epoch being processed.
    pub evaluated_epoch: u64,
    /// Fragment CIDs considered during reassignment.
    pub fragment_cids_considered: Vec<Cid>,
    /// Fragment CIDs actually reassigned.
    pub reassigned_fragments: Vec<Cid>,
    /// Reassignment batch plan.
    pub reassignment_batch_plan: DfspEpochReassignmentBatchPlanPreviewV1,
    /// Issuance preview.
    pub issuance_preview: DfspEpochIssuancePreviewV1,
    /// Finalize preview.
    pub finalize_preview: DfspEpochBoundaryFinalizePreviewV1,
}
```

#### 3.14.9 DfspFinalizeRootsCommitmentPreviewV1

Stage 7 output: four DFSP roots plus the operations digest, combined into a commitment digest.

```rust
#[derive(Debug, Clone)]
pub struct DfspFinalizeRootsCommitmentPreviewV1 {
    pub collateral_registry_root: Bytes32,
    pub cid_state_root: Bytes32,
    pub node_registry_root: Bytes32,
    pub namespace_epoch_root: Bytes32,
    pub operations_digest: Bytes32,
    /// SHA-256 of all five fields above. Placed in the block header's
    /// dfsp_finalize_commitment_root field.
    pub commitment_digest: Bytes32,
}
```

## 4. Epoch Phase State Machine

### 4.1 Phase Definitions

The epoch lifecycle is divided into four phases, each gated by L1 block height progress within the epoch's L1 window:

```
┌──────────────────────────────────────────────────────────────────────────┐
│                          Epoch L1 Window                                 │
│                                                                          │
│  0%              25%              50%              75%            100%    │
│  ├────────────────┼────────────────┼────────────────┼──────────────┤     │
│  │         Block Production        │   Checkpoint   │ Finalization │Done │
│  │      (propose + attest L2)      │   (submit)     │  (resolve)   │     │
│  └─────────────────────────────────┴────────────────┴──────────────┘     │
└──────────────────────────────────────────────────────────────────────────┘
```

| Phase | L1 Progress | Operations Allowed | Operations Prohibited |
|-------|------------|-------------------|----------------------|
| `BlockProduction` | 0% ≤ p < 50% | Propose L2 blocks, attest blocks, record fees/txns | Submit checkpoints, finalize competition |
| `Checkpoint` | 50% ≤ p < 75% | Submit checkpoint, collect competing submissions | Produce blocks, finalize competition |
| `Finalization` | 75% ≤ p < 100% | Select winner, compute rewards, apply DFSP close | Produce blocks, submit checkpoints |
| `Complete` | p ≥ 100% | Advance to next epoch | All production, submission, and finalization |

### 4.2 L1-Driven Phase Calculation

Phase is computed deterministically from three inputs:

```
Inputs:
  genesis_l1_height  — L1 height at network genesis
  epoch              — current epoch number
  current_l1_height  — latest observed L1 height

Derived:
  epoch_start_l1 = genesis_l1_height + (epoch × EPOCH_L1_BLOCKS)
  epoch_end_l1   = epoch_start_l1 + EPOCH_L1_BLOCKS
  progress       = current_l1_height - epoch_start_l1
  percentage     = (progress × 100) / EPOCH_L1_BLOCKS
```

```rust
pub fn l1_progress_phase_for_network_epoch(
    genesis_l1_height: u32,
    network_epoch: u64,
    current_l1_height: u32,
) -> EpochPhase {
    let epoch_start = genesis_l1_height + (network_epoch as u32 * EPOCH_L1_BLOCKS);
    let progress = current_l1_height.saturating_sub(epoch_start);
    let pct = (progress as u64 * 100) / EPOCH_L1_BLOCKS as u64;

    if pct < PHASE_BLOCK_PRODUCTION_END_PCT as u64 {
        EpochPhase::BlockProduction
    } else if pct < PHASE_CHECKPOINT_END_PCT as u64 {
        EpochPhase::Checkpoint
    } else if pct < PHASE_FINALIZATION_END_PCT as u64 {
        EpochPhase::Finalization
    } else {
        EpochPhase::Complete
    }
}
```

### 4.3 Phase Transition Detection

`EpochManager::update_phase(l1_height)` recalculates the phase and returns `Some(PhaseTransition)` if the phase changed:

```
EpochManager::update_phase(l1_height)
    │
    ├── calculate new phase from L1 progress
    ├── compare with stored phase
    ├── if different:
    │   ├── update stored phase
    │   └── return Some(PhaseTransition { epoch, from, to, l1_height })
    └── if same: return None
```

Phase transitions are monotonic for a given epoch — the phase index never decreases. This is guaranteed by L1 height monotonicity: if `l1_height_a < l1_height_b`, then `phase(l1_height_a).index() <= phase(l1_height_b).index()`.

## 5. Height–Epoch Arithmetic

All functions in this section are **stateless pure functions**. They require no `EpochManager` instance and can be called from any context.

### 5.1 Core Conversions

| Function | Formula | Examples |
|----------|---------|---------|
| `epoch_for_block_height(h)` | `(h - 1) / BLOCKS_PER_EPOCH` | h=1 → 0, h=32 → 0, h=33 → 1, h=64 → 1, h=65 → 2 |
| `first_height_in_epoch(e)` | `e × BLOCKS_PER_EPOCH + 1` | e=0 → 1, e=1 → 33, e=2 → 65 |
| `epoch_checkpoint_height(e)` | `(e + 1) × BLOCKS_PER_EPOCH` | e=0 → 32, e=1 → 64, e=2 → 96 |

**Epoch–height mapping diagram:**

```
Epoch 0:  heights  1  2  3  ...  31  [32]     ← 32 is checkpoint
Epoch 1:  heights 33 34 35  ...  63  [64]     ← 64 is checkpoint
Epoch 2:  heights 65 66 67  ...  95  [96]     ← 96 is checkpoint
```

### 5.2 Checkpoint Block Detection

| Function | Signature | Returns `true` when |
|----------|-----------|-------------------|
| `is_genesis_checkpoint_block(h)` | `(u64) -> bool` | `h == GENESIS_HEIGHT` (h == 1) |
| `is_epoch_checkpoint_block(h)` | `(u64) -> bool` | `h % BLOCKS_PER_EPOCH == 0` (h = 32, 64, 96, ...) |
| `is_checkpoint_class_block(h)` | `(u64) -> bool` | Genesis OR epoch checkpoint: `is_genesis_checkpoint_block(h) \|\| is_epoch_checkpoint_block(h)` |
| `is_first_block_after_epoch_checkpoint(h)` | `(u64) -> bool` | `h > 1 && (h - 1) % BLOCKS_PER_EPOCH == 0` (h = 33, 65, 97, ...) |

### 5.3 Boundary Guards

```rust
/// Enforces the empty-checkpoint-block invariant: checkpoint-class blocks
/// must contain zero SpendBundles and zero cost/fees.
///
/// Returns Ok(()) for non-checkpoint heights regardless of parameters.
/// Returns Err(EpochError::CheckpointBlockNotEmpty) if any count is non-zero
/// at a checkpoint-class height.
pub fn ensure_checkpoint_block_empty(
    height: u64,
    spend_bundle_count: u32,
    total_cost: u64,
    total_fees: u64,
) -> Result<(), EpochError>
```

### 5.4 Epoch L1 Range

```rust
/// Returns the (start_l1_height, end_l1_height) for a given epoch,
/// given the network's genesis L1 height.
pub fn l1_range_for_epoch(genesis_l1_height: u32, epoch: u64) -> (u32, u32) {
    let start = genesis_l1_height + (epoch as u32 * EPOCH_L1_BLOCKS);
    let end = start + EPOCH_L1_BLOCKS;
    (start, end)
}
```

### 5.5 Committed Height Capping (DL-CKP-001)

```rust
/// Returns the last L2 height that should be included in this epoch's
/// checkpoint. Caps at the epoch checkpoint height, even if the chain
/// tip is higher. Prevents namespace rollup from bleeding into the
/// next epoch.
pub fn last_committed_height_in_epoch(epoch: u64, tip_height: u64) -> u64 {
    let ckp = epoch_checkpoint_height(epoch);
    std::cmp::min(tip_height, ckp)
}
```

## 6. Epoch Manager

The `EpochManager` is the central coordinator for epoch lifecycle. It tracks the current epoch's mutable state, archives completed epochs, manages checkpoint competitions, and stores reward distributions.

### 6.1 Internal State

```rust
pub struct EpochManager {
    current_epoch: RwLock<EpochInfo>,
    history: RwLock<HashMap<u64, EpochSummary>>,
    competitions: RwLock<HashMap<u64, CheckpointCompetition>>,
    rewards: RwLock<HashMap<u64, RewardDistribution>>,
    network_id: Bytes32,
    genesis_l1_height: u32,
}
```

All fields are guarded by `RwLock` for interior mutability, allowing shared references (`&self`) across concurrent readers with exclusive access for writers. `network_id` and `genesis_l1_height` are immutable after construction.

### 6.2 Construction

```rust
pub fn new(
    network_id: Bytes32,
    genesis_l1_height: u32,
    initial_state_root: Bytes32,
) -> Self
```

Creates an `EpochManager` at epoch 0 with:
- `current_epoch` = `EpochInfo::new(0, genesis_l1_height, GENESIS_HEIGHT, initial_state_root)`
- Empty `history`, `competitions`, and `rewards` maps.

### 6.3 Phase Tracking

| Method | Signature | Description |
|--------|-----------|-------------|
| `current_epoch()` | `(&self) -> u64` | Returns the current epoch number. |
| `current_epoch_info()` | `(&self) -> EpochInfo` | Clone of the current epoch's full state. |
| `current_phase()` | `(&self) -> EpochPhase` | Current phase of the current epoch. |
| `update_phase()` | `(&self, l1_height: u32) -> Option<PhaseTransition>` | Recalculates phase from L1 progress; returns transition if changed. |
| `should_advance()` | `(&self, l1_height: u32) -> bool` | True if the epoch is `Complete` and ready to advance. |

### 6.4 Block Recording

```rust
/// Records a block's contribution to the current epoch.
/// Increments blocks_produced, accumulates fees and transaction count.
pub fn record_block(&self, fees: u64, tx_count: u64)

/// Overwrites the current epoch's chain totals. Used when replaying
/// or catching up from persisted state.
pub fn set_current_epoch_chain_totals(
    &self,
    blocks_produced: u32,
    total_fees: u64,
    total_transactions: u64,
)
```

### 6.5 Checkpoint Competition Management

| Method | Signature | Description |
|--------|-----------|-------------|
| `start_checkpoint_competition()` | `(&self) -> Result<(), EpochError>` | Creates a new `CheckpointCompetition` for the current epoch in `Collecting` state. |
| `get_competition()` | `(&self, epoch: u64) -> Option<CheckpointCompetition>` | Returns a clone of the competition for a given epoch. |
| `submit_checkpoint()` | `(&self, submission: CheckpointSubmission) -> Result<bool, EpochError>` | Adds a submission to the current epoch's competition. Returns `true` if it is the new leader. Returns `Err` if the competition hasn't started or the epoch doesn't match. |
| `finalize_competition()` | `(&self, epoch: u64) -> Result<Option<Checkpoint>, EpochError>` | Selects the winning checkpoint and transitions the competition to `WinnerSelected`. Returns `None` if no submissions. Sets the checkpoint on the current `EpochInfo`. |

### 6.6 DFSP Close Snapshot

```rust
/// Applies DFSP close values to the current epoch before advance.
/// Must be called after DFSP epoch-boundary processing completes
/// and before advance_epoch().
///
/// Copies all seven DfspCloseSnapshot fields into the current EpochInfo.
pub fn set_current_epoch_dfsp_close_snapshot(&self, snap: DfspCloseSnapshot)
```

### 6.7 Epoch Advancement

```rust
/// Archives the current epoch as an EpochSummary, then creates
/// a new epoch at the next epoch number.
///
/// Steps:
///   1. Read current EpochInfo
///   2. Create EpochSummary from current state (including DFSP close snapshot)
///   3. Insert summary into history map keyed by epoch number
///   4. Compute new epoch's L1 start height: genesis_l1_height + (new_epoch × EPOCH_L1_BLOCKS)
///   5. Compute new epoch's L2 start height: first_height_in_epoch(new_epoch)
///   6. Create new EpochInfo for epoch N+1 with provided state_root
///   7. Replace current_epoch with new EpochInfo
///   8. Return the new epoch number
pub fn advance_epoch(
    &self,
    l1_height: u32,
    state_root: Bytes32,
) -> Result<u64, EpochError>
```

### 6.8 Reward Management

| Method | Signature | Description |
|--------|-----------|-------------|
| `store_rewards()` | `(&self, distribution: RewardDistribution)` | Archives per-epoch reward distribution, keyed by epoch number. |
| `get_rewards()` | `(&self, epoch: u64) -> Option<RewardDistribution>` | Retrieves rewards for a completed epoch. |

### 6.9 Query API

| Method | Signature | Description |
|--------|-----------|-------------|
| `genesis_l1_height()` | `(&self) -> u32` | Returns the network's genesis L1 height. |
| `network_id()` | `(&self) -> &Bytes32` | Returns the network ID. |
| `get_epoch_info()` | `(&self, epoch: u64) -> Option<EpochInfo>` | Returns `Some(EpochInfo)` for current epoch; `None` for historical (use `get_epoch_summary` instead). |
| `get_epoch_summary()` | `(&self, epoch: u64) -> Option<EpochSummary>` | Returns archived summary for a completed epoch. |
| `epoch_for_l1_height()` | `(&self, l1_height: u32) -> u64` | Maps an L1 height to its epoch number. |
| `l1_range_for_epoch()` | `(&self, epoch: u64) -> (u32, u32)` | Returns (start, end) L1 heights for an epoch. |
| `recent_summaries()` | `(&self, count: usize) -> Vec<EpochSummary>` | Returns the `count` most recent completed epoch summaries, newest first. |
| `total_stats()` | `(&self) -> EpochStats` | Aggregate statistics across all managed epochs. |

## 7. Epoch Verification

### 7.1 Block Root Computation

```rust
/// Computes the Merkle root of an epoch's block hashes.
/// The block hashes must be in height order (ascending).
///
/// Implementation: delegates to chia-sdk-types::MerkleTree::new(&block_hashes).root().
/// This produces a balanced binary SHA-256 Merkle tree with tagged hashing:
/// leaf prefix 0x02, internal node prefix 0x01 (matching Chia's MerkleTree convention).
///
/// Returns EMPTY_ROOT if block_hashes is empty.
pub fn compute_epoch_block_root(block_hashes: &[Bytes32]) -> Bytes32 {
    if block_hashes.is_empty() {
        return EMPTY_ROOT;
    }
    chia_sdk_types::MerkleTree::new(block_hashes).root()
}
```

The epoch block root is included in the `Checkpoint::block_root` field and proves that the checkpoint covers a specific set of blocks in a specific order.

**Chia crate:** Uses [`chia-sdk-types::MerkleTree`](https://docs.rs/chia-sdk-types) directly — not a custom Merkle tree implementation. This ensures the epoch block root is computed identically to how Chia tooling computes Merkle roots, enabling cross-ecosystem verification.

### 7.2 Block Root Inclusion Proofs

```rust
/// Generates a Merkle inclusion proof for a specific block hash within
/// an epoch's block root.
///
/// Implementation: delegates to chia-sdk-types::MerkleTree::new(&block_hashes).proof(index).
/// Returns a MerkleProof { path, proof } that can be verified against the block root.
pub fn epoch_block_inclusion_proof(
    block_hashes: &[Bytes32],
    index: usize,
) -> Option<chia_sdk_types::MerkleProof>
```

**Chia crate:** Uses [`chia-sdk-types::MerkleProof`](https://docs.rs/chia-sdk-types) directly for proof generation and verification. Light clients can use this to verify that a specific block is part of a checkpoint's `block_root` without downloading all epoch blocks.

### 7.3 Withdrawal Root Computation

```rust
/// Computes the Merkle set root of withdrawal requests in an epoch.
/// Withdrawal requests are an unordered set — order doesn't matter.
///
/// Implementation: delegates to chia-consensus::compute_merkle_set_root()
/// which produces an order-independent root, matching Chia L1's additions/removals
/// root construction.
///
/// Returns EMPTY_ROOT if withdrawal_hashes is empty.
pub fn compute_epoch_withdrawals_root(withdrawal_hashes: &[Bytes32]) -> Bytes32
```

**Chia crate:** Uses [`chia-consensus::compute_merkle_set_root()`](https://github.com/Chia-Network/chia_rs) directly for order-independent Merkle set construction. This matches the same function used by `dig-block` for additions/removals roots, ensuring consistency across the DIG ecosystem.

### 7.4 Checkpoint Data Verification

`EpochCheckpointData` bundles the fields needed to verify a checkpoint's authenticity:

1. **Construction**: `from_checkpoint(network_id, &checkpoint)` extracts the verification fields from a `Checkpoint` (from `dig-block`) and binds them to the network ID.
2. **Hashing**: `hash()` computes `SHA-256(network_id || epoch_le || block_root || state_root || withdrawals_root || checkpoint_hash)` where `epoch_le` is the epoch number in little-endian 8-byte encoding. Uses `chia-sha2::Sha256` for hashing.
3. **Coin ID derivation**: `test_coin_id()` derives a synthetic coin ID from `hash()` for L1 verification, enabling the L1 driver to locate the finalization coin.

### 7.5 Checkpoint Signing Material

```rust
/// Builds the signing digest and checkpoint from the epoch's blocks.
///
/// Steps:
///   1. Compute block_root via MerkleTree::new(&block_hashes).root()
///      (chia-sdk-types::MerkleTree)
///   2. Extract state_root from the last block header
///   3. Accumulate tx_count and total_fees from all headers
///   4. Construct Checkpoint (from dig-block)
///   5. Compute score = stake_pct × block_count
///   6. Compute signing_digest = SHA-256(DOMAIN_CHECKPOINT || checkpoint fields)
///      using chia-sha2::Sha256, with domain separation via
///      chia-sdk-signer::AggSigConstants
///
/// Inputs:
///   network_id       — Network identifier
///   epoch            — Epoch number
///   epoch_blocks     — All block headers in the epoch (height order)
///   stake_pct        — Submitter's stake percentage for score computation
///   prev_checkpoint  — Hash of the previous epoch's checkpoint
///   withdrawals_root — Merkle root of withdrawal requests in this epoch
///   withdrawal_count — Number of withdrawal requests
///
/// Outputs:
///   EpochCheckpointSignMaterial containing the Checkpoint, score, and
///   signing_digest.
pub fn epoch_checkpoint_sign_material_from_l2_blocks(
    network_id: Bytes32,
    epoch: u64,
    epoch_blocks: &[L2BlockHeader],
    stake_pct: u64,
    prev_checkpoint: Bytes32,
    withdrawals_root: Bytes32,
    withdrawal_count: u32,
) -> Result<EpochCheckpointSignMaterial, EpochError>
```

```rust
#[derive(Debug, Clone)]
pub struct EpochCheckpointSignMaterial {
    pub checkpoint: Checkpoint,
    pub score: u64,
    pub signing_digest: Bytes32,
}
```

### 7.6 Aggregate Signature Construction

```rust
/// Constructs an aggregate checkpoint submission from per-validator signatures.
///
/// Uses chia-bls::aggregate() to combine individual BLS signatures into a
/// single aggregate signature, and chia-bls::aggregate_verify() to validate
/// the result against the signing digest and validator public keys.
///
/// This is the same BLS aggregation used by dig-block for per-SpendBundle
/// signatures — not a custom aggregation scheme.
pub fn stored_checkpoint_from_epoch_sign_material_with_aggregate_v1(
    material: &EpochCheckpointSignMaterial,
    validator_set: &[(u32, PublicKey)],   // (index, pubkey) pairs
    per_validator: &[(u32, PublicKey, Signature)],  // (index, pubkey, sig) per signer
    submitter: u32,
) -> Result<CheckpointSubmission, EpochError>
```

**Chia crates used:** `chia-bls::aggregate()` to combine per-validator `Signature` values, `chia-bls::aggregate_verify()` to validate the aggregate against `(PublicKey, signing_digest)` pairs, `chia-sdk-signer::AggSigConstants` for domain separation. No custom BLS wrapper.

## 8. Reward Economics

### 8.1 Block Reward Curve

Block rewards follow a halving schedule with tail emissions:

```rust
pub fn block_reward_at_height(height: u64) -> u64 {
    let halvings = height / HALVING_INTERVAL_BLOCKS;
    if halvings >= HALVINGS_BEFORE_TAIL {
        TAIL_BLOCK_REWARD
    } else {
        INITIAL_BLOCK_REWARD >> halvings
    }
}
```

| Halving | Block Range | Reward per Block | L2 Value |
|---------|------------|------------------|----------|
| 0 | 0 – 94,607,999 | 320,000,000,000 mojos | 0.32 L2 |
| 1 | 94,608,000 – 189,215,999 | 160,000,000,000 mojos | 0.16 L2 |
| 2 | 189,216,000 – 283,823,999 | 80,000,000,000 mojos | 0.08 L2 |
| 3 | 283,824,000 – 378,431,999 | 40,000,000,000 mojos | 0.04 L2 |
| 4+ (tail) | 378,432,000+ | 20,000,000,000 mojos | 0.02 L2 |

### 8.2 Epoch-Opening Bonus

The first block produced after an epoch checkpoint (`is_first_block_after_epoch_checkpoint(h) == true`) receives an additional `EPOCH_FIRST_BLOCK_BONUS` (100,000,000,000 mojos = 0.1 L2).

```rust
pub fn total_block_reward(height: u64, is_first_block_of_epoch: bool) -> u64 {
    let base = block_reward_at_height(height);
    if is_first_block_of_epoch {
        base + EPOCH_FIRST_BLOCK_BONUS
    } else {
        base
    }
}
```

### 8.3 Fee Distribution

Fees collected within an epoch are split between the proposer and burn:

```rust
pub fn proposer_fee_share(total_fees: u64) -> u64 {
    total_fees * FEE_PROPOSER_SHARE_PCT / 100
}

pub fn burned_fee_remainder(total_fees: u64) -> u64 {
    total_fees - proposer_fee_share(total_fees)
}
```

| Recipient | Share | Formula |
|-----------|-------|---------|
| Proposer | 50% | `total_fees × 50 / 100` |
| Burn | 50% | `total_fees - proposer_fee_share` |

### 8.4 Epoch Reward Distribution

The epoch-level reward pool is distributed across five roles:

| Role | Share | Constant | Purpose |
|------|-------|----------|---------|
| Proposer | 10% | `PROPOSER_REWARD_SHARE` | Block production incentive |
| Attester | 80% | `ATTESTER_REWARD_SHARE` | Attestation participation incentive |
| EF Spawner | 3% | `EF_SPAWNER_REWARD_SHARE` | Epoch finalizer coin spawning on L1 |
| Score Submitter | 4% | `SCORE_SUBMITTER_REWARD_SHARE` | Checkpoint score submission to L1 |
| Finalizer | 3% | `FINALIZER_REWARD_SHARE` | L1 epoch finalization transaction |

```rust
pub fn compute_reward_distribution(
    epoch: u64,
    total_reward: u64,
    total_fees: u64,
) -> RewardDistribution {
    RewardDistribution {
        epoch,
        total_reward,
        proposer_reward: total_reward * PROPOSER_REWARD_SHARE / 100,
        attester_reward: total_reward * ATTESTER_REWARD_SHARE / 100,
        ef_spawner_reward: total_reward * EF_SPAWNER_REWARD_SHARE / 100,
        score_submitter_reward: total_reward * SCORE_SUBMITTER_REWARD_SHARE / 100,
        finalizer_reward: total_reward * FINALIZER_REWARD_SHARE / 100,
        total_fees,
        proposer_fee_share: proposer_fee_share(total_fees),
        burned_fees: burned_fee_remainder(total_fees),
    }
}
```

## 9. DFSP Epoch-Boundary Processing

At each epoch boundary, the DFSP (Decentralized File Storage Protocol) subsystem performs a deterministic seven-stage execution pipeline to update its four Sparse Merkle Tree roots and compute issuance rewards.

### 9.1 Execution Pipeline

```
Stage 1: [Input Collection]    — Gather epoch blocks, extract REMARK conditions
    ↓
Stage 2: EpochBurn             — Apply burn transitions (grace → expired → removed)
    ↓
Stage 3: CollateralAndCid      — Apply collateral deltas, CID state transitions
    ↓
Stage 4: NodeRegistry          — Apply node registry deltas (join/leave/stake changes)
    ↓
Stage 5: Proofs                — Evaluate ingestion + retrieval storage proofs
    ↓
Stage 6: Namespace             — Apply namespace pointer updates, compute cumulative root
    ↓
Stage 7: FinalizeRoots         — Compute four DFSP roots + commitment digest
```

Each stage reads the outputs of all preceding stages and produces its own outputs. The pipeline is deterministic: given the same epoch blocks and starting DFSP state, all validators produce identical results.

### 9.2 Burn Policy

The burn policy governs how collateral decays across epoch boundaries. CIDs whose bond falls below `min_bond_threshold` enter grace state; those that exceed `DFSP_GRACE_PERIOD_NETWORK_EPOCHS` in grace are expired and removed.

```rust
/// Returns the protocol default burn policy.
pub fn protocol_dfsp_epoch_burn_policy() -> DfspEpochBurnPolicyV1

/// Resolves the active burn policy at a given L2 height from a schedule.
/// Scans the schedule in reverse order; returns the first entry where
/// start_height <= height. Falls back to protocol default if no match.
pub fn resolve_dfsp_epoch_burn_policy_from_schedule(
    height: u64,
    schedule: &[DfspEpochBurnPolicyScheduleEntryV1],
) -> DfspEpochBurnPolicyV1

/// Parses a burn policy schedule from a string configuration.
pub fn parse_dfsp_epoch_burn_policy_schedule_v1(
    schedule_str: &str,
) -> Result<Vec<DfspEpochBurnPolicyScheduleEntryV1>, EpochError>
```

### 9.3 Storage Proof Evaluation

```rust
/// Evaluates storage proofs submitted during the epoch.
/// Determines which nodes provided valid retrieval proofs
/// and computes their proof shares for issuance allocation.
pub fn apply_epoch_storage_proof_evaluation_step_v1(
    context: &DfspEpochStorageProofEvaluationContextV1,
    // ... additional DFSP state inputs
) -> DfspEpochIssuancePreviewV1
```

### 9.4 Root Commitment

After all stages complete, the four DFSP roots and an operations digest are combined into a single commitment:

```rust
/// Computes the epoch-boundary operations digest from the reassignment plan
/// and issuance preview. This digest binds the staged operations so they
/// can be independently verified.
///
/// Uses chia-sha2::Sha256 for hashing: SHA-256(epoch_le || plan_bytes || issuance_bytes).
pub fn compute_epoch_boundary_operations_digest_v1(
    evaluated_epoch: u64,
    plan: &DfspEpochReassignmentBatchPlanPreviewV1,
    issuance: &DfspEpochIssuancePreviewV1,
) -> Bytes32

/// Computes the finalize roots commitment digest: SHA-256 of all four
/// DFSP roots plus the operations digest. This value is placed in the
/// block header's dfsp_finalize_commitment_root field.
///
/// Uses chia-sha2::Sha256 for hashing: SHA-256(collateral_root || cid_root ||
/// node_root || namespace_root || operations_digest).
pub fn compute_dfsp_finalize_roots_commitment_digest_v1(
    roots: &DfspFinalizeRootsCommitmentPreviewV1,
    operations_digest: Bytes32,
) -> Bytes32
```

**Chia crate:** Both digest functions use `chia-sha2::Sha256` — the same SHA-256 implementation used by `Coin::coin_id()`, `dig-block`'s header hashing, and the rest of the Chia ecosystem.

### 9.5 DFSP Tail Roots for Checkpoint Signing

```rust
/// Extracts the four DFSP tail roots from the last block in the epoch
/// that carries DFSP roots. Used as input to checkpoint signing.
///
/// Returns (collateral_registry_root, cid_state_root, node_registry_root,
///          namespace_epoch_root).
///
/// Returns Err if no blocks have DFSP roots (DFSP not active).
pub fn dfsp_checkpoint_signing_tail_roots_v1(
    blocks: &[L2BlockHeader],
) -> Result<(Bytes32, Bytes32, Bytes32, Bytes32), EpochError>
```

### 9.6 Namespace Epoch Root Rollup (DL-CKP-004)

```rust
/// Computes the cumulative namespace epoch root by rolling up
/// per-block namespace_update_root values across the epoch.
/// Only includes heights up to last_committed_height_in_epoch()
/// to prevent bleed into the next epoch.
///
/// The rollup is order-dependent: namespace roots are combined
/// sequentially by block height.
pub fn compute_dfsp_namespace_epoch_root_rollup_v1(
    blocks: &[L2BlockHeader],
) -> Result<Bytes32, EpochError>
```

### 9.7 DFSP Activation Control

```rust
/// Returns the DFSP activation height for the current network.
/// Checks the DIG_DFSP_ACTIVATION_HEIGHT environment variable first;
/// falls back to DFSP_ACTIVATION_HEIGHT constant (u64::MAX = disabled).
pub fn dfsp_activation_height_for_network() -> u64

/// Returns true if DFSP is active at the given L2 height.
pub fn is_dfsp_active_at_height(height: u64) -> bool

/// Sets a runtime override for DFSP activation height (testing only).
pub fn set_dfsp_activation_height_override(height: Option<u64>)
```

## 10. Error Types

### 10.1 EpochError

```rust
#[derive(Debug, Clone, thiserror::Error)]
pub enum EpochError {
    /// Attempted to advance an epoch that hasn't reached Complete phase.
    #[error("Cannot advance: epoch {0} is not complete")]
    EpochNotComplete(u64),

    /// Attempted to advance an epoch with no finalized checkpoint.
    #[error("Cannot advance: epoch {0} has no finalized checkpoint")]
    NoFinalizedCheckpoint(u64),

    /// Checkpoint-class block contains non-zero SpendBundles, cost, or fees.
    #[error("Checkpoint block at height {0} is not empty: {1} bundles, {2} cost, {3} fees")]
    CheckpointBlockNotEmpty(u64, u32, u64, u64),

    /// Operation requires a specific phase but the epoch is in a different one.
    #[error("Phase mismatch: expected {expected}, got {got}")]
    PhaseMismatch {
        expected: EpochPhase,
        got: EpochPhase,
    },

    /// Submission or query references the wrong epoch.
    #[error("Epoch mismatch: expected {expected}, got {got}")]
    EpochMismatch { expected: u64, got: u64 },

    /// L2 height is below genesis (height 0 or underflow).
    #[error("Invalid height {0}: below genesis")]
    InvalidHeight(u64),

    /// DFSP operation attempted at a height before activation.
    #[error("DFSP not active at height {0}")]
    DfspNotActive(u64),

    /// DFSP epoch-boundary processing error.
    #[error("DFSP epoch-boundary error: {0}")]
    DfspBoundary(String),

    /// Checkpoint competition error (delegated).
    #[error("Competition error: {0}")]
    Competition(#[from] CheckpointCompetitionError),
}
```

### 10.2 CheckpointCompetitionError

```rust
#[derive(Debug, Clone, thiserror::Error)]
pub enum CheckpointCompetitionError {
    /// Checkpoint data failed validation.
    #[error("Invalid checkpoint data: {0}")]
    InvalidData(String),

    /// No competition exists for the requested epoch.
    #[error("Checkpoint competition not found for epoch {0}")]
    NotFound(u64),

    /// Submitted checkpoint's score does not exceed the current leader.
    #[error("Score not higher: current {current}, submitted {submitted}")]
    ScoreNotHigher { current: u64, submitted: u64 },

    /// Submission's epoch field doesn't match the competition's epoch.
    #[error("Epoch mismatch: expected {expected}, got {got}")]
    EpochMismatch { expected: u64, got: u64 },

    /// Competition has already been finalized; no more submissions accepted.
    #[error("Competition already finalized")]
    AlreadyFinalized,

    /// Competition hasn't been started yet (still in Pending state).
    #[error("Competition not started")]
    NotStarted,
}
```

## 11. Serialization

All data types use **bincode** for compact binary serialization, consistent with `dig-block`.

### 11.1 Serializable Types

| Type | `to_bytes()` | `from_bytes()` |
|------|-------------|----------------|
| `EpochInfo` | `&self -> Vec<u8>` | `&[u8] -> Result<Self, EpochError>` |
| `EpochSummary` | `&self -> Vec<u8>` | `&[u8] -> Result<Self, EpochError>` |
| `EpochCheckpointData` | `&self -> Vec<u8>` | `&[u8] -> Result<Self, EpochError>` |
| `RewardDistribution` | `&self -> Vec<u8>` | `&[u8] -> Result<Self, EpochError>` |
| `CheckpointCompetition` | `&self -> Vec<u8>` | `&[u8] -> Result<Self, EpochError>` |
| `EpochBlockLink` | `&self -> Vec<u8>` | `&[u8] -> Result<Self, EpochError>` |

### 11.2 Conventions

- **Round-trip guarantee**: `from_bytes(to_bytes(x)) == x` for all types.
- **Encoding**: Bincode with little-endian byte order and variable-length integers.
- **Derivation**: All serializable types derive `Serialize` + `Deserialize` from `serde`.
- **Error mapping**: Bincode deserialization errors are wrapped in the appropriate `EpochError` variant.

## 12. Public API Summary

### 12.1 Constants

| Constant | Value | Section |
|----------|-------|---------|
| `BLOCKS_PER_EPOCH` | `32` | 2.1 |
| `EPOCH_L1_BLOCKS` | `32` | 2.1 |
| `GENESIS_HEIGHT` | `1` | 2.1 |
| `PHASE_BLOCK_PRODUCTION_END_PCT` | `50` | 2.2 |
| `PHASE_CHECKPOINT_END_PCT` | `75` | 2.2 |
| `PHASE_FINALIZATION_END_PCT` | `100` | 2.2 |
| `MOJOS_PER_L2` | `1_000_000_000_000` | 2.3 |
| `INITIAL_BLOCK_REWARD` | `320_000_000_000` | 2.3 |
| `TAIL_BLOCK_REWARD` | `20_000_000_000` | 2.3 |
| `HALVING_INTERVAL_BLOCKS` | `94_608_000` | 2.3 |
| `HALVINGS_BEFORE_TAIL` | `4` | 2.3 |
| `INITIAL_EPOCH_REWARD` | `32_000_000_000_000` | 2.3 |
| `HALVING_INTERVAL_EPOCHS` | `315_576` | 2.3 |
| `MINIMUM_EPOCH_REWARD` | `2_000_000_000_000` | 2.3 |
| `EPOCH_FIRST_BLOCK_BONUS` | `100_000_000_000` | 2.3 |
| `FEE_PROPOSER_SHARE_PCT` | `50` | 2.3 |
| `FEE_BURN_SHARE_PCT` | `50` | 2.3 |
| `PROPOSER_REWARD_SHARE` | `10` | 2.3 |
| `ATTESTER_REWARD_SHARE` | `80` | 2.3 |
| `EF_SPAWNER_REWARD_SHARE` | `3` | 2.3 |
| `SCORE_SUBMITTER_REWARD_SHARE` | `4` | 2.3 |
| `FINALIZER_REWARD_SHARE` | `3` | 2.3 |
| `DFSP_WALL_CLOCK_EPOCH_SECONDS` | `96` | 2.4 |
| `DFSP_GRACE_PERIOD_NETWORK_EPOCHS` | `27_000` | 2.4 |
| `DFSP_GENESIS_ISSUANCE_SUBSIDY_MOJOS_V1` | `0` | 2.4 |
| `DFSP_ACTIVATION_HEIGHT` | `u64::MAX` | 2.4 |
| `SOFT_FINALITY_THRESHOLD_PCT` | `67` | 2.5 |
| `HARD_FINALITY_THRESHOLD_PCT` | `67` | 2.5 |
| `CHECKPOINT_THRESHOLD_PCT` | `67` | 2.5 |
| `CORRELATION_WINDOW_EPOCHS` | `36` | 2.6 |
| `SLASH_LOOKBACK_EPOCHS` | `1_000` | 2.6 |
| `WITHDRAWAL_DELAY_EPOCHS` | `50` | 2.6 |

### 12.2 Height–Epoch Arithmetic (free functions)

| Function | Signature | Description |
|----------|-----------|-------------|
| `epoch_for_block_height()` | `(u64) -> u64` | Maps L2 height to epoch number. |
| `first_height_in_epoch()` | `(u64) -> u64` | First L2 height in an epoch. |
| `epoch_checkpoint_height()` | `(u64) -> u64` | Checkpoint height for an epoch. |
| `last_committed_height_in_epoch()` | `(u64, u64) -> u64` | Capped height for epoch checkpoint (DL-CKP-001). |
| `is_genesis_checkpoint_block()` | `(u64) -> bool` | True at height 1. |
| `is_epoch_checkpoint_block()` | `(u64) -> bool` | True at multiples of `BLOCKS_PER_EPOCH`. |
| `is_checkpoint_class_block()` | `(u64) -> bool` | Genesis or epoch checkpoint. |
| `is_first_block_after_epoch_checkpoint()` | `(u64) -> bool` | True at h=33, 65, 97, ... |
| `ensure_checkpoint_block_empty()` | `(u64, u32, u64, u64) -> Result<(), EpochError>` | Enforces empty checkpoint invariant. |
| `l1_range_for_epoch()` | `(u32, u64) -> (u32, u32)` | L1 height range for an epoch. |

### 12.3 Phase Functions (free functions)

| Function | Signature | Description |
|----------|-----------|-------------|
| `l1_progress_phase_for_network_epoch()` | `(u32, u64, u32) -> EpochPhase` | Computes phase from genesis L1 height, epoch number, and current L1 height. |

### 12.4 Reward Functions (free functions)

| Function | Signature | Description |
|----------|-----------|-------------|
| `block_reward_at_height()` | `(u64) -> u64` | Base block reward with halving. |
| `total_block_reward()` | `(u64, bool) -> u64` | Base reward + optional epoch-opening bonus. |
| `proposer_fee_share()` | `(u64) -> u64` | Proposer's 50% cut of fees. |
| `burned_fee_remainder()` | `(u64) -> u64` | Burned 50% of fees. |
| `compute_reward_distribution()` | `(u64, u64, u64) -> RewardDistribution` | Full five-way epoch reward distribution. |

### 12.5 Verification Functions (free functions)

| Function | Signature | Description | Chia crates used |
|----------|-----------|-------------|-----------------|
| `compute_epoch_block_root()` | `(&[Bytes32]) -> Bytes32` | Merkle root of epoch block hashes. | `chia-sdk-types::MerkleTree` |
| `epoch_block_inclusion_proof()` | `(&[Bytes32], usize) -> Option<MerkleProof>` | Merkle inclusion proof for a block in epoch root. | `chia-sdk-types::MerkleTree`, `MerkleProof` |
| `compute_epoch_withdrawals_root()` | `(&[Bytes32]) -> Bytes32` | Order-independent Merkle set root of withdrawals. | `chia-consensus::compute_merkle_set_root` |
| `epoch_checkpoint_sign_material_from_l2_blocks()` | `(Bytes32, u64, &[L2BlockHeader], u64, Bytes32, Bytes32, u32) -> Result<EpochCheckpointSignMaterial>` | Builds checkpoint signing material from epoch blocks. | `chia-sdk-types::MerkleTree`, `chia-sha2`, `chia-sdk-signer::AggSigConstants` |
| `stored_checkpoint_from_epoch_sign_material_with_aggregate_v1()` | `(&EpochCheckpointSignMaterial, &[(u32, PublicKey)], &[(u32, PublicKey, Signature)], u32) -> Result<CheckpointSubmission>` | Constructs aggregate checkpoint submission from per-validator signatures. | `chia-bls::aggregate`, `chia-bls::aggregate_verify` |

### 12.6 DFSP Epoch-Boundary Functions (free functions)

| Function | Signature | Description |
|----------|-----------|-------------|
| `protocol_dfsp_epoch_burn_policy()` | `() -> DfspEpochBurnPolicyV1` | Default protocol burn policy. |
| `resolve_dfsp_epoch_burn_policy_from_schedule()` | `(u64, &[DfspEpochBurnPolicyScheduleEntryV1]) -> DfspEpochBurnPolicyV1` | Height-aware policy lookup. |
| `parse_dfsp_epoch_burn_policy_schedule_v1()` | `(&str) -> Result<Vec<DfspEpochBurnPolicyScheduleEntryV1>>` | Parse schedule from config string. |
| `compute_epoch_boundary_operations_digest_v1()` | `(u64, &..., &...) -> Bytes32` | Operations digest for epoch boundary. |
| `compute_dfsp_finalize_roots_commitment_digest_v1()` | `(&DfspFinalizeRootsCommitmentPreviewV1, Bytes32) -> Bytes32` | Final commitment digest for header. |
| `dfsp_checkpoint_signing_tail_roots_v1()` | `(&[L2BlockHeader]) -> Result<(Bytes32, Bytes32, Bytes32, Bytes32)>` | Extract DFSP tail roots from epoch blocks. |
| `compute_dfsp_namespace_epoch_root_rollup_v1()` | `(&[L2BlockHeader]) -> Result<Bytes32>` | Cumulative namespace root (DL-CKP-004). |
| `dfsp_activation_height_for_network()` | `() -> u64` | Runtime DFSP activation height. |
| `is_dfsp_active_at_height()` | `(u64) -> bool` | Check DFSP activation at height. |
| `set_dfsp_activation_height_override()` | `(Option<u64>)` | Testing-only activation override. |

### 12.7 EpochManager Methods

| Method | Signature | Description |
|--------|-----------|-------------|
| `new()` | `(Bytes32, u32, Bytes32) -> Self` | Constructor at epoch 0. |
| `genesis_l1_height()` | `(&self) -> u32` | Genesis L1 height. |
| `network_id()` | `(&self) -> &Bytes32` | Network ID. |
| `current_epoch()` | `(&self) -> u64` | Current epoch number. |
| `current_epoch_info()` | `(&self) -> EpochInfo` | Current epoch state (clone). |
| `current_phase()` | `(&self) -> EpochPhase` | Current phase. |
| `update_phase()` | `(&self, u32) -> Option<PhaseTransition>` | Recalculate phase from L1 height. |
| `should_advance()` | `(&self, u32) -> bool` | Check if epoch is Complete. |
| `record_block()` | `(&self, u64, u64)` | Record block fees and tx count. |
| `set_current_epoch_chain_totals()` | `(&self, u32, u64, u64)` | Overwrite counters (replay). |
| `set_current_epoch_dfsp_close_snapshot()` | `(&self, DfspCloseSnapshot)` | Apply DFSP close before advance. |
| `start_checkpoint_competition()` | `(&self) -> Result<()>` | Open competition for current epoch. |
| `get_competition()` | `(&self, u64) -> Option<CheckpointCompetition>` | Query competition by epoch. |
| `submit_checkpoint()` | `(&self, CheckpointSubmission) -> Result<bool>` | Submit to competition. |
| `finalize_competition()` | `(&self, u64) -> Result<Option<Checkpoint>>` | Select winner. |
| `advance_epoch()` | `(&self, u32, Bytes32) -> Result<u64>` | Archive current, create next. |
| `epoch_for_l1_height()` | `(&self, u32) -> u64` | Map L1 height to epoch. |
| `l1_range_for_epoch()` | `(&self, u64) -> (u32, u32)` | Epoch L1 range. |
| `store_rewards()` | `(&self, RewardDistribution)` | Store epoch rewards. |
| `get_rewards()` | `(&self, u64) -> Option<RewardDistribution>` | Query epoch rewards. |
| `get_epoch_info()` | `(&self, u64) -> Option<EpochInfo>` | Query current epoch info. |
| `get_epoch_summary()` | `(&self, u64) -> Option<EpochSummary>` | Query historical summary. |
| `recent_summaries()` | `(&self, usize) -> Vec<EpochSummary>` | Recent completed summaries. |
| `total_stats()` | `(&self) -> EpochStats` | Aggregate stats. |

### 12.8 EpochCheckpointData Methods

| Method | Signature | Description |
|--------|-----------|-------------|
| `from_checkpoint()` | `(Bytes32, &Checkpoint) -> Self` | Construct from checkpoint + network ID. |
| `hash()` | `(&self) -> Bytes32` | SHA-256 of all verification fields. |
| `test_coin_id()` | `(&self) -> Bytes32` | Derive L1 verification coin ID. |

### 12.9 Serialization

| Function | Input | Output |
|----------|-------|--------|
| `EpochInfo::to_bytes()` / `from_bytes()` | `&self` / `&[u8]` | `Vec<u8>` / `Result<Self, EpochError>` |
| `EpochSummary::to_bytes()` / `from_bytes()` | `&self` / `&[u8]` | `Vec<u8>` / `Result<Self, EpochError>` |
| `EpochCheckpointData::to_bytes()` / `from_bytes()` | `&self` / `&[u8]` | `Vec<u8>` / `Result<Self, EpochError>` |
| `RewardDistribution::to_bytes()` / `from_bytes()` | `&self` / `&[u8]` | `Vec<u8>` / `Result<Self, EpochError>` |
| `CheckpointCompetition::to_bytes()` / `from_bytes()` | `&self` / `&[u8]` | `Vec<u8>` / `Result<Self, EpochError>` |
| `EpochBlockLink::to_bytes()` / `from_bytes()` | `&self` / `&[u8]` | `Vec<u8>` / `Result<Self, EpochError>` |

## 13. Crate Boundary

### 13.1 What This Crate Owns

| Concern | Owned by `dig-epoch` | Chia crates used |
|---------|---------------------|-----------------|
| Epoch lifecycle types (`EpochInfo`, `EpochSummary`, `DfspCloseSnapshot`) | Yes | `chia-protocol` (`Bytes32`) |
| Epoch phase state machine (`EpochPhase`, `PhaseTransition`) | Yes | — |
| Epoch management (`EpochManager`) | Yes | `dig-block` (`Checkpoint`, `CheckpointSubmission`), `parking_lot` (`RwLock`) |
| Height–epoch arithmetic (10 pure functions) | Yes | — |
| Checkpoint competition lifecycle (`CheckpointCompetition`, `CompetitionStatus`) | Yes | `dig-block` (`CheckpointSubmission`), `chia-bls` (`aggregate_verify`) |
| Epoch block root computation (`compute_epoch_block_root`) | Yes | `chia-sdk-types` (`MerkleTree`) — used directly, not custom |
| Epoch block inclusion proofs (`epoch_block_inclusion_proof`) | Yes | `chia-sdk-types` (`MerkleTree`, `MerkleProof`) — used directly |
| Withdrawal root computation (`compute_epoch_withdrawals_root`) | Yes | `chia-consensus` (`compute_merkle_set_root`) — used directly |
| Checkpoint data verification (`EpochCheckpointData`) | Yes | `chia-sha2` (`Sha256`) |
| Checkpoint signing material (`EpochCheckpointSignMaterial`) | Yes | `dig-block` (`Checkpoint`, `L2BlockHeader`), `chia-sdk-types` (`MerkleTree`), `chia-sha2`, `chia-sdk-signer` (`AggSigConstants`) |
| Aggregate signature construction | Yes | `chia-bls` (`aggregate`, `aggregate_verify`, `Signature`, `PublicKey`) |
| Reward economics (block rewards, halving, fee splits, epoch distribution) | Yes | — |
| Reward distribution type (`RewardDistribution`) | Yes | — |
| Block continuity proof (`EpochBlockLink`) | Yes | `chia-protocol` (`Bytes32`) |
| DFSP epoch-boundary types (9 types in §3.14) | Yes | `chia-protocol` (`Bytes32`) |
| DFSP epoch-boundary root computation (commitment digest, namespace rollup) | Yes | `chia-sha2` (`Sha256`), `dig-block` (`L2BlockHeader`) |
| DFSP activation control | Yes | — |
| Epoch constants (§2 — geometry, phases, rewards, DFSP, thresholds, slashing) | Yes | `chia-sdk-types` (`MAINNET_CONSTANTS`) for reference values |
| Epoch error types (`EpochError`, `CheckpointCompetitionError`) | Yes | `thiserror` |
| Epoch event types (`EpochEvent`) | Yes | `dig-block` (`Checkpoint`) |
| Serialization of all epoch types | Yes | `bincode`, `serde` |

### 13.2 What This Crate Does NOT Own

| Concern | Owned by | Notes |
|---------|----------|-------|
| Block type definitions (`L2BlockHeader`, `L2Block`, `AttestedBlock`) | `dig-block` | `dig-epoch` reads block headers for DFSP root extraction |
| Checkpoint and CheckpointSubmission type definitions | `dig-block` | `dig-epoch` manages competitions over these types |
| Checkpoint building (`CheckpointBuilder`) | `dig-block` | — |
| Block production and validation pipeline | `dig-block` | — |
| CLVM execution engine | `dig-clvm` | — |
| Global CoinSet state (database, rollback, queries) | `dig-coinstore` | — |
| Transaction pool (fee prioritization, conflict detection) | `dig-mempool` | — |
| Networking (block gossip, peer sync, checkpoint relay) | `dig-gossip` | — |
| Network-level constants (genesis challenge, network ID) | `dig-constants` | `dig-epoch` receives `network_id` as a constructor parameter |
| Merkle tree implementation | `chia-sdk-types` crate (Chia) | `MerkleTree` / `MerkleProof` — used directly, not redefined |
| Merkle set root implementation | `chia-consensus` crate (Chia) | `compute_merkle_set_root()` — used directly, not redefined |
| BLS signature aggregation and verification | `chia-bls` crate (Chia) | `aggregate()`, `aggregate_verify()` — used directly, not wrapped |
| SHA-256 hashing | `chia-sha2` crate (Chia) | `Sha256` — used directly, not a generic sha2 crate |
| Aggregate signature domain separation | `chia-sdk-signer` crate (Chia) | `AggSigConstants` — used directly for domain bytes |
| Tree hashing (CLVM semantics) | `clvm-utils` crate (Chia) | `tree_hash()`, `TreeHash` — used directly |
| Validator set management (registration, stake tracking, activation) | Consensus layer | Validator stakes are injected into checkpoint scoring by the caller |
| L1 coin interactions (puzzle spends, singleton management) | L1 driver | Uses `EpochBlockLink` for chain continuity proofs |
| Block storage and chain indexing | Chain store | — |
| DFSP registry storage (LMDB/RocksDB Merkle trees) | DFSP storage layer | `dig-epoch` defines boundary types; storage manages the actual trees |

### 13.3 Dependency Direction

```
dig-epoch  (this crate — epoch lifecycle, management, economics)
    │
    │  ┌─── DIG ecosystem ──────────────────────────────────────────────────┐
    ├──► dig-block         (Checkpoint, CheckpointSubmission, SignerBitmap,
    │                       L2BlockHeader — read-only for DFSP root extraction)
    ├──► dig-constants     (NetworkConstants, network ID, genesis challenge)
    │  └────────────────────────────────────────────────────────────────────┘
    │
    │  ┌─── Chia ecosystem (used directly for types, Merkle, BLS, hashing) ┐
    ├──► chia-protocol     (Bytes32)
    ├──► chia-bls          (Signature, PublicKey, aggregate, aggregate_verify)
    ├──► chia-consensus    (compute_merkle_set_root, validate_merkle_proof)
    ├──► chia-sdk-types    (MerkleTree, MerkleProof, MAINNET_CONSTANTS)
    ├──► chia-sdk-signer   (AggSigConstants — domain separation for signing digest)
    ├──► chia-sha2         (Sha256 — checkpoint data hash, operations digest,
    │                       commitment digest, all non-Merkle hashing)
    ├──► clvm-utils        (tree_hash, TreeHash — CLVM tree hashing semantics)
    │  └───────────────────────────────────────────────────────────────────┘
    │
    ├──► bincode           (epoch type serialization)
    ├──► serde             (derive Serialize/Deserialize)
    ├──► thiserror         (error derivation)
    └──► parking_lot       (RwLock for EpochManager interior mutability)

    chia-sdk-types  (transitive deps, NOT direct deps of dig-epoch)
        ├──► chia-protocol     (Bytes32 — leaf type)
        └──► chia-sha2         (Sha256 — Merkle hashing)

    chia-bls  (transitive deps)
        └──► blst              (BLS12-381 pairing library)

Downstream consumers:
    L2 driver      ──► dig-epoch  (EpochManager, phase tracking, advance_epoch)
    block proposer ──► dig-epoch  (epoch_for_block_height, is_checkpoint_class_block,
                                   block_reward_at_height, total_block_reward)
    checkpoint mgr ──► dig-epoch  (start_checkpoint_competition, submit_checkpoint,
                                   finalize_competition, compute_epoch_block_root,
                                   epoch_block_inclusion_proof,
                                   epoch_checkpoint_sign_material_from_l2_blocks,
                                   stored_checkpoint_from_epoch_sign_material_with_aggregate_v1)
    DFSP subsystem ──► dig-epoch  (DFSP epoch-boundary types, root computation,
                                   DfspCloseSnapshot → set_current_epoch_dfsp_close_snapshot)
    consensus      ──► dig-epoch  (phase queries, reward distribution, epoch events)
    full-node RPC  ──► dig-epoch  (EpochInfo, EpochSummary, EpochStats for API responses)
    light client   ──► dig-epoch  (epoch_block_inclusion_proof, MerkleProof verification,
                                   compute_epoch_withdrawals_root)
    dig-block      ──  (no dependency — dig-block does NOT depend on dig-epoch)
```

**Note:** The dependency is strictly one-directional: `dig-epoch` → `dig-block`, never `dig-block` → `dig-epoch`. `dig-block` defines the checkpoint data types; `dig-epoch` manages their lifecycle. This prevents circular dependencies and keeps `dig-block` as a leaf crate in the dependency graph.

## 14. Testing Strategy

### 14.1 Unit Tests

| Category | Tests |
|----------|-------|
| **EpochPhase** | `index()` returns 0–3 in order. `next()` / `previous()` chain correctly and terminate with `None`. `allows_block_production()` true only for `BlockProduction`. `allows_checkpoint_submission()` true only for `Checkpoint`. `allows_finalization()` true only for `Finalization`. All return `false` for `Complete`. `name()` returns expected strings. `Display` trait matches `name()`. |
| **PhaseTransition** | `new()` stores all fields correctly. |
| **EpochInfo construction** | `new()` initializes with `BlockProduction` phase, zeroed counters (`blocks_produced`, `total_fees`, `total_transactions`), `EMPTY_ROOT` for all four DFSP roots, `None` checkpoint, `dfsp_issuance_total = 0`, `active_cid_count = 0`, `active_node_count = 0`. `end_l1_height` = `start_l1_height + EPOCH_L1_BLOCKS`. |
| **EpochInfo phase calculation** | `calculate_phase()` returns `BlockProduction` at 0–49% L1 progress, `Checkpoint` at 50–74%, `Finalization` at 75–99%, `Complete` at 100%+. Boundary values tested: exactly 50%, exactly 75%, exactly 100%. |
| **EpochInfo helpers** | `target_blocks()` returns `BLOCKS_PER_EPOCH`. `can_produce_blocks()` / `can_submit_checkpoint()` / `is_complete()` match phase. `progress_percentage()` returns 0 at epoch start, 100 at epoch end. |
| **EpochInfo mutators** | `record_block(fees, tx_count)` increments all three counters correctly. Multiple calls accumulate. `set_checkpoint()` sets checkpoint and `is_finalized()` returns `true`. |
| **EpochSummary** | Created from `EpochInfo` with correct field mapping. `finalized` = `checkpoint.is_some()`. `checkpoint_hash` = `checkpoint.map(\|c\| c.hash())`. All seven DFSP fields preserved. |
| **DfspCloseSnapshot** | All seven fields round-trip correctly through `set_current_epoch_dfsp_close_snapshot()` and appear in the archived `EpochSummary`. |
| **EpochEvent** | All four variants constructable with expected fields. Pattern matching works. |
| **EpochStats** | Default is all zeros. |
| **CompetitionStatus transitions** | `Pending` → `Collecting` via `start()`. `Collecting` → `WinnerSelected` via `finalize()`. `WinnerSelected` → `Finalized` via `confirm_finalization()`. Any → `Failed` via `fail()`. `Finalized` rejects further transitions. |
| **CheckpointCompetition** | `new(epoch)` creates `Pending` status with empty submissions. `start()` transitions to `Collecting`. `add_submission()` appends and returns `true` on new leader. `winner()` returns highest-scoring. `finalize()` locks winner. `submission_count()` accurate. `is_finalized()` correct. |
| **RewardDistribution** | Five shares sum to `total_reward`. Fee split: `proposer_fee_share + burned_fees == total_fees`. |

### 14.2 Height–Epoch Arithmetic Tests

| Category | Tests |
|----------|-------|
| **epoch_for_block_height** | h=1 → 0, h=32 → 0, h=33 → 1, h=64 → 1, h=65 → 2. Large heights (h=1,000,000) return correct epoch. |
| **first_height_in_epoch** | e=0 → 1, e=1 → 33, e=2 → 65. Inverse relationship: `epoch_for_block_height(first_height_in_epoch(e)) == e` for all e. |
| **epoch_checkpoint_height** | e=0 → 32, e=1 → 64, e=2 → 96. `epoch_for_block_height(epoch_checkpoint_height(e)) == e` for all e. |
| **is_genesis_checkpoint_block** | True only at h=1. False at h=0, h=2, h=32. |
| **is_epoch_checkpoint_block** | True at h=32, 64, 96. False at h=1, 33, 65. |
| **is_checkpoint_class_block** | True at h=1, 32, 64. False at h=2, 33, 65. |
| **is_first_block_after_epoch_checkpoint** | True at h=33, 65, 97. False at h=1, 32, 34, 64. |
| **ensure_checkpoint_block_empty** | Passes when all counts are zero at checkpoint height. Rejects non-zero bundle count, non-zero cost, non-zero fees at checkpoint heights. Passes at non-checkpoint heights regardless of counts. |
| **last_committed_height_in_epoch** | Caps at checkpoint height when tip exceeds it. Returns tip when tip is within epoch range. |
| **l1_range_for_epoch** | Genesis=100, e=0 → (100, 132). e=1 → (132, 164). e=2 → (164, 196). |
| **Round-trip identity** | `first_height_in_epoch(epoch_for_block_height(h))` ≤ h for all h ≥ 1. `epoch_for_block_height(epoch_checkpoint_height(e)) == e` for all e ≥ 0. |

### 14.3 Phase Calculation Tests

| Category | Tests |
|----------|-------|
| **l1_progress_phase_for_network_epoch** | Genesis L1=100, epoch=0: L1=100 → `BlockProduction` (0%), L1=115 → `BlockProduction` (49%), L1=116 → `Checkpoint` (50%), L1=123 → `Checkpoint` (74%), L1=124 → `Finalization` (75%), L1=131 → `Finalization` (99%), L1=132 → `Complete` (100%). |
| **Multi-epoch phases** | Epoch 1 starts at L1=132: L1=132 → `BlockProduction`, L1=148 → `Checkpoint`, L1=156 → `Finalization`, L1=164 → `Complete`. |
| **Edge cases** | L1 height before epoch start (saturating_sub to 0) returns `BlockProduction`. Very large L1 height returns `Complete`. |

### 14.4 Reward Economics Tests

| Category | Tests |
|----------|-------|
| **block_reward_at_height** | h=0 → `INITIAL_BLOCK_REWARD` (320B). h=`HALVING_INTERVAL_BLOCKS` → 160B. h=`2×HALVING_INTERVAL_BLOCKS` → 80B. h=`3×HALVING_INTERVAL_BLOCKS` → 40B. h=`4×HALVING_INTERVAL_BLOCKS` → `TAIL_BLOCK_REWARD` (20B). h=`10×HALVING_INTERVAL_BLOCKS` → 20B (stays at tail). |
| **total_block_reward** | Without bonus: equals `block_reward_at_height()`. With bonus: equals `block_reward_at_height() + EPOCH_FIRST_BLOCK_BONUS`. |
| **proposer_fee_share** | 1000 fees → 500 proposer. 0 fees → 0. Odd fee (1001) → 500 proposer (integer division). |
| **burned_fee_remainder** | 1000 fees → 500 burned. 1001 fees → 501 burned (remainder goes to burn). |
| **compute_reward_distribution** | Five shares sum to `total_reward`. Each share matches `total_reward × SHARE / 100`. Fee split is correct. Epoch number propagated. |

### 14.5 EpochManager Tests

| Category | Tests |
|----------|-------|
| **Construction** | `new()` creates epoch 0 with correct genesis parameters. `current_epoch()` returns 0. `current_phase()` returns `BlockProduction`. `genesis_l1_height()` and `network_id()` match constructor args. |
| **Phase tracking** | `update_phase()` returns `Some(PhaseTransition)` when phase changes, `None` when unchanged. Sequential L1 height advances produce expected phase sequence: `BlockProduction` → `Checkpoint` → `Finalization` → `Complete`. |
| **Block recording** | `record_block()` increments counters. Multiple calls accumulate correctly. `set_current_epoch_chain_totals()` overwrites (does not accumulate). |
| **Competition lifecycle** | `start_checkpoint_competition()` creates competition in `Collecting` state. `submit_checkpoint()` accepts valid submission and returns `true` for new leader. Higher-score submission becomes leader; lower-score does not. `finalize_competition()` returns winning checkpoint. Double-finalize returns error. |
| **Epoch advancement** | `advance_epoch()` archives current epoch as `EpochSummary`, creates epoch N+1. History map grows. New epoch has correct start heights, zeroed counters, and provided state root. `current_epoch()` returns N+1. |
| **DFSP close** | `set_current_epoch_dfsp_close_snapshot()` updates all seven fields on current `EpochInfo`. Values survive `advance_epoch()` into the archived `EpochSummary`. |
| **Reward storage** | `store_rewards()` then `get_rewards()` returns same distribution. `get_rewards()` for unstored epoch returns `None`. |
| **Query API** | `get_epoch_info()` returns `Some` for current epoch, `None` for historical. `get_epoch_summary()` returns `Some` for archived epochs, `None` for current. `recent_summaries(n)` returns up to `n` summaries, newest first. `total_stats()` reflects all recorded blocks, fees, transactions across all epochs. |
| **Multi-epoch scenario** | Create manager, produce blocks, advance through 3 epochs. Verify history has 3 summaries. Verify `total_stats()` accumulates correctly. |

### 14.6 Verification Tests

| Category | Tests |
|----------|-------|
| **compute_epoch_block_root** | Empty list → `EMPTY_ROOT`. Single hash → leaf hash. Multiple hashes → deterministic Merkle root. Same hashes in different order → different root. Consistent across repeated calls. **Chia parity:** result matches `chia_sdk_types::MerkleTree::new(&hashes).root()` for all test vectors — this function is a thin wrapper, not a reimplementation. |
| **epoch_block_inclusion_proof** | Proof for valid index succeeds. Proof for out-of-bounds index returns `None`. Generated `MerkleProof` verifies against `compute_epoch_block_root()`. Invalid proof (tampered path or proof vec) fails verification. **Chia parity:** proof format matches `chia_sdk_types::MerkleProof` exactly. |
| **compute_epoch_withdrawals_root** | Empty list → `EMPTY_ROOT`. Single hash → correct root. Multiple hashes → deterministic root. Same hashes in different order → same root (order-independent). **Chia parity:** result matches `chia_consensus::compute_merkle_set_root()` — same function used by `dig-block` for additions/removals roots. |
| **EpochCheckpointData** | `from_checkpoint()` extracts correct fields. `hash()` is deterministic (same inputs → same hash). Different epochs produce different hashes. Different networks produce different hashes. `test_coin_id()` is deterministic and differs from `hash()`. |
| **EpochCheckpointSignMaterial** | `epoch_checkpoint_sign_material_from_l2_blocks()` computes correct block root from headers via `MerkleTree`. Score = `stake_pct × block_count`. Signing digest uses `AggSigConstants` for domain separation. Signing digest is deterministic. Empty block list returns error. |
| **Aggregate signature construction** | `stored_checkpoint_from_epoch_sign_material_with_aggregate_v1()` combines per-validator signatures via `chia_bls::aggregate()`. Resulting aggregate signature verifies via `chia_bls::aggregate_verify()` against all signer pubkeys and the signing digest. Invalid individual signature causes aggregate verification to fail. |

### 14.7 DFSP Epoch-Boundary Tests

| Category | Tests |
|----------|-------|
| **Burn policy** | `protocol_dfsp_epoch_burn_policy()` returns expected defaults. Schedule resolution picks correct entry for height (latest entry where `start_height <= height`). Empty schedule falls back to protocol default. |
| **Operations digest** | Deterministic: same inputs → same digest. Different epochs → different digests. Different plans → different digests. |
| **Commitment digest** | Deterministic: same roots + operations_digest → same commitment. Different roots → different commitment. |
| **Namespace rollup** | Empty blocks → `EMPTY_ROOT`. Single block with namespace root → that root. Multiple blocks → cumulative rollup. Respects `last_committed_height_in_epoch()` cap. |
| **Tail roots extraction** | Extracts four roots from last DFSP-active block in the epoch. Returns error if no blocks have DFSP roots. Ignores blocks before DFSP activation. |
| **DFSP activation** | `is_dfsp_active_at_height(h)` false when `h < DFSP_ACTIVATION_HEIGHT`, true when `h >= DFSP_ACTIVATION_HEIGHT`. Override via `set_dfsp_activation_height_override()` works. |

### 14.8 Error Type Tests

| Category | Tests |
|----------|-------|
| **EpochError Display** | All variants produce meaningful error messages. |
| **CheckpointCompetitionError Display** | All variants produce meaningful error messages. |
| **EpochError::Competition** | `From<CheckpointCompetitionError>` conversion works. |

### 14.9 Serialization Tests

| Category | Tests |
|----------|-------|
| **Round-trip** | All six serializable types survive `from_bytes(to_bytes(x)) == x`. |
| **EpochInfo** | Preserves all fields including DFSP roots, checkpoint (`Some` and `None`), and all counters. |
| **CheckpointCompetition** | Preserves submissions, status (all variants), and `current_winner`. |
| **EpochSummary** | Preserves all fields including DFSP fields and optional `checkpoint_hash`. |

### 14.10 Property Tests

| Property | Description |
|----------|-------------|
| **Height–epoch round-trip** | For any h ≥ 1: `epoch_for_block_height(h)` is a valid epoch, and `first_height_in_epoch(epoch_for_block_height(h))` ≤ h ≤ `epoch_checkpoint_height(epoch_for_block_height(h))`. |
| **Phase monotonicity** | For a fixed epoch and genesis height, increasing L1 heights never decrease the phase index: if `h1 < h2`, then `phase(h1).index() <= phase(h2).index()`. |
| **Reward non-negative** | `block_reward_at_height(h) > 0` for all h. `total_block_reward(h, _) >= block_reward_at_height(h)`. |
| **Reward non-increasing** | `block_reward_at_height(h1) >= block_reward_at_height(h2)` for all `h1 < h2` (halving only decreases rewards). |
| **Competition score ordering** | After multiple submissions to a `CheckpointCompetition`, `winner().score >= all other submission scores`. |
| **Serialization round-trip** | For all epoch types: `from_bytes(to_bytes(x)) == x`. |
| **Phase determinism** | `l1_progress_phase_for_network_epoch(g, e, h)` returns the same result for identical inputs, regardless of call count or ordering. |
| **Checkpoint height is last in epoch** | `epoch_checkpoint_height(e) == first_height_in_epoch(e) + BLOCKS_PER_EPOCH - 1` for all e ≥ 0. |
| **Fee conservation** | `proposer_fee_share(f) + burned_fee_remainder(f) == f` for all f. |

### 14.11 Integration Tests

| Test | Description |
|------|-------------|
| **Full epoch lifecycle** | Construct `EpochManager`, advance L1 through all four phases, record blocks, submit checkpoints, finalize competition, apply DFSP close, advance epoch. Verify all state transitions and archived summary. |
| **Multi-epoch advancement** | Run 5 complete epochs. Verify `total_stats()`, `recent_summaries(3)`, and individual `get_epoch_summary()` calls. |
| **Competition with multiple submissions** | Submit 5 checkpoints with different scores. Verify winner has highest score. Verify `submission_count() == 5`. Verify `finalize()` locks the correct winner. |
| **DFSP epoch-boundary end-to-end** | Build epoch blocks with DFSP roots, extract tail roots, compute namespace rollup, compute operations digest and commitment digest. Verify all digests are deterministic and differ across epochs. |
| **Reward distribution across halvings** | Compute rewards at heights spanning multiple halving intervals. Verify the halving curve, tail emission floor, and epoch-opening bonus. |
| **Genesis epoch** | Create manager at epoch 0, verify `first_height_in_epoch(0) == 1`, `epoch_checkpoint_height(0) == 32`, and `is_genesis_checkpoint_block(1) == true`. |
| **Phase-gated operations** | Verify that operations respect phase: blocks only during `BlockProduction`, checkpoints only during `Checkpoint`, finalization only during `Finalization`. |
| **Chia Merkle parity** | Construct 32 random block hashes, compute `compute_epoch_block_root()`, independently compute `chia_sdk_types::MerkleTree::new(&hashes).root()`. Verify identical results. Generate inclusion proof via `epoch_block_inclusion_proof()`, verify it against the root using `MerkleProof` verification. |
| **Chia BLS aggregate parity** | Generate per-validator BLS signatures over a signing digest using `chia_bls::sign()`. Construct aggregate via `stored_checkpoint_from_epoch_sign_material_with_aggregate_v1()`. Verify the aggregate via `chia_bls::aggregate_verify()` independently. |
| **Withdrawal root Chia parity** | Construct withdrawal request hashes, compute `compute_epoch_withdrawals_root()`, independently compute `chia_consensus::compute_merkle_set_root()`. Verify identical results. Verify order-independence: shuffled inputs produce the same root. |
