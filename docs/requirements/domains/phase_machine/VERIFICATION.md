# Phase Machine - Verification Matrix

| Field       | Value         |
|-------------|---------------|
| Domain      | phase_machine |
| Prefix      | PHS           |
| Total Items | 4             |
| Status      | Draft         |
| Spec        | [SPEC.md](../../../resources/SPEC.md) |

| ID      | Status  | Summary                          | Verification Approach                                                        |
|---------|---------|----------------------------------|------------------------------------------------------------------------------|
| PHS-001 | Pending | L1 Progress Phase Calculation    | Unit test all four phase boundaries; verify determinism with identical inputs; test edge cases at exact boundary percentages (0%, 50%, 75%, 100%) |
| PHS-002 | Pending | EpochManager Phase Tracking      | Unit test current_phase() returns EpochInfo.phase; test update_phase() returns Some(PhaseTransition) on change and None when unchanged |
| PHS-003 | Pending | Phase Transition Events          | Unit test should_advance() returns true only for Complete; verify PhaseTransition fields; verify EpochEvent::PhaseChanged emission |
| PHS-004 | Pending | Phase Boundary Enforcement       | Unit test record_block rejected outside BlockProduction; submit_checkpoint rejected outside Checkpoint; finalize_competition rejected outside Finalization; verify PhaseMismatch error contains expected and got |
