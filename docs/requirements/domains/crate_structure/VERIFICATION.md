# Crate Structure - Verification Matrix

> **Domain:** crate_structure
> **Prefix:** STR
> **Normative:** [NORMATIVE.md](./NORMATIVE.md)
> **Tracking:** [TRACKING.yaml](./TRACKING.yaml)

| ID | Status | Summary | Verification Approach |
|----|--------|---------|----------------------|
| STR-001 | gap | Cargo.toml Dependencies | Build check: `cargo check` succeeds with all listed dependencies. Manual review: verify Cargo.toml lists dig-block 0.1, dig-constants 0.1, chia-protocol 0.26, chia-bls 0.26, chia-consensus 0.26, chia-sdk-types 0.30, chia-sdk-signer 0.30, chia-sha2 0.26, clvm-utils 0.26, bincode, serde (derive), thiserror, parking_lot. |
| STR-002 | gap | Module Hierarchy | Build check: `cargo check` confirms all modules resolve. Manual review: verify constants.rs, types/ directory with epoch_phase.rs, epoch_info.rs, epoch_summary.rs, dfsp.rs, events.rs, checkpoint_competition.rs, reward.rs, verification.rs exist. Verify arithmetic.rs, phase.rs, rewards.rs, manager.rs, verification.rs, dfsp.rs, error.rs exist at crate root. |
| STR-003 | gap | Public Re-exports | Compile-time check: downstream test crate uses `use dig_epoch::*` and references EpochManager, all type structs/enums, all constants, all free functions, and error types. Compilation success proves all re-exports are present. |
| STR-004 | gap | EpochManager Constructor | Unit test: call `EpochManager::new(network_id, genesis_l1_height, initial_state_root)`. Verify `current_epoch()` returns 0, `current_phase()` returns `BlockProduction`, `current_epoch_info()` has correct start heights and state root. Verify `get_competition(0)` returns `None` and `get_rewards(0)` returns `None`. |
| STR-005 | gap | Test Infrastructure | Unit test: call each test helper and verify it produces a usable result. Create test EpochManager, advance through phases, create mock checkpoint submissions, build N-block epoch chain. Verify all helpers produce deterministic, valid outputs. |
