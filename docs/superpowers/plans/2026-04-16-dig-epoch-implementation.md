# dig-epoch Implementation Plan (TDD, One Requirement at a Time)

> **For agentic workers:** REQUIRED SUB-SKILL: Use `superpowers:subagent-driven-development` (recommended) or `superpowers:executing-plans` to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement the 63 requirements listed in `docs/requirements/IMPLEMENTATION_ORDER.md` for the `dig-epoch` Rust crate (epoch geometry, phase machine, manager, checkpoint competition) following strict TDD, one requirement per commit.

**Architecture:** Rust crate layered as specified in `docs/resources/SPEC.md` Section 13. Public surface re-exported through `src/lib.rs`. Concurrency via `parking_lot::RwLock`. Serialization via `bincode` + `serde`. All Merkle/BLS/hash primitives sourced from the Chia ecosystem crates (never redefined). Checkpoint, block header, and signer bitmap types sourced from `dig-block` (never redefined). Network constants sourced from `dig-constants`.

**Tech Stack:** Rust, `dig-block 0.1`, `dig-constants 0.1`, `chia-protocol 0.26`, `chia-bls 0.26`, `chia-consensus 0.26`, `chia-sdk-types 0.30`, `chia-sdk-signer 0.30`, `chia-sha2 0.26`, `clvm-utils 0.26`, `bincode`, `serde`, `thiserror`, `parking_lot`.

---

## How To Use This Plan

This plan has **three parts**:

1. **Session preamble** (one-time per session) — sync, tool freshness, context packing.
2. **Per-requirement TDD template** (reused for every requirement) — the exact sequence of steps to follow for each of the 63 items.
3. **Requirement schedule** — the 63 requirements in dependency order, each a checkbox that references the template with the specific files, spec pointers, and commit message to use.

Before beginning **each** requirement, the engineer re-reads `docs/prompt/chat.md` (which trampolines into `docs/prompt/start.md`). This is a hard rule — re-reading re-grounds the engineer on the workflow discipline (TDD first, tools fresh, one-per-commit, tracking updates) and prevents drift across the long schedule.

---

## Part 1 — Session Preamble (Run Once Per Work Session)

- [ ] **P1.1 — Sync repository**

  ```bash
  git fetch origin && git pull origin main
  git status
  ```
  Expected: clean working tree, on `main`, up to date with `origin/main`.

- [ ] **P1.2 — Confirm GitNexus index is fresh**

  ```bash
  npx gitnexus status
  ```
  If output says index is stale or missing, run:
  ```bash
  npx gitnexus analyze
  ```
  (Add `--embeddings` if `.gitnexus/meta.json` → `stats.embeddings > 0`.)

- [ ] **P1.3 — Confirm SocratiCode is operational**

  Call the `codebase_status` MCP tool (via SocratiCode) and confirm the index is built. If not, run `codebase_index` to build it.

- [ ] **P1.4 — Pack repo context with Repomix**

  ```bash
  mkdir -p .repomix
  npx repomix@latest src -o .repomix/pack-src.xml          || echo "src not yet created"
  npx repomix@latest tests -o .repomix/pack-tests.xml      || echo "tests not yet created"
  npx repomix@latest docs/requirements -o .repomix/pack-requirements.xml
  npx repomix@latest docs/resources -o .repomix/pack-resources.xml
  ```
  (The `src` and `tests` packs will be empty until STR-001/STR-002 land — that is expected.)

- [ ] **P1.5 — Load the requirements pack into conversation context**

  Read `.repomix/pack-requirements.xml` and `.repomix/pack-resources.xml` so the engineer/subagent has the full spec in working memory.

---

## Part 2 — Per-Requirement TDD Template

**Run every one of these steps for every requirement in Part 3, in order. No shortcuts, no batching across requirements.**

Let `<ID>` = the requirement identifier (e.g. `STR-001`), `<domain>` = its domain directory name (e.g. `crate_structure`), and `<commit_msg>` = the commit message listed for that requirement.

- [ ] **T1 — Re-read the prompt chain (mandatory re-grounding)**

  Read, in order:
  1. `docs/prompt/chat.md`
  2. `docs/prompt/start.md` (linked from chat.md)

  This refreshes the workflow discipline (TDD, tool freshness, one-per-commit, tracking updates, hard requirements list). Do not skip — stale workflow memory is the primary cause of drift on long schedules.

- [ ] **T2 — Read the authoritative spec for `<ID>`**

  Read, in order:
  1. `docs/requirements/domains/<domain>/NORMATIVE.md` (search for `<ID>` anchor)
  2. `docs/requirements/domains/<domain>/specs/<ID>.md` (detailed spec + test plan + expected test file paths)
  3. `docs/requirements/domains/<domain>/VERIFICATION.md` (verification procedure)
  4. `docs/requirements/domains/<domain>/TRACKING.yaml` (current status — confirm `<ID>` is `not_started`)

  Also open the referenced section of `docs/resources/SPEC.md` if the spec links into it.

- [ ] **T3 — GitNexus impact analysis**

  If the requirement modifies an existing symbol (most post-Phase-0 items do), run:
  ```
  gitnexus_impact({target: "<primary_symbol>", direction: "upstream"})
  ```
  Report the blast radius (direct callers, affected processes, risk level). If HIGH/CRITICAL, pause and surface the warning before editing. For net-new symbols (most of Phase 0–3) this step is a no-op — note that explicitly.

- [ ] **T4 — SocratiCode semantic search (when touching existing code)**

  For requirements that modify existing behavior, run a `codebase_search` for the feature area to locate adjacent code you must not break. Skip for net-new files.

- [ ] **T5 — Repack src/tests if prior commits changed them**

  ```bash
  npx repomix@latest src -o .repomix/pack-src.xml
  npx repomix@latest tests -o .repomix/pack-tests.xml
  ```

- [ ] **T6 — Write the failing test FIRST**

  Create the dedicated test file named in the spec's "Expected Test Files" section (typically `tests/<domain>/test_<snake_case_feature>.rs`). One dedicated file per requirement.

  The file MUST contain:
  - A file-level doc comment stating which requirement it verifies and how (semantic link to `docs/requirements/domains/<domain>/specs/<ID>.md`).
  - One or more `#[test]` functions implementing **every row** of the spec's "Test Plan" table.
  - Per-test doc comments describing (a) what behavior is under test and (b) how passing it proves the requirement is satisfied.

- [ ] **T7 — Run the test and confirm it FAILS**

  ```bash
  cargo test --test test_<snake_case_feature>
  ```
  Expected: compile error (symbol missing) OR assertion failure. A passing test here means the test is not actually exercising the new behavior — fix the test before continuing.

- [ ] **T8 — Implement the minimal code to satisfy the test**

  Write or edit the source files identified in the spec. Every new item (fn/struct/enum/const/module) gets a high-signal doc comment covering:
  - What it is and how to use it.
  - Rationale / design decision / invariants.
  - Semantic links to `docs/resources/SPEC.md` section and `docs/requirements/domains/<domain>/specs/<ID>.md`.
  - Cross-links to related symbols in the crate when helpful.

  Use only crates listed in `docs/prompt/start.md` → "Tech Stack". Never redefine `Checkpoint`, `CheckpointSubmission`, `SignerBitmap`, `L2BlockHeader`, `Bytes32`, BLS, or Merkle primitives — import them.

- [ ] **T9 — Run the test and confirm it PASSES**

  ```bash
  cargo test --test test_<snake_case_feature>
  ```
  Expected: all test cases pass.

- [ ] **T10 — Run the full suite, clippy, and fmt**

  ```bash
  cargo test
  cargo clippy --all-targets -- -D warnings
  cargo fmt --check
  ```
  All three must succeed. If `fmt --check` reports diffs, run `cargo fmt` and re-run.

- [ ] **T11 — Update tracking artifacts**

  1. `docs/requirements/domains/<domain>/TRACKING.yaml` — flip `<ID>` status from `not_started` → `verified`, fill in completion date (`2026-04-16` or current), test file path, and commit hash (leave blank; fill after commit).
  2. `docs/requirements/domains/<domain>/VERIFICATION.md` — append a row/section showing the verification evidence (test file path, `cargo test` output snippet, date).
  3. `docs/requirements/IMPLEMENTATION_ORDER.md` — change `- [ ] <ID>` to `- [x] <ID>`.

- [ ] **T12 — Verify change scope with GitNexus**

  ```
  gitnexus_detect_changes({scope: "staged"})
  ```
  Confirm only expected symbols/files changed. Investigate any surprises before committing.

- [ ] **T13 — Commit (one requirement per commit)**

  ```bash
  git add -- <specific files you changed>   # avoid git add -A
  git commit -m "<commit_msg>"
  ```
  Commit message format: `feat(<domain>): <ID> <short description>` (matches repo convention). Keep the body empty unless the *why* is non-obvious.

- [ ] **T14 — Back-fill the commit hash in TRACKING.yaml**

  Paste the `git rev-parse HEAD` short hash into the `<ID>` entry, `git add` and amend into the same commit:
  ```bash
  git commit --amend --no-edit
  ```

- [ ] **T15 — Push**

  ```bash
  git push origin main
  ```

- [ ] **T16 — Refresh the GitNexus index**

  ```bash
  npx gitnexus analyze
  ```
  (Add `--embeddings` if previously enabled.)

- [ ] **T17 — Move to the next requirement in Part 3**

---

## Part 3 — Requirement Schedule (63 items)

For each entry: apply Template T1–T17 using the listed target files, spec pointers, and commit message. Entries are in dependency order; do not reorder.

### Phase 0 — Crate Structure & Foundation

- [ ] **STR-001** — Cargo.toml dependencies
  - Domain: `crate_structure`
  - Spec: `docs/requirements/domains/crate_structure/specs/STR-001.md`
  - Create: `Cargo.toml`, `src/lib.rs` (empty stub), `tests/crate_structure/test_dependency_imports.rs`
  - Commit: `feat(crate_structure): STR-001 declare Cargo.toml dependencies`

- [ ] **STR-002** — Module hierarchy matching SPEC §13
  - Spec: `docs/requirements/domains/crate_structure/specs/STR-002.md`
  - Create: `src/constants.rs`, `src/errors.rs`, `src/types.rs`, `src/height.rs`, `src/phase.rs`, `src/rewards.rs`, `src/manager.rs`, `src/checkpoint_competition.rs`, `src/verification.rs`, `src/dfsp.rs`, `src/serialization.rs` (empty stubs with module doc comments); wire them into `src/lib.rs`; `tests/crate_structure/test_module_hierarchy.rs`
  - Commit: `feat(crate_structure): STR-002 scaffold module hierarchy`

- [ ] **STR-003** — Public re-exports
  - Spec: `docs/requirements/domains/crate_structure/specs/STR-003.md`
  - Modify: `src/lib.rs` to `pub use` the canonical surface (EpochManager, types, constants, functions)
  - Test: `tests/crate_structure/test_public_reexports.rs`
  - Commit: `feat(crate_structure): STR-003 expose public re-exports`

- [ ] **STR-004** — EpochManager constructor
  - Spec: `docs/requirements/domains/crate_structure/specs/STR-004.md`
  - Modify: `src/manager.rs` to declare `EpochManager` struct + `new(network_id, genesis_l1_height, initial_state_root)`
  - Test: `tests/crate_structure/test_epoch_manager_constructor.rs`
  - Commit: `feat(crate_structure): STR-004 add EpochManager::new constructor`

- [ ] **STR-005** — Test infrastructure
  - Spec: `docs/requirements/domains/crate_structure/specs/STR-005.md`
  - Create: `tests/common/mod.rs` (test EpochManager builder, test block fixtures, helpers)
  - Test: `tests/crate_structure/test_test_infrastructure.rs`
  - Commit: `test(crate_structure): STR-005 add shared test infrastructure`

### Phase 1 — Constants

- [ ] **CON-001** — Epoch geometry constants
  - Spec: `docs/requirements/domains/constants/specs/CON-001.md`
  - Modify: `src/constants.rs` (add `BLOCKS_PER_EPOCH`, `EPOCH_L1_BLOCKS`, `GENESIS_HEIGHT`)
  - Test: `tests/constants/test_epoch_geometry.rs`
  - Commit: `feat(constants): CON-001 declare epoch geometry constants`

- [ ] **CON-002** — Phase boundary constants
  - Spec: `docs/requirements/domains/constants/specs/CON-002.md` (fall back to NORMATIVE.md if spec file absent)
  - Modify: `src/constants.rs` (50%, 75%, 100% phase thresholds)
  - Test: `tests/constants/test_phase_boundaries.rs`
  - Commit: `feat(constants): CON-002 declare phase boundary constants`

- [ ] **CON-003** — Reward economics constants
  - Spec: `docs/requirements/domains/constants/specs/CON-003.md`
  - Modify: `src/constants.rs` (MOJOS_PER_L2, INITIAL_BLOCK_REWARD, halving interval, tail, bonus)
  - Test: `tests/constants/test_reward_economics.rs`
  - Commit: `feat(constants): CON-003 declare reward economics constants`

- [ ] **CON-004** — Fee and reward distribution percentages (5-role split)
  - Spec: `docs/requirements/domains/constants/specs/CON-004.md` (fall back to NORMATIVE.md if absent)
  - Modify: `src/constants.rs`
  - Test: `tests/constants/test_distribution_percentages.rs` — MUST assert percentages sum to 100
  - Commit: `feat(constants): CON-004 declare 5-role distribution percentages`

- [ ] **CON-005** — DFSP, consensus, slashing, withdrawal constants
  - Spec: `docs/requirements/domains/constants/specs/CON-005.md`
  - Modify: `src/constants.rs`
  - Test: `tests/constants/test_dfsp_consensus_slashing.rs`
  - Commit: `feat(constants): CON-005 declare DFSP/consensus/slashing/withdrawal constants`

- [ ] **CON-006** — Sentinel constants (EMPTY_ROOT)
  - Spec: `docs/requirements/domains/constants/specs/CON-006.md`
  - Modify: `src/constants.rs`
  - Test: `tests/constants/test_sentinels.rs`
  - Commit: `feat(constants): CON-006 declare sentinel constants`

### Phase 2 — Error Types

- [ ] **ERR-001** — `EpochError` enum
  - Spec: `docs/requirements/domains/error_types/specs/ERR-001.md`
  - Modify: `src/errors.rs` (use `thiserror`)
  - Test: `tests/error_types/test_epoch_error.rs`
  - Commit: `feat(error_types): ERR-001 define EpochError enum`

- [ ] **ERR-002** — `CheckpointCompetitionError` enum
  - Spec: `docs/requirements/domains/error_types/specs/ERR-002.md`
  - Modify: `src/errors.rs`
  - Test: `tests/error_types/test_checkpoint_competition_error.rs`
  - Commit: `feat(error_types): ERR-002 define CheckpointCompetitionError enum`

- [ ] **ERR-003** — `From` conversions + `Display` messages
  - Spec: `docs/requirements/domains/error_types/specs/ERR-003.md`
  - Modify: `src/errors.rs`
  - Test: `tests/error_types/test_error_conversions.rs`
  - Commit: `feat(error_types): ERR-003 implement error conversions and messages`

### Phase 3 — Epoch Types

- [ ] **TYP-001** — `EpochPhase` enum + `PhaseTransition` struct
  - Spec: `docs/requirements/domains/epoch_types/specs/TYP-001.md`
  - Modify: `src/types.rs`
  - Test: `tests/epoch_types/test_epoch_phase.rs`
  - Commit: `feat(epoch_types): TYP-001 define EpochPhase and PhaseTransition`

- [ ] **TYP-002** — `EpochInfo` struct
  - Spec: `docs/requirements/domains/epoch_types/specs/TYP-002.md`
  - Modify: `src/types.rs`
  - Test: `tests/epoch_types/test_epoch_info.rs`
  - Commit: `feat(epoch_types): TYP-002 define EpochInfo struct`

- [ ] **TYP-003** — `EpochSummary` struct (immutable archive)
  - Spec: `docs/requirements/domains/epoch_types/specs/TYP-003.md`
  - Modify: `src/types.rs`
  - Test: `tests/epoch_types/test_epoch_summary.rs`
  - Commit: `feat(epoch_types): TYP-003 define EpochSummary archive struct`

- [ ] **TYP-004** — `DfspCloseSnapshot` struct
  - Spec: `docs/requirements/domains/epoch_types/specs/TYP-004.md`
  - Modify: `src/types.rs`
  - Test: `tests/epoch_types/test_dfsp_close_snapshot.rs`
  - Commit: `feat(epoch_types): TYP-004 define DfspCloseSnapshot struct`

- [ ] **TYP-005** — `EpochEvent` enum
  - Spec: `docs/requirements/domains/epoch_types/specs/TYP-005.md`
  - Modify: `src/types.rs`
  - Test: `tests/epoch_types/test_epoch_event.rs`
  - Commit: `feat(epoch_types): TYP-005 define EpochEvent enum`

- [ ] **TYP-006** — `EpochBlockLink` struct
  - Spec: `docs/requirements/domains/epoch_types/specs/TYP-006.md`
  - Modify: `src/types.rs`
  - Test: `tests/epoch_types/test_epoch_block_link.rs`
  - Commit: `feat(epoch_types): TYP-006 define EpochBlockLink struct`

### Phase 4 — Height-Epoch Arithmetic

- [ ] **HEA-001** — `epoch_for_block_height(h) -> u64`
  - Spec: `docs/requirements/domains/height_arithmetic/specs/HEA-001.md`
  - Modify: `src/height.rs`
  - Test: `tests/height_arithmetic/test_epoch_for_block_height.rs`
  - Commit: `feat(height_arithmetic): HEA-001 implement epoch_for_block_height`

- [ ] **HEA-002** — `first_height_in_epoch(e)` + `epoch_checkpoint_height(e)`
  - Spec: `docs/requirements/domains/height_arithmetic/specs/HEA-002.md`
  - Modify: `src/height.rs`
  - Test: `tests/height_arithmetic/test_epoch_bounds.rs`
  - Commit: `feat(height_arithmetic): HEA-002 implement epoch bounds helpers`

- [ ] **HEA-003** — `is_genesis_checkpoint_block`, `is_epoch_checkpoint_block`, `is_checkpoint_class_block`, `ensure_checkpoint_block_empty`
  - Spec: `docs/requirements/domains/height_arithmetic/specs/HEA-003.md`
  - Modify: `src/height.rs`
  - Test: `tests/height_arithmetic/test_checkpoint_block_predicates.rs` — MUST assert checkpoint blocks have zero SpendBundles, zero cost, zero fees
  - Commit: `feat(height_arithmetic): HEA-003 implement checkpoint block predicates`

- [ ] **HEA-004** — `l1_range_for_epoch(genesis_l1_height, epoch) -> (u32, u32)`
  - Spec: `docs/requirements/domains/height_arithmetic/specs/HEA-004.md`
  - Modify: `src/height.rs`
  - Test: `tests/height_arithmetic/test_l1_range_for_epoch.rs`
  - Commit: `feat(height_arithmetic): HEA-004 implement l1_range_for_epoch`

- [ ] **HEA-005** — Height-epoch round-trip identity property
  - Spec: `docs/requirements/domains/height_arithmetic/specs/HEA-005.md`
  - Test: `tests/height_arithmetic/test_height_epoch_roundtrip.rs` — property-style test across full height domain
  - Commit: `test(height_arithmetic): HEA-005 verify height-epoch round-trip identity`

### Phase 5 — Phase State Machine

- [ ] **PHS-001** — `l1_progress_phase_for_network_epoch()` free function
  - Spec: `docs/requirements/domains/phase_machine/specs/PHS-001.md`
  - Modify: `src/phase.rs`
  - Test: `tests/phase_machine/test_l1_progress_phase.rs`
  - Commit: `feat(phase_machine): PHS-001 implement l1_progress_phase_for_network_epoch`

- [ ] **PHS-002** — `EpochManager` phase tracking (`current_phase`, `update_phase`)
  - Spec: `docs/requirements/domains/phase_machine/specs/PHS-002.md`
  - Modify: `src/manager.rs`, `src/phase.rs`
  - Test: `tests/phase_machine/test_phase_tracking.rs`
  - Commit: `feat(phase_machine): PHS-002 add EpochManager phase tracking`

- [ ] **PHS-003** — Phase transition events + `should_advance()`
  - Spec: `docs/requirements/domains/phase_machine/specs/PHS-003.md`
  - Modify: `src/phase.rs`, `src/manager.rs`
  - Test: `tests/phase_machine/test_phase_transitions.rs`
  - Commit: `feat(phase_machine): PHS-003 emit transition events and should_advance`

- [ ] **PHS-004** — Phase boundary enforcement (`PhaseMismatch` errors)
  - Spec: `docs/requirements/domains/phase_machine/specs/PHS-004.md`
  - Modify: `src/phase.rs`, `src/manager.rs`, `src/errors.rs`
  - Test: `tests/phase_machine/test_phase_boundary_enforcement.rs`
  - Commit: `feat(phase_machine): PHS-004 enforce phase boundaries`

### Phase 6 — Reward Economics

- [ ] **REW-001** — `block_reward_at_height()` with halving schedule
  - Spec: `docs/requirements/domains/reward_economics/specs/REW-001.md`
  - Modify: `src/rewards.rs`
  - Test: `tests/reward_economics/test_block_reward_at_height.rs`
  - Commit: `feat(reward_economics): REW-001 implement block_reward_at_height with halvings`

- [ ] **REW-002** — `total_block_reward()` with epoch-first-block bonus
  - Spec: `docs/requirements/domains/reward_economics/specs/REW-002.md`
  - Modify: `src/rewards.rs`
  - Test: `tests/reward_economics/test_total_block_reward.rs`
  - Commit: `feat(reward_economics): REW-002 implement total_block_reward with first-block bonus`

- [ ] **REW-003** — `proposer_fee_share()` + `burned_fee_remainder()`
  - Spec: `docs/requirements/domains/reward_economics/specs/REW-003.md`
  - Modify: `src/rewards.rs`
  - Test: `tests/reward_economics/test_fee_split.rs`
  - Commit: `feat(reward_economics): REW-003 implement proposer fee share and burn`

- [ ] **REW-004** — `compute_reward_distribution()` 5-role split
  - Spec: `docs/requirements/domains/reward_economics/specs/REW-004.md`
  - Modify: `src/rewards.rs`
  - Test: `tests/reward_economics/test_reward_distribution.rs` — MUST assert sum-of-shares == total reward, no mojo lost
  - Commit: `feat(reward_economics): REW-004 implement 5-role reward distribution`

- [ ] **REW-005** — Tail emission floor (`MINIMUM_EPOCH_REWARD`)
  - Spec: `docs/requirements/domains/reward_economics/specs/REW-005.md`
  - Modify: `src/rewards.rs`
  - Test: `tests/reward_economics/test_tail_emission_floor.rs`
  - Commit: `feat(reward_economics): REW-005 enforce tail emission floor`

- [ ] **REW-006** — Halving interval boundary verification
  - Spec: `docs/requirements/domains/reward_economics/specs/REW-006.md`
  - Test: `tests/reward_economics/test_halving_boundaries.rs` — exercise both sides of each halving block
  - Commit: `test(reward_economics): REW-006 verify halving interval boundaries`

### Phase 7 — Epoch Manager

- [ ] **MGR-001** — `EpochManager` with interior mutability (`parking_lot::RwLock`)
  - Spec: `docs/requirements/domains/epoch_manager/specs/MGR-001.md`
  - Modify: `src/manager.rs`
  - Test: `tests/epoch_manager/test_interior_mutability.rs` — MUST include a concurrent reader/writer case
  - Commit: `feat(epoch_manager): MGR-001 wrap EpochManager state in parking_lot RwLock`

- [ ] **MGR-002** — `record_block(fees, tx_count)`
  - Spec: `docs/requirements/domains/epoch_manager/specs/MGR-002.md`
  - Modify: `src/manager.rs`
  - Test: `tests/epoch_manager/test_record_block.rs`
  - Commit: `feat(epoch_manager): MGR-002 implement record_block`

- [ ] **MGR-003** — `set_current_epoch_chain_totals(blocks, fees, txns)`
  - Spec: `docs/requirements/domains/epoch_manager/specs/MGR-003.md`
  - Modify: `src/manager.rs`
  - Test: `tests/epoch_manager/test_set_current_epoch_chain_totals.rs`
  - Commit: `feat(epoch_manager): MGR-003 implement set_current_epoch_chain_totals`

- [ ] **MGR-004** — `advance_epoch(l1_height, state_root)`
  - Spec: `docs/requirements/domains/epoch_manager/specs/MGR-004.md`
  - Modify: `src/manager.rs`
  - Test: `tests/epoch_manager/test_advance_epoch.rs` — MUST verify prior epoch summary is archived immutably and current-epoch state is reset
  - Commit: `feat(epoch_manager): MGR-004 implement advance_epoch`

- [ ] **MGR-005** — Query methods (`get_epoch_info`, `get_epoch_summary`, `recent_summaries`, `total_stats`)
  - Spec: `docs/requirements/domains/epoch_manager/specs/MGR-005.md`
  - Modify: `src/manager.rs`
  - Test: `tests/epoch_manager/test_query_methods.rs`
  - Commit: `feat(epoch_manager): MGR-005 implement query methods`

- [ ] **MGR-006** — `set_current_epoch_dfsp_close_snapshot()`
  - Spec: `docs/requirements/domains/epoch_manager/specs/MGR-006.md`
  - Modify: `src/manager.rs`
  - Test: `tests/epoch_manager/test_set_dfsp_close_snapshot.rs`
  - Commit: `feat(epoch_manager): MGR-006 implement set_current_epoch_dfsp_close_snapshot`

- [ ] **MGR-007** — Epoch history management (summaries storage)
  - Spec: `docs/requirements/domains/epoch_manager/specs/MGR-007.md`
  - Modify: `src/manager.rs`
  - Test: `tests/epoch_manager/test_summaries_storage.rs`
  - Commit: `feat(epoch_manager): MGR-007 implement epoch history storage`

### Phase 8 — Checkpoint Competition

- [ ] **CKP-001** — `CheckpointCompetition` struct + `CompetitionStatus` enum
  - Spec: `docs/requirements/domains/checkpoint_competition/specs/CKP-001.md`
  - Modify: `src/checkpoint_competition.rs`
  - Test: `tests/checkpoint_competition/test_types.rs`
  - Commit: `feat(checkpoint_competition): CKP-001 define competition types`

- [ ] **CKP-002** — `start_checkpoint_competition()`
  - Spec: `docs/requirements/domains/checkpoint_competition/specs/CKP-002.md`
  - Modify: `src/checkpoint_competition.rs`, `src/manager.rs`
  - Test: `tests/checkpoint_competition/test_start_competition.rs`
  - Commit: `feat(checkpoint_competition): CKP-002 implement start_checkpoint_competition`

- [ ] **CKP-003** — `submit_checkpoint()` with score comparison
  - Spec: `docs/requirements/domains/checkpoint_competition/specs/CKP-003.md`
  - Modify: `src/checkpoint_competition.rs`
  - Test: `tests/checkpoint_competition/test_submit_checkpoint.rs`
  - Commit: `feat(checkpoint_competition): CKP-003 implement submit_checkpoint scoring`

- [ ] **CKP-004** — `finalize_competition()` + `get_competition()`
  - Spec: `docs/requirements/domains/checkpoint_competition/specs/CKP-004.md`
  - Modify: `src/checkpoint_competition.rs`
  - Test: `tests/checkpoint_competition/test_finalize_competition.rs`
  - Commit: `feat(checkpoint_competition): CKP-004 implement finalize_competition`

- [ ] **CKP-005** — Lifecycle: Pending → Collecting → WinnerSelected → Finalized/Failed
  - Spec: `docs/requirements/domains/checkpoint_competition/specs/CKP-005.md`
  - Test: `tests/checkpoint_competition/test_lifecycle.rs` — exercise every legal and every illegal transition
  - Commit: `test(checkpoint_competition): CKP-005 verify competition lifecycle`

### Phase 9 — Verification

- [ ] **VER-001** — `compute_epoch_block_root()` via `chia-sdk-types::MerkleTree`
  - Spec: `docs/requirements/domains/verification/specs/VER-001.md`
  - Modify: `src/verification.rs`
  - Test: `tests/verification/test_compute_epoch_block_root.rs`
  - Commit: `feat(verification): VER-001 implement compute_epoch_block_root`

- [ ] **VER-002** — `epoch_block_inclusion_proof()` via `MerkleProof`
  - Spec: `docs/requirements/domains/verification/specs/VER-002.md`
  - Modify: `src/verification.rs`
  - Test: `tests/verification/test_epoch_block_inclusion_proof.rs`
  - Commit: `feat(verification): VER-002 implement epoch_block_inclusion_proof`

- [ ] **VER-003** — `compute_epoch_withdrawals_root()` via `chia-consensus::compute_merkle_set_root`
  - Spec: `docs/requirements/domains/verification/specs/VER-003.md`
  - Modify: `src/verification.rs`
  - Test: `tests/verification/test_compute_epoch_withdrawals_root.rs`
  - Commit: `feat(verification): VER-003 implement compute_epoch_withdrawals_root`

- [ ] **VER-004** — `EpochCheckpointData` + `EpochCheckpointSignMaterial`
  - Spec: `docs/requirements/domains/verification/specs/VER-004.md`
  - Modify: `src/verification.rs`
  - Test: `tests/verification/test_checkpoint_sign_material.rs`
  - Commit: `feat(verification): VER-004 define EpochCheckpointData and sign material`

- [ ] **VER-005** — `stored_checkpoint_from_epoch_sign_material_with_aggregate_v1()`
  - Spec: `docs/requirements/domains/verification/specs/VER-005.md`
  - Modify: `src/verification.rs`
  - Test: `tests/verification/test_stored_checkpoint_from_sign_material.rs`
  - Commit: `feat(verification): VER-005 implement stored_checkpoint_from_epoch_sign_material_with_aggregate_v1`

### Phase 10 — DFSP Processing

- [ ] **DFS-001** — `DfspEpochBurnPolicyV1` + burn context
  - Spec: `docs/requirements/domains/dfsp_processing/specs/DFS-001.md`
  - Modify: `src/dfsp.rs`
  - Test: `tests/dfsp_processing/test_burn_policy.rs`
  - Commit: `feat(dfsp_processing): DFS-001 define DfspEpochBurnPolicyV1 and burn context`

- [ ] **DFS-002** — Storage proof evaluation context + issuance preview
  - Spec: `docs/requirements/domains/dfsp_processing/specs/DFS-002.md`
  - Modify: `src/dfsp.rs`
  - Test: `tests/dfsp_processing/test_storage_proof_context.rs`
  - Commit: `feat(dfsp_processing): DFS-002 add storage proof context and issuance preview`

- [ ] **DFS-003** — Epoch boundary finalize preview + staged outputs
  - Spec: `docs/requirements/domains/dfsp_processing/specs/DFS-003.md`
  - Modify: `src/dfsp.rs`
  - Test: `tests/dfsp_processing/test_finalize_preview.rs`
  - Commit: `feat(dfsp_processing): DFS-003 add finalize preview and staged outputs`

- [ ] **DFS-004** — Finalize roots commitment digest computation
  - Spec: `docs/requirements/domains/dfsp_processing/specs/DFS-004.md` (fall back to NORMATIVE.md if absent)
  - Modify: `src/dfsp.rs`
  - Test: `tests/dfsp_processing/test_finalize_roots_digest.rs`
  - Commit: `feat(dfsp_processing): DFS-004 compute finalize roots commitment digest`

- [ ] **DFS-005** — DFSP activation control (`is_dfsp_active_at_height`, `dfsp_activation_height_for_network`)
  - Spec: `docs/requirements/domains/dfsp_processing/specs/DFS-005.md`
  - Modify: `src/dfsp.rs`
  - Test: `tests/dfsp_processing/test_activation_control.rs`
  - Commit: `feat(dfsp_processing): DFS-005 implement DFSP activation control`

- [ ] **DFS-006** — DFSP namespace rollup + tail roots computation
  - Spec: `docs/requirements/domains/dfsp_processing/specs/DFS-006.md` (fall back to NORMATIVE.md if absent)
  - Modify: `src/dfsp.rs`
  - Test: `tests/dfsp_processing/test_namespace_rollup.rs`
  - Commit: `feat(dfsp_processing): DFS-006 compute DFSP namespace rollup and tail roots`

- [ ] **DFS-007** — Parse burn policy schedule from string configuration
  - Spec: `docs/requirements/domains/dfsp_processing/specs/DFS-007.md`
  - Modify: `src/dfsp.rs`
  - Test: `tests/dfsp_processing/test_parse_burn_policy_schedule.rs` — MUST include malformed-input cases
  - Commit: `feat(dfsp_processing): DFS-007 parse burn policy schedule config`

- [ ] **DFS-008** — `apply_epoch_storage_proof_evaluation_step_v1`
  - Spec: `docs/requirements/domains/dfsp_processing/specs/DFS-008.md`
  - Modify: `src/dfsp.rs`
  - Test: `tests/dfsp_processing/test_apply_storage_proof_step.rs`
  - Commit: `feat(dfsp_processing): DFS-008 implement apply_epoch_storage_proof_evaluation_step_v1`

### Phase 11 — Serialization

- [ ] **SER-001** — Bincode serialization for all epoch types
  - Spec: `docs/requirements/domains/serialization/specs/SER-001.md`
  - Modify: `src/serialization.rs`, plus any type that gains `Serialize`/`Deserialize` derives
  - Test: `tests/serialization/test_bincode_support.rs`
  - Commit: `feat(serialization): SER-001 add bincode serialization for epoch types`

- [ ] **SER-002** — `to_bytes` / `from_bytes` conventions
  - Spec: `docs/requirements/domains/serialization/specs/SER-002.md`
  - Modify: `src/serialization.rs`
  - Test: `tests/serialization/test_to_from_bytes.rs`
  - Commit: `feat(serialization): SER-002 implement to_bytes/from_bytes conventions`

- [ ] **SER-003** — Round-trip integrity for all serializable types
  - Spec: `docs/requirements/domains/serialization/specs/SER-003.md`
  - Test: `tests/serialization/test_roundtrip_integrity.rs` — exercise every serializable type in the public surface
  - Commit: `test(serialization): SER-003 verify round-trip integrity for all types`

---

## Final Gate (Run After Phase 11 — Not Per Requirement)

- [ ] **F1 — Full crate verification**

  ```bash
  cargo test
  cargo clippy --all-targets -- -D warnings
  cargo fmt --check
  cargo doc --no-deps
  ```
  All must pass cleanly.

- [ ] **F2 — Requirement registry sanity check**

  Confirm every ID in `docs/requirements/REQUIREMENTS_REGISTRY.yaml` and every `- [ ]` in `IMPLEMENTATION_ORDER.md` is now `- [x]` and verified in its domain `TRACKING.yaml`.

- [ ] **F3 — GitNexus final impact scan**

  ```
  gitnexus_detect_changes({scope: "compare", base_ref: "main~63"})
  ```
  Sanity-check: the branch produced exactly the expected files, nothing extraneous.

- [ ] **F4 — Index + repack for the next operator**

  ```bash
  npx gitnexus analyze
  npx repomix@latest src -o .repomix/pack-src.xml
  npx repomix@latest tests -o .repomix/pack-tests.xml
  ```

---

## Self-Review Notes (Author → Executor)

1. **Spec coverage:** Every ID listed in `IMPLEMENTATION_ORDER.md` has a Part 3 entry with a spec pointer, target file(s), a dedicated test file path, and a commit message.
2. **Template discipline:** Each requirement reuses the same 17-step template (T1–T17). T1 is the chat.md re-reading step the user explicitly requested.
3. **No placeholders in the plan itself:** Code bodies live in the spec files (they already contain full test plans and implementation notes). The plan deliberately defers to those authoritative documents rather than duplicating and risking divergence — read the spec at T2, do not invent.
4. **Hard requirements** from `docs/prompt/start.md` (checkpoint/block types from `dig-block`, `Bytes32` from `chia-protocol`, BLS from `chia-bls`, Merkle from `chia-consensus`/`chia-sdk-types`, SHA-256 from `chia-sha2`, interior mutability via `parking_lot`, height 1 = genesis, checkpoint blocks empty, TEST FIRST, one-per-commit, update tracking) are encoded in T6–T11 of the template and called out in the relevant requirement entries.
5. **Post-Phase-11 drift:** Some spec files exist for IDs not in `IMPLEMENTATION_ORDER.md` (HEA-006, HEA-007, TYP-007, REW-007, MGR-008). They are intentionally out of scope for this plan.
