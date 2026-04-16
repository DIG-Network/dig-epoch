# Crate Structure - Verification Matrix

> **Domain:** crate_structure
> **Prefix:** STR
> **Normative:** [NORMATIVE.md](./NORMATIVE.md)
> **Tracking:** [TRACKING.yaml](./TRACKING.yaml)

| ID | Status | Summary | Verification Approach |
|----|--------|---------|----------------------|
| STR-001 | verified | Cargo.toml Dependencies | Build check: `cargo check` succeeds with all listed dependencies. Manual review: verify Cargo.toml lists dig-block 0.1, dig-constants 0.1, chia-protocol 0.26, chia-bls 0.26, chia-consensus 0.26, chia-sdk-types 0.30, chia-sdk-signer 0.30, chia-sha2 0.26, clvm-utils 0.26, bincode, serde (derive), thiserror, parking_lot. **Evidence:** `tests/crate_structure/test_dependency_imports.rs` covers all 7 rows of the STR-001 Test Plan plus cross-checks for the serde derive feature and every non-row crate; `cargo test --test test_dependency_imports` reports 10/10 passing. |
| STR-002 | verified | Module Hierarchy | Build check: `cargo check` confirms all modules resolve. Manual review: verify constants.rs, types/ directory with epoch_phase.rs, epoch_info.rs, epoch_summary.rs, dfsp.rs, events.rs, checkpoint_competition.rs, reward.rs, verification.rs exist. Verify arithmetic.rs, phase.rs, rewards.rs, manager.rs, verification.rs, dfsp.rs, error.rs exist at crate root. **Evidence:** `tests/crate_structure/test_module_hierarchy.rs` implements all 10 rows of the STR-002 Test Plan (one `#[test]` per row, each importing `STR_002_MODULE_PRESENT` from the module under test); `cargo test --test test_module_hierarchy` reports 10/10 passing. `cargo clippy --all-targets -- -D warnings` and `cargo fmt --check` clean. `src/lib.rs` declares all 8 root modules plus `pub mod types;`, with `src/types/mod.rs` declaring the 8 `types/` submodules — no `pub use` re-exports yet (that is STR-003). |
| STR-003 | gap | Public Re-exports | Compile-time check: downstream test crate uses `use dig_epoch::*` and references EpochManager, all type structs/enums, all constants, all free functions, and error types. Compilation success proves all re-exports are present. |
| STR-004 | gap | EpochManager Constructor | Unit test: call `EpochManager::new(network_id, genesis_l1_height, initial_state_root)`. Verify `current_epoch()` returns 0, `current_phase()` returns `BlockProduction`, `current_epoch_info()` has correct start heights and state root. Verify `get_competition(0)` returns `None` and `get_rewards(0)` returns `None`. |
| STR-005 | gap | Test Infrastructure | Unit test: call each test helper and verify it produces a usable result. Create test EpochManager, advance through phases, create mock checkpoint submissions, build N-block epoch chain. Verify all helpers produce deterministic, valid outputs. |
