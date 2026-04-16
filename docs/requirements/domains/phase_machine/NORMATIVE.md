# Phase Machine - Normative Requirements

| Field       | Value         |
|-------------|---------------|
| Domain      | phase_machine |
| Prefix      | PHS           |
| Total Items | 4             |
| Status      | Draft         |
| Spec        | [SPEC.md](../../../resources/SPEC.md) |

## Requirements

### PHS-001: L1 Progress Phase Calculation

`l1_progress_phase_for_network_epoch(genesis_l1_height: u32, epoch: u64, current_l1_height: u32) -> EpochPhase` MUST compute L1 progress as a percentage of the epoch's L1 window and return the corresponding phase:

- `< PHASE_BLOCK_PRODUCTION_END_PCT` (50%) → `BlockProduction`
- `>= PHASE_BLOCK_PRODUCTION_END_PCT` and `< PHASE_CHECKPOINT_END_PCT` (75%) → `Checkpoint`
- `>= PHASE_CHECKPOINT_END_PCT` and `< PHASE_FINALIZATION_END_PCT` (100%) → `Finalization`
- `>= PHASE_FINALIZATION_END_PCT` (100%) → `Complete`

The function is pure and deterministic: given the same inputs, it MUST always return the same result. No wall-clock time is used.

**Spec Reference:** Section 4.2

### PHS-002: EpochManager Phase Tracking

`EpochManager` MUST provide the following phase tracking methods:

- `current_phase() -> EpochPhase` — Returns the current `EpochInfo.phase`.
- `update_phase(l1_height: u32)` — MUST recalculate the phase from the given L1 height and update `EpochInfo.phase`. MUST return `Option<PhaseTransition>` — `Some(PhaseTransition)` if the phase changed, `None` otherwise.

**Spec Reference:** Section 6.3

### PHS-003: Phase Transition Events

`should_advance(l1_height: u32) -> bool` MUST return `true` when the current phase is `Complete`.

`PhaseTransition` MUST record the following fields for each transition:

- `epoch`: the epoch number
- `from`: the previous `EpochPhase`
- `to`: the new `EpochPhase`
- `l1_height`: the L1 height at which the transition occurred

`EpochEvent::PhaseChanged` MUST be emitted on every phase transition.

**Spec Reference:** Section 4.3

### PHS-004: Phase Boundary Enforcement

Operations that require specific phases MUST reject with `EpochError::PhaseMismatch { expected, got }` when called in the wrong phase:

- `record_block` MUST require `BlockProduction` phase.
- `submit_checkpoint` MUST require `Checkpoint` phase.
- `finalize_competition` MUST require `Finalization` phase.

**Spec Reference:** Section 4.1
