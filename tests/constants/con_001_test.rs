//! # CON-001 — Epoch Geometry Constants (Integration Test)
//!
//! **Requirement under test:** `CON-001 — Epoch geometry constants`
//!
//! **Spec (authoritative):**
//! [`docs/requirements/domains/constants/specs/CON-001.md`](../../docs/requirements/domains/constants/specs/CON-001.md)
//!
//! **Normative language:**
//! [`docs/requirements/domains/constants/NORMATIVE.md`](../../docs/requirements/domains/constants/NORMATIVE.md)
//!
//! **Verification matrix:**
//! [`docs/requirements/domains/constants/VERIFICATION.md`](../../docs/requirements/domains/constants/VERIFICATION.md)
//!
//! **Spec of spec:** [`docs/resources/SPEC.md` §2.1](../../docs/resources/SPEC.md)
//!
//! ## How this file proves CON-001 is satisfied
//!
//! CON-001 mandates that the `dig-epoch` crate declares three `pub const`
//! items in the `constants` module with the exact types and values required
//! by [`SPEC.md` §2.1](../../docs/resources/SPEC.md):
//!
//! | Constant             | Type  | Value |
//! |----------------------|-------|-------|
//! | `BLOCKS_PER_EPOCH`   | `u64` |  32   |
//! | `EPOCH_L1_BLOCKS`    | `u32` |  32   |
//! | `GENESIS_HEIGHT`     | `u64` |   1   |
//!
//! The values are non-negotiable: every downstream requirement in the
//! `height_epoch_arithmetic` (HEA-*), `phase_state_machine` (PHS-*),
//! `reward_economics` (REW-*), and `dfsp_processing` (DFS-*) domains
//! assumes these exact numbers. The types are equally load-bearing because
//! L2 heights are `u64` (so `BLOCKS_PER_EPOCH` must participate in `u64`
//! arithmetic without casting) and L1 heights are `u32` (so
//! `EPOCH_L1_BLOCKS` must participate in `u32` arithmetic without casting).
//!
//! Each `#[test]` below implements exactly one row of the Test Plan table
//! from [`specs/CON-001.md`](../../docs/requirements/domains/constants/specs/CON-001.md#verification)
//! (seven rows → seven tests). Integration tests compile as an external
//! crate, so every access below necessarily uses the fully-qualified
//! `dig_epoch::constants::*` path. STR-003 will later add convenience
//! re-exports at the crate root; until then these fully-qualified paths
//! also prove the module is wired through `lib.rs` at its canonical
//! location.
//!
//! ## Test Plan coverage map
//!
//! | Row                        | Test fn below                    |
//! |----------------------------|----------------------------------|
//! | `test_blocks_per_epoch`    | [`test_blocks_per_epoch`]        |
//! | `test_epoch_l1_blocks`     | [`test_epoch_l1_blocks`]         |
//! | `test_genesis_height`      | [`test_genesis_height`]          |
//! | `test_blocks_per_epoch_type` | [`test_blocks_per_epoch_type`] |
//! | `test_epoch_l1_blocks_type` | [`test_epoch_l1_blocks_type`]  |
//! | `test_epoch_0_range`       | [`test_epoch_0_range`]           |
//! | `test_epoch_1_range`       | [`test_epoch_1_range`]           |

// Fully-qualified imports from the crate under test. CON-001 does NOT add
// re-exports at the crate root (that is STR-003's job); consumers must
// reach into `dig_epoch::constants::*` by path. Proving that path compiles
// is itself part of the acceptance criteria in `specs/CON-001.md`.
use dig_epoch::constants::{BLOCKS_PER_EPOCH, EPOCH_L1_BLOCKS, GENESIS_HEIGHT};

/// Test Plan row 1: `test_blocks_per_epoch`.
///
/// Asserts `BLOCKS_PER_EPOCH == 32`. Passing this proves the
/// spec-mandated value from [`SPEC.md` §2.1](../../docs/resources/SPEC.md)
/// has been encoded correctly. Any drift (e.g. 64, 30, 100) would
/// invalidate every epoch range calculation (`[e*32+1, (e+1)*32]`) across
/// the HEA-* domain, so this assertion is a load-bearing guard against
/// accidental constant edits.
#[test]
fn test_blocks_per_epoch() {
    assert_eq!(
        BLOCKS_PER_EPOCH, 32,
        "BLOCKS_PER_EPOCH must equal 32 per SPEC.md §2.1 and NORMATIVE.md CON-001"
    );
}

/// Test Plan row 2: `test_epoch_l1_blocks`.
///
/// Asserts `EPOCH_L1_BLOCKS == 32`. Passing this proves the L1 phase
/// window is sized at 32 Chia L1 blocks (~10 minutes at 18s block time),
/// matching Design Decision #2 in [`SPEC.md` §1.3](../../docs/resources/SPEC.md).
/// PHS-* (phase state machine) computes phase progression as a percentage
/// of this window, so the value must be exactly 32 for `calculate_phase()`
/// to partition the window into the 50% / 75% / 100% boundaries declared
/// by CON-002.
#[test]
fn test_epoch_l1_blocks() {
    assert_eq!(
        EPOCH_L1_BLOCKS, 32,
        "EPOCH_L1_BLOCKS must equal 32 per SPEC.md §2.1 and NORMATIVE.md CON-001"
    );
}

/// Test Plan row 3: `test_genesis_height`.
///
/// Asserts `GENESIS_HEIGHT == 1`. Passing this proves Design Decision #3
/// from [`SPEC.md` §1.3](../../docs/resources/SPEC.md) — the L2 chain
/// begins at height 1, never height 0. HEA-003 (`is_genesis_checkpoint_block`)
/// and every reward-emission path that references "the genesis block"
/// compares against `GENESIS_HEIGHT`, so an off-by-one here would cascade
/// into reward miscomputation at epoch 0.
#[test]
fn test_genesis_height() {
    assert_eq!(
        GENESIS_HEIGHT, 1,
        "GENESIS_HEIGHT must equal 1 per SPEC.md §2.1 and Design Decision #3"
    );
}

/// Test Plan row 4: `test_blocks_per_epoch_type`.
///
/// Proves `BLOCKS_PER_EPOCH` is a `u64`, by compiling a `u64`-typed
/// arithmetic expression that would fail type inference if the constant
/// were any narrower (e.g. `u32`) or wider (e.g. `u128`) without a cast.
/// L2 heights in dig-epoch are `u64` (SPEC §2.1), so `BLOCKS_PER_EPOCH`
/// must slot into height arithmetic (`(height - 1) / BLOCKS_PER_EPOCH`)
/// without any `as u64` coercion. If a future edit silently changed the
/// type, this test would fail to compile — which, under TDD, is the
/// earliest possible feedback.
#[test]
fn test_blocks_per_epoch_type() {
    // Canonical height-to-epoch formula. The left operand is explicitly
    // typed `u64` — if `BLOCKS_PER_EPOCH` were not `u64`, this expression
    // would emit a "mismatched types" compiler error rather than running.
    let height: u64 = 97;
    let epoch: u64 = (height - GENESIS_HEIGHT) / BLOCKS_PER_EPOCH;
    assert_eq!(
        epoch, 3,
        "height 97 must map to epoch 3 via u64 arithmetic: (97 - 1) / 32 == 3"
    );

    // Secondary proof: binding to an explicit `u64` variable forces a
    // type-identity check at compile time. This is redundant with the
    // arithmetic above but makes the type contract explicit in the
    // source so a reader sees "BLOCKS_PER_EPOCH : u64" stated directly.
    let typed: u64 = BLOCKS_PER_EPOCH;
    assert_eq!(typed, 32);
}

/// Test Plan row 5: `test_epoch_l1_blocks_type`.
///
/// Proves `EPOCH_L1_BLOCKS` is a `u32`, by compiling a `u32`-typed
/// arithmetic expression. Chia L1 heights are `u32` (SPEC §1.2 dependency
/// table — `chia-protocol` / `chia-consensus` use 32-bit heights), and
/// PHS-001 computes phase progression as `(l1_now - l1_start) * 100 /
/// EPOCH_L1_BLOCKS` within `u32` space. A silent widening to `u64` would
/// force every caller to cast, defeating the point of the per-domain
/// integer type choice. A narrowing to `u16` would overflow the
/// percentage calculation.
#[test]
fn test_epoch_l1_blocks_type() {
    // Canonical phase-progress formula (simplified; PHS-001 owns the
    // full signature). Left operand explicitly typed `u32`.
    let l1_now: u32 = 24;
    let pct: u32 = l1_now * 100 / EPOCH_L1_BLOCKS;
    assert_eq!(
        pct, 75,
        "24 of 32 L1 blocks must compute to 75% via u32 arithmetic"
    );

    // Type-identity check: binding to `u32` fails to compile if the
    // declaration drifts.
    let typed: u32 = EPOCH_L1_BLOCKS;
    assert_eq!(typed, 32);
}

/// Test Plan row 6: `test_epoch_0_range`.
///
/// Proves the height-to-epoch formula `(h - 1) / BLOCKS_PER_EPOCH` places
/// heights `1` and `32` in epoch 0. This is the spec-mandated boundary
/// from [`SPEC.md` §2.1](../../docs/resources/SPEC.md) table "Epoch 0 L2
/// range: [1, 32]" and from [`specs/CON-001.md`](../../docs/requirements/domains/constants/specs/CON-001.md)
/// row 6. Any off-by-one in `BLOCKS_PER_EPOCH` or `GENESIS_HEIGHT` would
/// misplace the genesis block or the first epoch-checkpoint block (height
/// 32), which HEA-002 uses to compute `epoch_checkpoint_height(0) == 32`.
#[test]
fn test_epoch_0_range() {
    // Lower bound: the genesis block (height 1) must live in epoch 0.
    let first_epoch: u64 = (1 - GENESIS_HEIGHT) / BLOCKS_PER_EPOCH;
    assert_eq!(
        first_epoch, 0,
        "height 1 (GENESIS_HEIGHT) must map to epoch 0: (1 - 1) / 32 == 0"
    );

    // Upper bound: the epoch-0 checkpoint (height 32) must still be
    // epoch 0, not spill into epoch 1.
    let last_in_epoch_0: u64 = (32 - GENESIS_HEIGHT) / BLOCKS_PER_EPOCH;
    assert_eq!(
        last_in_epoch_0, 0,
        "height 32 (epoch-0 checkpoint) must map to epoch 0: (32 - 1) / 32 == 0"
    );
}

/// Test Plan row 7: `test_epoch_1_range`.
///
/// Proves the height-to-epoch formula correctly rolls over at the
/// epoch-0/epoch-1 boundary. Height 33 (first block of epoch 1) and
/// height 64 (epoch-1 checkpoint) must both map to epoch 1 via
/// `(h - 1) / 32`. This pins down the rollover semantics — a buggy
/// formula like `h / 32` would misclassify height 32 as epoch 1 instead
/// of epoch 0. Together with `test_epoch_0_range`, this test establishes
/// the full CON-001 boundary invariant that every HEA-* function relies
/// on.
#[test]
fn test_epoch_1_range() {
    // Lower bound: first block of epoch 1 is height 33.
    let first_in_epoch_1: u64 = (33 - GENESIS_HEIGHT) / BLOCKS_PER_EPOCH;
    assert_eq!(
        first_in_epoch_1, 1,
        "height 33 (first of epoch 1) must map to epoch 1: (33 - 1) / 32 == 1"
    );

    // Upper bound: epoch-1 checkpoint height is 64, still within epoch 1.
    let last_in_epoch_1: u64 = (64 - GENESIS_HEIGHT) / BLOCKS_PER_EPOCH;
    assert_eq!(
        last_in_epoch_1, 1,
        "height 64 (epoch-1 checkpoint) must map to epoch 1: (64 - 1) / 32 == 1"
    );
}
