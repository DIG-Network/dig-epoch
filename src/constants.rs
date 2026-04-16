//! # `constants` — compile-time epoch constants
//!
//! **Introduced by:** `STR-002` — Module hierarchy (SPEC §13).
//!
//! **Future owners:** This module is the landing zone for Phase 1 of
//! `IMPLEMENTATION_ORDER.md`:
//!
//! - [`CON-001`](../../docs/requirements/domains/constants/specs/CON-001.md)
//!   — epoch geometry (`BLOCKS_PER_EPOCH`, `EPOCH_L1_BLOCKS`, `GENESIS_HEIGHT`)
//!   **[LANDED]**
//! - [`CON-002`](../../docs/requirements/domains/constants/specs/CON-002.md)
//!   — phase boundary thresholds (50 % / 75 % / 100 %)
//! - [`CON-003`](../../docs/requirements/domains/constants/specs/CON-003.md)
//!   — reward economics (`MOJOS_PER_L2`, `INITIAL_BLOCK_REWARD`, halvings,
//!   tail emission, epoch-first-block bonus)
//! - [`CON-004`](../../docs/requirements/domains/constants/specs/CON-004.md)
//!   — 5-role fee / reward split percentages
//! - [`CON-005`](../../docs/requirements/domains/constants/specs/CON-005.md)
//!   — DFSP, consensus, slashing, and withdrawal constants
//! - [`CON-006`](../../docs/requirements/domains/constants/specs/CON-006.md)
//!   — sentinel constants (`EMPTY_ROOT`)
//!
//! **Spec reference:**
//! [`SPEC.md` §13](../../docs/resources/SPEC.md) — canonical module list.
//!
//! ## Content rule
//!
//! Per the STR-002 responsibility table, this module holds **only**
//! `pub const` declarations. It must never grow a function, enum, struct,
//! or `use` import that has runtime semantics; cross-cutting concerns such
//! as the genesis challenge come from `dig-constants` and are re-exported
//! through `lib.rs`, not re-defined here.
//!
//! ## Status
//!
//! - **CON-001 (epoch geometry)** has landed — see the three `pub const`
//!   items below.
//! - CON-002 … CON-006 are still pending; their constants will be added
//!   alongside this triple by their respective requirement commits.
//!
//! The [`STR_002_MODULE_PRESENT`] sentinel from STR-002 is kept in place;
//! removing it is explicitly STR-002's cleanup task, not CON-001's, and
//! pulling it out early would scope-creep across requirements.

use chia_protocol::Bytes32;

/// Sentinel marker proving the module exists, is declared in `lib.rs`, and
/// is reachable from an external crate at its canonical path.
///
/// See the STR-002 integration test at
/// [`tests/crate_structure/str_002_test.rs`](../../tests/crate_structure/str_002_test.rs).
///
/// The value is a zero-sized `()` constant; its only purpose is to let
/// `use dig_epoch::constants::STR_002_MODULE_PRESENT` succeed. No production
/// code should depend on it — later requirements are free to add real
/// `pub const` items alongside it without coordination.
#[doc(hidden)]
pub const STR_002_MODULE_PRESENT: () = ();

// -----------------------------------------------------------------------------
// CON-001 — Epoch geometry
// -----------------------------------------------------------------------------
//
// Spec refs:
//   * Normative    : docs/requirements/domains/constants/NORMATIVE.md#con-001
//   * Detailed spec: docs/requirements/domains/constants/specs/CON-001.md
//   * SPEC source  : docs/resources/SPEC.md §2.1
//   * Design basis : SPEC.md §1.3 Design Decisions #2 (epoch sizing) and #3
//                    (height-1 as genesis)
//
// Values and types are **non-negotiable**. Changing any of them alters the
// consensus-critical epoch geometry shared by every validator: all height-
// to-epoch arithmetic (HEA-*), phase progression (PHS-*), reward emissions
// (REW-*), and DFSP epoch boundaries (DFS-*) derive from this triple.
//
// Types in particular are load-bearing:
//   * `u64` for `BLOCKS_PER_EPOCH` and `GENESIS_HEIGHT` — L2 heights are
//     `u64`, so these constants slot into height arithmetic (e.g.
//     `(height - GENESIS_HEIGHT) / BLOCKS_PER_EPOCH`) without casts.
//   * `u32` for `EPOCH_L1_BLOCKS` — Chia L1 block heights are `u32`
//     (chia-protocol / chia-consensus use 32-bit L1 heights), so phase
//     progression computed as `(l1_now - l1_start) * 100 / EPOCH_L1_BLOCKS`
//     stays inside `u32` without casts.
//
// Downstream consumers (non-exhaustive):
//   * HEA-001 `epoch_for_block_height` — `(h - GENESIS_HEIGHT) / BLOCKS_PER_EPOCH`
//   * HEA-002 `epoch_checkpoint_height` — `(e + 1) * BLOCKS_PER_EPOCH`
//   * HEA-003 `is_genesis_checkpoint_block` — compares against `GENESIS_HEIGHT`
//   * HEA-004 `l1_range_for_epoch` — multiplies by `EPOCH_L1_BLOCKS`
//   * PHS-001 `l1_progress_phase_for_network_epoch` — uses `EPOCH_L1_BLOCKS`
//     as the denominator of phase-progress percentage
//   * REW-002 `total_block_reward` — uses `BLOCKS_PER_EPOCH` to identify the
//     epoch-opening block that receives `EPOCH_FIRST_BLOCK_BONUS`
//   * DFS-005 `is_dfsp_active_at_height` — heights are interpreted in the
//     `BLOCKS_PER_EPOCH` geometry established here
// -----------------------------------------------------------------------------

/// L2 blocks per epoch — each epoch spans exactly this many committed heights.
///
/// **Value (locked by [SPEC §2.1](../../docs/resources/SPEC.md) and
/// [CON-001](../../docs/requirements/domains/constants/specs/CON-001.md)):** `32`.
///
/// Epoch `e` contains heights `[e * BLOCKS_PER_EPOCH + 1, (e + 1) * BLOCKS_PER_EPOCH]`
/// (half-open at the low end so that epoch 0 starts at [`GENESIS_HEIGHT`] and
/// closed at the high end so that the epoch's last block is the checkpoint).
///
/// # Rationale
///
/// 32 blocks × 3 s wall-clock block time ≈ **96 seconds** per epoch — small
/// enough for fast finality, large enough to amortise checkpoint overhead.
/// See [SPEC.md §1.3 Design Decision #2](../../docs/resources/SPEC.md) for the
/// full sizing argument. The value is also aligned with
/// [`EPOCH_L1_BLOCKS`] so the two clocks (L2 chain and L1 phase window) tick
/// at commensurate rates.
///
/// # Why `u64`
///
/// L2 heights are `u64` throughout dig-epoch (SPEC §2.1). Declaring
/// `BLOCKS_PER_EPOCH` as `u64` lets callers write
/// `(height - GENESIS_HEIGHT) / BLOCKS_PER_EPOCH` without an `as u64` cast,
/// which both reads cleanly and avoids subtle truncation bugs. Any narrower
/// type would force defensive casts at every use site; any wider type (e.g.
/// `u128`) would unnecessarily poison arithmetic with a 128-bit upgrade.
///
/// # Consumers
///
/// This constant is read by the entire `height_epoch_arithmetic` domain
/// (HEA-001 … HEA-005), the `reward_economics` domain (REW-002 / REW-005 use
/// it to identify epoch boundaries for the first-block bonus and emission
/// tally), the `epoch_manager` domain (MGR-004 `advance_epoch` counts blocks
/// per epoch), and the `dfsp_processing` domain (DFS-005 activation height
/// is rounded to epoch boundaries). Hard-coding `32` at a call site is a
/// spec violation — callers MUST reference this symbol.
pub const BLOCKS_PER_EPOCH: u64 = 32;

/// L1 blocks per epoch window — the Chia L1 block count that maps to one
/// dig-epoch phase cycle.
///
/// **Value (locked by [SPEC §2.1](../../docs/resources/SPEC.md) and
/// [CON-001](../../docs/requirements/domains/constants/specs/CON-001.md)):** `32`.
///
/// At Chia's 18-second L1 block cadence this is ≈ **576 seconds (~10 minutes)**
/// per window — the wall-clock envelope in which the four-phase epoch
/// lifecycle (`BlockProduction` → `Checkpoint` → `Finalization` → `Complete`)
/// runs. Phase progression is computed as `(l1_now - l1_start) * 100 /
/// EPOCH_L1_BLOCKS`, which is then compared against the CON-002 boundary
/// percentages (50 % / 75 % / 100 %).
///
/// # Rationale
///
/// 32 L1 blocks gives a comfortable window for block production, checkpoint
/// collection, and finalization without pinning the phase clock to
/// wall-clock time — which varies between nodes and drifts (SPEC §1.3
/// Decision #1). Anchoring to L1 height makes phase progression objective
/// and consensus-observable: every validator with the same L1 view computes
/// the same phase.
///
/// # Why `u32`
///
/// Chia L1 heights are `u32` (chia-protocol / chia-consensus use 32-bit
/// heights throughout the L1 ecosystem — see `Cargo.toml` for the version
/// pins that lock this). Declaring `EPOCH_L1_BLOCKS` as `u32` keeps the
/// phase-progress formula inside `u32` space without casts. A silent
/// promotion to `u64` would force every caller to cast and would be
/// incompatible with `chia-consensus` L1 height types.
///
/// # Consumers
///
/// Read by the `phase_state_machine` domain (PHS-001 … PHS-004) and by
/// HEA-004 `l1_range_for_epoch`, which multiplies `EPOCH_L1_BLOCKS` by an
/// epoch number to compute the L1 window for any epoch. Like
/// [`BLOCKS_PER_EPOCH`], this symbol MUST be referenced by name — hard-coded
/// `32` in phase or L1-range code is a spec violation.
pub const EPOCH_L1_BLOCKS: u32 = 32;

/// First L2 block height — the genesis block.
///
/// **Value (locked by [SPEC §2.1](../../docs/resources/SPEC.md) and
/// [CON-001](../../docs/requirements/domains/constants/specs/CON-001.md)):** `1`.
///
/// The DIG L2 chain begins at height **1**, not height 0. The block at
/// height 1 is the **epoch-0 genesis checkpoint** and is the sole occupant
/// of the special genesis state in HEA-003's
/// `is_genesis_checkpoint_block(h)` predicate.
///
/// # Rationale
///
/// Reserving `0` as "no blocks" removes an ambiguity that plagues
/// zero-indexed chains: `height == 0` can mean either "the first block" or
/// "the chain is empty", depending on the author. By starting at height 1
/// (SPEC §1.3 Design Decision #3 and [`start.md` Hard Requirement 11](../../docs/prompt/start.md)),
/// dig-epoch lets `u64` default-zero semantics serve as an explicit
/// "uninitialised" marker (e.g. a freshly-constructed `EpochInfo` whose
/// `current_height` is `0` is unambiguously pre-genesis).
///
/// This also lets the height-to-epoch formula stay simple:
/// `epoch = (height - GENESIS_HEIGHT) / BLOCKS_PER_EPOCH`, which places both
/// genesis (height 1) and the epoch-0 checkpoint (height 32) in epoch 0 by
/// arithmetic — no special cases needed.
///
/// # Why `u64`
///
/// Matches the `u64` type used for every L2 block height in dig-epoch
/// (SPEC §2.1). Using a matching type lets
/// `(height - GENESIS_HEIGHT) / BLOCKS_PER_EPOCH` participate in native
/// `u64` arithmetic.
///
/// # Consumers
///
/// HEA-001 / HEA-003 / HEA-005 (height arithmetic and the genesis predicate),
/// MGR-001 / MGR-004 (the `EpochManager` initialises its head pointer at
/// `GENESIS_HEIGHT - 1` so the first `record_block` call advances cleanly
/// to genesis), and REW-002 (the epoch-opening bonus triggers when a
/// recorded height equals `e * BLOCKS_PER_EPOCH + GENESIS_HEIGHT`).
pub const GENESIS_HEIGHT: u64 = 1;

// -----------------------------------------------------------------------------
// CON-002 — Phase boundary percentages
// -----------------------------------------------------------------------------
//
// Spec refs:
//   * Normative    : docs/requirements/domains/constants/NORMATIVE.md#con-002
//   * SPEC source  : docs/resources/SPEC.md §2.2
//
// These three values define where each active phase ends within the
// EPOCH_L1_BLOCKS-wide L1 window. Phase progress is computed as:
//   `progress_pct = (l1_now - epoch_start_l1) * 100 / EPOCH_L1_BLOCKS`
// and compared against these thresholds in order:
//   0 % … 50 %  → BlockProduction phase
//   50 % … 75 % → Checkpoint phase
//   75 % … 100 %→ Finalization phase
//   >= 100 %    → Complete phase
//
// Types:
//   * `u32` — L1 heights are `u32` (Chia), so the progress formula stays
//     inside `u32` arithmetic without casts. The percentages are well within
//     [0, 100], so no overflow risk.
//
// Values are **non-negotiable** (SPEC §2.2). Changing any of them shifts
// phase windows for all validators and breaks consensus.
// -----------------------------------------------------------------------------

/// L1-progress percentage at which the BlockProduction phase ends.
///
/// **Value (locked by [SPEC §2.2]):** `50`.
///
/// When `(l1_now - epoch_start_l1) * 100 / EPOCH_L1_BLOCKS >= 50` the phase
/// transitions from `BlockProduction` to `Checkpoint`.
pub const PHASE_BLOCK_PRODUCTION_END_PCT: u32 = 50;

/// L1-progress percentage at which the Checkpoint submission phase ends.
///
/// **Value (locked by [SPEC §2.2]):** `75`.
///
/// When progress reaches 75% the phase transitions from `Checkpoint` to
/// `Finalization`.
pub const PHASE_CHECKPOINT_END_PCT: u32 = 75;

/// L1-progress percentage at which the Finalization phase ends.
///
/// **Value (locked by [SPEC §2.2]):** `100`.
///
/// When progress reaches 100% the phase transitions to `Complete`.
pub const PHASE_FINALIZATION_END_PCT: u32 = 100;

// -----------------------------------------------------------------------------
// CON-003 — Reward economics constants
// -----------------------------------------------------------------------------
//
// Spec refs:
//   * Normative    : docs/requirements/domains/constants/NORMATIVE.md#con-003
//   * Detailed spec: docs/requirements/domains/constants/specs/CON-003.md
//   * SPEC source  : docs/resources/SPEC.md §2.3
//
// These are **economic commitments** identical across all validators. They
// MUST NOT be runtime-configurable (SPEC §2.3).
// -----------------------------------------------------------------------------

/// 1 L2 token = 10^12 mojos.
pub const MOJOS_PER_L2: u64 = 1_000_000_000_000;

/// L2 block time in milliseconds.
pub const L2_BLOCK_TIME_MS: u64 = 3_000;

/// L2 blocks per 10-minute window (600_000 ms / 3_000 ms).
pub const L2_BLOCKS_PER_10_MIN: u64 = 200;

/// Initial emission rate: 64 L2 per 10 minutes.
pub const INITIAL_EMISSION_PER_10_MIN: u64 = 64 * MOJOS_PER_L2;

/// Tail emission rate: 4 L2 per 10 minutes.
pub const TAIL_EMISSION_PER_10_MIN: u64 = 4 * MOJOS_PER_L2;

/// Per-block reward before any halving (0.32 L2).
/// Derived: `INITIAL_EMISSION_PER_10_MIN / L2_BLOCKS_PER_10_MIN`.
pub const INITIAL_BLOCK_REWARD: u64 = 320_000_000_000;

/// Per-block reward at tail emission (0.02 L2).
/// Derived: `TAIL_EMISSION_PER_10_MIN / L2_BLOCKS_PER_10_MIN`.
pub const TAIL_BLOCK_REWARD: u64 = 20_000_000_000;

/// Halving interval: ~3 years of blocks at 3-second block time.
pub const HALVING_INTERVAL_BLOCKS: u64 = 94_608_000;

/// Number of halvings before switching to tail emission.
pub const HALVINGS_BEFORE_TAIL: u64 = 4;

/// Initial epoch reward (spec-declared; see CON-003.md).
pub const INITIAL_EPOCH_REWARD: u64 = 32_000_000_000_000;

/// Halving interval in epochs (spec-declared; see CON-003.md).
pub const HALVING_INTERVAL_EPOCHS: u64 = 315_576;

/// Minimum epoch reward (tail emission floor).
pub const MINIMUM_EPOCH_REWARD: u64 = 2_000_000_000_000;

/// Bonus reward for the first block after an epoch checkpoint.
pub const EPOCH_FIRST_BLOCK_BONUS: u64 = 100_000_000_000;

// -----------------------------------------------------------------------------
// CON-004 — Fee and reward distribution constants
// -----------------------------------------------------------------------------
//
// Spec refs:
//   * Normative  : docs/requirements/domains/constants/NORMATIVE.md#con-004
//   * SPEC source: docs/resources/SPEC.md §2.3
//
// Invariants (SPEC §2.3):
//   FEE_PROPOSER_SHARE_PCT + FEE_BURN_SHARE_PCT == 100
//   PROPOSER_REWARD_SHARE + ATTESTER_REWARD_SHARE + EF_SPAWNER_REWARD_SHARE
//     + SCORE_SUBMITTER_REWARD_SHARE + FINALIZER_REWARD_SHARE == 100
// -----------------------------------------------------------------------------

/// Proposer share of collected transaction fees (percentage).
pub const FEE_PROPOSER_SHARE_PCT: u64 = 50;

/// Burn share of collected transaction fees (percentage).
pub const FEE_BURN_SHARE_PCT: u64 = 50;

/// Proposer share of the epoch block reward.
pub const PROPOSER_REWARD_SHARE: u64 = 10;

/// Attester share of the epoch block reward.
pub const ATTESTER_REWARD_SHARE: u64 = 80;

/// EF spawner share of the epoch block reward.
pub const EF_SPAWNER_REWARD_SHARE: u64 = 3;

/// Score submitter share of the epoch block reward.
pub const SCORE_SUBMITTER_REWARD_SHARE: u64 = 4;

/// Finalizer share of the epoch block reward.
pub const FINALIZER_REWARD_SHARE: u64 = 3;

// -----------------------------------------------------------------------------
// CON-005 — DFSP, consensus, slashing, and withdrawal constants
// -----------------------------------------------------------------------------
//
// Spec refs:
//   * Normative    : docs/requirements/domains/constants/NORMATIVE.md#con-005
//   * Detailed spec: docs/requirements/domains/constants/specs/CON-005.md
//   * SPEC source  : docs/resources/SPEC.md §2.4-2.6
// -----------------------------------------------------------------------------

/// Wall-clock seconds per DFSP accounting epoch.
/// Derived: `(BLOCKS_PER_EPOCH * L2_BLOCK_TIME_MS) / 1_000`.
pub const DFSP_WALL_CLOCK_EPOCH_SECONDS: u64 = 96;

/// Network epochs a CID may remain in Grace state before expiring (30-day window).
pub const DFSP_GRACE_PERIOD_NETWORK_EPOCHS: u64 = 27_000;

/// Bootstrap genesis issuance subsidy per evaluated epoch (mojos). `u128` for
/// mojo-precision DFSP calculations.
pub const DFSP_GENESIS_ISSUANCE_SUBSIDY_MOJOS_V1: u128 = 0;

/// DFSP activation height — `u64::MAX` means disabled by default.
pub const DFSP_ACTIVATION_HEIGHT: u64 = u64::MAX;

/// Environment variable name used to override DFSP activation height.
pub const DIG_DFSP_ACTIVATION_HEIGHT_ENV: &str = "DIG_DFSP_ACTIVATION_HEIGHT";

/// Stake percentage required for soft finality.
pub const SOFT_FINALITY_THRESHOLD_PCT: u64 = 67;

/// Stake percentage required for a checkpoint to win the competition.
pub const HARD_FINALITY_THRESHOLD_PCT: u64 = 67;

/// Stake percentage required for a valid checkpoint submission.
pub const CHECKPOINT_THRESHOLD_PCT: u64 = 67;

/// Epochs to track for correlation penalty calculation. `u32` per SPEC §2.6.
pub const CORRELATION_WINDOW_EPOCHS: u32 = 36;

/// Maximum lookback for slashable offenses (in epochs).
pub const SLASH_LOOKBACK_EPOCHS: u64 = 1_000;

/// DFSP slashing evidence lookback — alias for [`SLASH_LOOKBACK_EPOCHS`].
pub const DFSP_SLASH_LOOKBACK_EPOCHS: u64 = SLASH_LOOKBACK_EPOCHS;

/// Epochs before a withdrawal completes.
pub const WITHDRAWAL_DELAY_EPOCHS: u64 = 50;

// -----------------------------------------------------------------------------
// CON-006 — Sentinel constants
// -----------------------------------------------------------------------------
//
// Spec refs:
//   * Normative    : docs/requirements/domains/constants/NORMATIVE.md#con-006
//   * Detailed spec: docs/requirements/domains/constants/specs/CON-006.md
//   * SPEC source  : docs/resources/SPEC.md §7.1, 7.3, 14.1
// -----------------------------------------------------------------------------

/// SHA-256 hash of the empty string (`e3b0c44…b855`).
///
/// Used as the default root for empty Merkle trees (VER-001, VER-003) and
/// as the initial value for all four DFSP roots in `EpochInfo::new()`.
pub const EMPTY_ROOT: Bytes32 = Bytes32::new([
    0xe3, 0xb0, 0xc4, 0x42, 0x98, 0xfc, 0x1c, 0x14, 0x9a, 0xfb, 0xf4, 0xc8, 0x99, 0x6f, 0xb9, 0x24,
    0x27, 0xae, 0x41, 0xe4, 0x64, 0x9b, 0x93, 0x4c, 0xa4, 0x95, 0x99, 0x1b, 0x78, 0x52, 0xb8, 0x55,
]);
