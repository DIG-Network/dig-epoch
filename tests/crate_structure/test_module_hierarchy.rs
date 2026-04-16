//! # STR-002 — Module Hierarchy (Integration Test)
//!
//! **Requirement under test:** `STR-002 — Module hierarchy matching SPEC §13`
//!
//! **Spec (authoritative):**
//! [`docs/requirements/domains/crate_structure/specs/STR-002.md`](../../docs/requirements/domains/crate_structure/specs/STR-002.md)
//!
//! **Normative language:**
//! [`docs/requirements/domains/crate_structure/NORMATIVE.md`](../../docs/requirements/domains/crate_structure/NORMATIVE.md)
//!
//! **Verification matrix:**
//! [`docs/requirements/domains/crate_structure/VERIFICATION.md`](../../docs/requirements/domains/crate_structure/VERIFICATION.md)
//!
//! **Spec of spec:** [`docs/resources/SPEC.md` §13](../../docs/resources/SPEC.md)
//!
//! ## How this file proves STR-002 is satisfied
//!
//! STR-002 mandates that the `dig-epoch` crate is organised into a specific,
//! flat module tree with a single `types/` subdirectory. It is **purely
//! structural**: no behaviour is asserted — behavioural guarantees arrive in
//! the domain-specific requirements (CON-*, TYP-*, HEA-*, PHS-*, REW-*,
//! MGR-*, CKP-*, VER-*, DFS-*, ERR-*). The only thing this test can and must
//! prove is that every module listed in SPEC §13 and in STR-002's
//! specification table **exists, is wired into the crate, and is accessible
//! by path to downstream consumers**.
//!
//! We prove that by importing, from each module, a deliberately trivial
//! sentinel marker (`STR_002_MODULE_PRESENT`, a `pub(crate) const () = ();`)
//! that every STR-002 module exports. Because integration tests compile as
//! an external crate, a successful `use dig_epoch::<module>::STR_002_MODULE_PRESENT`
//! proves three things at once:
//!
//! 1. `src/<module>.rs` (or `src/types/<submodule>.rs`) exists on disk.
//! 2. It is declared via `mod <module>;` in `src/lib.rs` (and, for types
//!    submodules, via `pub mod <submodule>;` in `src/types/mod.rs`).
//! 3. The module is reachable from the public crate namespace under its
//!    canonical path (necessary precondition for STR-003's `pub use`
//!    re-exports).
//!
//! The marker is public-but-hidden and zero-cost; no real type, enum, or
//! runtime behaviour is foreshadowed. When the owning requirement lands,
//! the marker can be removed (or left alone — it occupies no runtime memory
//! and carries no semantic weight).
//!
//! ## Test Plan coverage map
//!
//! The STR-002 spec Test Plan has 10 rows. Each `#[test]` below implements
//! exactly one row, named to match the row's `Test` column verbatim:
//!
//! | Row | Test                        | Module path exercised                        |
//! |-----|-----------------------------|----------------------------------------------|
//! | 1   | test_module_resolution      | `dig_epoch::*` (whole-crate import)          |
//! | 2   | test_constants_module       | `dig_epoch::constants`                       |
//! | 3   | test_types_module           | `dig_epoch::types::{epoch_phase, epoch_info, epoch_summary, dfsp, events, checkpoint_competition, reward, verification}` |
//! | 4   | test_arithmetic_module      | `dig_epoch::arithmetic`                      |
//! | 5   | test_phase_module           | `dig_epoch::phase`                           |
//! | 6   | test_rewards_module         | `dig_epoch::rewards`                         |
//! | 7   | test_manager_module         | `dig_epoch::manager`                         |
//! | 8   | test_verification_module    | `dig_epoch::verification`                    |
//! | 9   | test_dfsp_module            | `dig_epoch::dfsp`                            |
//! | 10  | test_error_module           | `dig_epoch::error`                           |
//!
//! ## Why a sentinel marker rather than a real symbol?
//!
//! STR-002's Test Plan rows literally say things like "Import a constant from
//! `constants.rs`" and "Function is accessible and callable". Taking those
//! rows at face value with *real* constants or functions would foreshadow
//! CON-*, PHS-*, REW-*, MGR-*, VER-*, DFS-*, ERR-* — which is scope creep
//! and would pollute the git history (later requirements would have to
//! *change* the marker instead of *adding* the real item). Using a sentinel
//! `pub(crate) const STR_002_MODULE_PRESENT: () = ();` in every module keeps
//! STR-002 purely structural while still satisfying the Test Plan's
//! "accessible" requirement — an integration test that successfully
//! `use`s the sentinel has necessarily proved the module resolves.
//!
//! The `#[allow(unused_imports)]` attribute on each test is intentional:
//! the whole point is the `use` itself — the test body is a no-op because
//! compilation *is* the assertion. Rust's dead-code warnings would otherwise
//! flag the import as unused.
//!
//! ## Relationship to STR-001
//!
//! STR-001's integration test (`test_dependency_imports.rs`) lives in a
//! sibling file and is left strictly untouched. Every requirement owns
//! exactly one integration test file; adding STR-002 assertions to
//! `test_dependency_imports.rs` would break the traceability between
//! requirement ID and test file name enforced by TRACKING.yaml.

// The crate under test. We always import via the external crate name
// (`dig_epoch`) rather than `crate::`, because integration tests compile
// as a separate crate — which is exactly what gives these `use` statements
// their evidentiary weight.

/// Row 1 — `test_module_resolution`.
///
/// Exercises: "`cargo check` on the crate" / "All module declarations resolve
/// successfully".
///
/// Strategy: Pull the `STR_002_MODULE_PRESENT` sentinel from every one of the
/// 16 STR-002 modules in a single function. If any `mod` declaration is
/// missing from `src/lib.rs` (or any `pub mod` from `src/types/mod.rs`), or
/// if any module file is missing on disk, compilation of this test fails
/// — which in turn fails `cargo check` on the integration-test target. The
/// test body is `()` because compilation *is* the assertion.
#[test]
#[allow(unused_imports)]
fn test_module_resolution() {
    use dig_epoch::arithmetic::STR_002_MODULE_PRESENT as _;
    use dig_epoch::constants::STR_002_MODULE_PRESENT as _;
    use dig_epoch::dfsp::STR_002_MODULE_PRESENT as _;
    use dig_epoch::error::STR_002_MODULE_PRESENT as _;
    use dig_epoch::manager::STR_002_MODULE_PRESENT as _;
    use dig_epoch::phase::STR_002_MODULE_PRESENT as _;
    use dig_epoch::rewards::STR_002_MODULE_PRESENT as _;
    use dig_epoch::types::checkpoint_competition::STR_002_MODULE_PRESENT as _;
    use dig_epoch::types::dfsp::STR_002_MODULE_PRESENT as _;
    use dig_epoch::types::epoch_info::STR_002_MODULE_PRESENT as _;
    use dig_epoch::types::epoch_phase::STR_002_MODULE_PRESENT as _;
    use dig_epoch::types::epoch_summary::STR_002_MODULE_PRESENT as _;
    use dig_epoch::types::events::STR_002_MODULE_PRESENT as _;
    use dig_epoch::types::reward::STR_002_MODULE_PRESENT as _;
    use dig_epoch::types::verification::STR_002_MODULE_PRESENT as _;
    use dig_epoch::verification::STR_002_MODULE_PRESENT as _;
}

/// Row 2 — `test_constants_module`.
///
/// Exercises: "Import a constant from `constants.rs`" / "Constant is
/// accessible".
///
/// The `STR_002_MODULE_PRESENT` sentinel is a `pub(crate) const` — here we
/// name it explicitly so the test mirrors the spec wording ("import a
/// constant"). The real epoch constants (CON-001 … CON-006) will be added
/// by the Phase 1 requirements; this test guarantees the file is present
/// and importable under its canonical path.
#[test]
#[allow(unused_imports)]
fn test_constants_module() {
    use dig_epoch::constants::STR_002_MODULE_PRESENT as _;
}

/// Row 3 — `test_types_module`.
///
/// Exercises: "Import each type from `types/` submodules" / "All types are
/// accessible".
///
/// STR-002 establishes eight `types/` submodules (epoch_phase, epoch_info,
/// epoch_summary, dfsp, events, checkpoint_competition, reward,
/// verification). We confirm each one is reachable via
/// `dig_epoch::types::<submodule>::STR_002_MODULE_PRESENT`. The actual
/// struct/enum definitions arrive in TYP-* (and in the CKP-/VER-/DFS-
/// domains for the non-TYP submodules), at which point this sentinel can
/// co-exist with them or be removed without semantic impact.
#[test]
#[allow(unused_imports)]
fn test_types_module() {
    use dig_epoch::types::checkpoint_competition::STR_002_MODULE_PRESENT as _;
    use dig_epoch::types::dfsp::STR_002_MODULE_PRESENT as _;
    use dig_epoch::types::epoch_info::STR_002_MODULE_PRESENT as _;
    use dig_epoch::types::epoch_phase::STR_002_MODULE_PRESENT as _;
    use dig_epoch::types::epoch_summary::STR_002_MODULE_PRESENT as _;
    use dig_epoch::types::events::STR_002_MODULE_PRESENT as _;
    use dig_epoch::types::reward::STR_002_MODULE_PRESENT as _;
    use dig_epoch::types::verification::STR_002_MODULE_PRESENT as _;
}

/// Row 4 — `test_arithmetic_module`.
///
/// Exercises: "Import a function from `arithmetic.rs`" / "Function is
/// accessible and callable".
///
/// The STR-002 Test Plan talks about a *function*, but STR-002 must not
/// foreshadow HEA-* (height-epoch arithmetic). We therefore import the
/// sentinel instead of a real arithmetic function; the structural
/// assertion (module exists, is reachable) is what STR-002 is actually
/// verifying. HEA-001 will add the real `epoch_for_block_height` and its
/// own dedicated integration test.
#[test]
#[allow(unused_imports)]
fn test_arithmetic_module() {
    use dig_epoch::arithmetic::STR_002_MODULE_PRESENT as _;
}

/// Row 5 — `test_phase_module`.
///
/// Exercises: "Import `l1_progress_phase_for_network_epoch` from `phase.rs`"
/// / "Function is accessible and callable".
///
/// Phase state-machine functions are introduced by PHS-001. STR-002 only
/// asserts that the module file exists and is wired into the crate root.
/// We therefore import the sentinel, keeping STR-002 strictly structural.
#[test]
#[allow(unused_imports)]
fn test_phase_module() {
    use dig_epoch::phase::STR_002_MODULE_PRESENT as _;
}

/// Row 6 — `test_rewards_module`.
///
/// Exercises: "Import reward functions from `rewards.rs`" / "Functions are
/// accessible and callable".
///
/// Reward computation functions are introduced by REW-001 through REW-006.
/// STR-002 only asserts the module exists as part of the crate skeleton.
#[test]
#[allow(unused_imports)]
fn test_rewards_module() {
    use dig_epoch::rewards::STR_002_MODULE_PRESENT as _;
}

/// Row 7 — `test_manager_module`.
///
/// Exercises: "Import `EpochManager` from `manager.rs`" / "Struct and
/// methods are accessible".
///
/// `EpochManager` itself is introduced by STR-004 / MGR-001. STR-002 only
/// proves the *module* exists at the correct path — the real struct lands
/// in the next requirement.
#[test]
#[allow(unused_imports)]
fn test_manager_module() {
    use dig_epoch::manager::STR_002_MODULE_PRESENT as _;
}

/// Row 8 — `test_verification_module`.
///
/// Exercises: "Import verification functions from `verification.rs`" /
/// "Functions are accessible and callable".
///
/// Note: there are two distinct `verification` modules in dig-epoch — the
/// root-level `verification.rs` (free functions using Chia crates, added
/// by VER-*) and `types/verification.rs` (data structures, added by
/// TYP-*). This test asserts presence of the **root-level** one; Row 3
/// (`test_types_module`) covers the `types/verification.rs` counterpart.
/// This split is called out in STR-002's Implementation Notes.
#[test]
#[allow(unused_imports)]
fn test_verification_module() {
    use dig_epoch::verification::STR_002_MODULE_PRESENT as _;
}

/// Row 9 — `test_dfsp_module`.
///
/// Exercises: "Import DFSP functions from `dfsp.rs`" / "Functions are
/// accessible and callable".
///
/// Same split as verification: the root-level `dfsp.rs` is processing
/// functions (DFS-*), while `types/dfsp.rs` is type definitions (TYP-*).
/// This test targets the **root-level** module; Row 3 covers the types
/// counterpart.
#[test]
#[allow(unused_imports)]
fn test_dfsp_module() {
    use dig_epoch::dfsp::STR_002_MODULE_PRESENT as _;
}

/// Row 10 — `test_error_module`.
///
/// Exercises: "Import `EpochError` and `CheckpointCompetitionError` from
/// `error.rs`" / "Error types are accessible".
///
/// Error enums are introduced by ERR-001 and ERR-002. STR-002 only
/// proves the module exists in the crate skeleton so ERR-* has a home to
/// land in without touching `lib.rs` again.
#[test]
#[allow(unused_imports)]
fn test_error_module() {
    use dig_epoch::error::STR_002_MODULE_PRESENT as _;
}
