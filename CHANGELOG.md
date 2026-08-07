# Changelog

All notable changes to this project are documented here.
This project adheres to [Semantic Versioning](https://semver.org) and
[Conventional Commits](https://www.conventionalcommits.org).

## [0.1.3] - 2026-08-07

### CI
- **dig-epoch:** Title GitHub Releases dig-epoch, not dig-constants (#2299)

## [0.1.2] - 2026-08-07

### Documentation
- **spec:** Land the root normative SPEC.md and name one canonical spec (#2)

## [0.1.1] - 2026-07-12

### Testing
- Cover remaining manager + competition branches (98.2% -> 99.9%)

### CI
- Gate line coverage at >=80% with cargo-llvm-cov- Run test + coverage gate on push/PR via dedicated ci.yml- Enforce version increment in PRs (package.json / Cargo.toml)- Enforce Conventional Commits with commitlint on PRs- Enforce Conventional Commits with commitlint on PRs- Release automation (git-cliff changelog + tag on merge); publish is manual workflow_dispatch (#230)- Re-arm crates.io auto-publish on version tag (token in org secrets; auto-publish-everything #230)- Add flaky-test management (#489) (#1)

### Styling
- Rustfmt wrap winner() expect chain in ckp_005 test

### Chores
- **changelog:** Add git-cliff config for Conventional-Commit changelog

## [0.1.0] - 2026-04-17

### Features
- **crate_structure:** STR-001 declare Cargo.toml dependencies- **crate_structure:** STR-002 scaffold module hierarchy- **constants:** CON-001 declare epoch geometry constants- **constants:** CON-002 declare phase boundary constants- **constants:** CON-003 declare reward economics constants- **constants:** CON-004 declare fee and reward distribution constants- **constants:** CON-005 declare DFSP, consensus, and slashing constants- **constants:** CON-006 declare EMPTY_ROOT sentinel constant- **error_types,epoch_types:** ERR-001/002/003 error enums + TYP-001 EpochPhase- **epoch_types:** TYP-002/004/005/006/007 implement epoch type structs- **epoch_types:** TYP-003 implement EpochSummary immutable archive- **height_arithmetic:** HEA-001–005 implement epoch height arithmetic functions- **height_arithmetic:** HEA-006/007 last_committed_height and first_after_checkpoint- **phase_machine:** PHS-001–004 implement phase calculation and EpochManager phase tracking- **reward_economics:** REW-001–007 implement reward functions and RewardDistribution- **epoch_manager:** MGR-001–008 + CKP-001 struct/enum- **checkpoint_competition:** CKP-002–005 lifecycle methods- **verification:** VER-001–005 Merkle roots, proofs, and BLS aggregation- **serialization:** SER-001–003 bincode + to_bytes/from_bytes- **crate_structure:** STR-003/004/005 re-exports, constructor, helpers

### Bug Fixes
- **crate_structure:** Correct dig-block/dig-constants path deps for main workspace- **cargo:** Drop path overrides for dig-block and dig-constants

### Refactor
- Rename integration test files to <prefix>_<NNN>_test.rs convention

### Documentation
- **readme:** Full public interface reference

### CI
- **publish:** Add crates.io publish + GitHub release workflow on v* tags

### Chores
- Commit baseline docs, requirements, and implementation plan- **cohesion:** End-to-end test, ecosystem re-exports, doc cleanup- **clippy:** Resolve all warnings under -D warnings


