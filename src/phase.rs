//! # `phase` — L1-progress phase calculation
//!
//! **Introduced by:** `STR-002` — Module hierarchy (SPEC §13).
//!
//! **Future owners:** Phase 5 of `IMPLEMENTATION_ORDER.md`:
//!
//! - [`PHS-001`](../../docs/requirements/domains/phase_state_machine/specs/PHS-001.md)
//!   — `l1_progress_phase_for_network_epoch()` free function
//! - [`PHS-002`](../../docs/requirements/domains/phase_state_machine/specs/PHS-002.md)
//!   — [`crate::manager::EpochManager`] phase tracking
//! - [`PHS-003`](../../docs/requirements/domains/phase_state_machine/specs/PHS-003.md)
//!   — transition events and `should_advance()`
//! - [`PHS-004`](../../docs/requirements/domains/phase_state_machine/specs/PHS-004.md)
//!   — phase-boundary enforcement (`PhaseMismatch` errors)
//!
//! **Spec reference:**
//! [`SPEC.md` §13](../../docs/resources/SPEC.md) — canonical module list;
//! [`SPEC.md` §4](../../docs/resources/SPEC.md) — phase state machine.
//!
//! ## Content rule
//!
//! This module owns the *free-function* surface of the phase state
//! machine — stateless calculation driven by the L1 height relative to
//! the network-epoch window. The `EpochPhase` enum and
//! `PhaseTransition` struct live in [`crate::types::epoch_phase`] because
//! STR-002 splits **types** (in `types/`) from **functions** (at the crate
//! root).
//!
//! Per start.md Hard Requirement 9, phase calculation is driven by L1
//! height alone — never by wall-clock time. This invariant is structural
//! (there is no clock dependency to import) and must be preserved by
//! every PHS-* requirement.
//!
//! ## Status at STR-002
//!
//! Empty aside from the [`STR_002_MODULE_PRESENT`] sentinel.

/// Sentinel marker proving the module exists and is reachable at
/// `dig_epoch::phase::STR_002_MODULE_PRESENT`.
///
/// Exercised by the STR-002 integration test — see
/// [`tests/crate_structure/str_002_test.rs`](../../tests/crate_structure/str_002_test.rs).
#[doc(hidden)]
pub const STR_002_MODULE_PRESENT: () = ();
