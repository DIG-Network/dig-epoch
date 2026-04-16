# Error Types - Normative Requirements

> **Domain:** error_types
> **Prefix:** ERR
> **Spec reference:** [SPEC.md - Section 10](../../../resources/SPEC.md)

## Requirements

### ERR-001: EpochError Enum

EpochError MUST define the following 9 variants using `thiserror::Error` derivation:

- `EpochNotComplete(u64)` — Attempted to advance an epoch that hasn't reached Complete phase. Message: `"Cannot advance: epoch {0} is not complete"`.
- `NoFinalizedCheckpoint(u64)` — Attempted to advance an epoch with no finalized checkpoint. Message: `"Cannot advance: epoch {0} has no finalized checkpoint"`.
- `CheckpointBlockNotEmpty(u64, u32, u64, u64)` — Checkpoint-class block contains non-zero SpendBundles, cost, or fees. Message: `"Checkpoint block at height {0} is not empty: {1} bundles, {2} cost, {3} fees"`.
- `PhaseMismatch { expected: EpochPhase, got: EpochPhase }` — Operation requires a specific phase but the epoch is in a different one. Message: `"Phase mismatch: expected {expected}, got {got}"`.
- `EpochMismatch { expected: u64, got: u64 }` — Submission or query references the wrong epoch. Message: `"Epoch mismatch: expected {expected}, got {got}"`.
- `InvalidHeight(u64)` — L2 height is below genesis (height 0 or underflow). Message: `"Invalid height {0}: below genesis"`.
- `DfspNotActive(u64)` — DFSP operation attempted at a height before activation. Message: `"DFSP not active at height {0}"`.
- `DfspBoundary(String)` — DFSP epoch-boundary processing error. Message: `"DFSP epoch-boundary error: {0}"`.
- `Competition(CheckpointCompetitionError)` — Checkpoint competition error (delegated via `#[from]`). Message: `"Competition error: {0}"`.

EpochError MUST derive Debug, Clone, and implement `thiserror::Error`.

**Spec reference:** SPEC Section 10.1

### ERR-002: CheckpointCompetitionError Enum

CheckpointCompetitionError MUST define the following 6 variants using `thiserror::Error` derivation:

- `InvalidData(String)` — Checkpoint data failed validation. Message: `"Invalid checkpoint data: {0}"`.
- `NotFound(u64)` — No competition exists for the requested epoch. Message: `"Checkpoint competition not found for epoch {0}"`.
- `ScoreNotHigher { current: u64, submitted: u64 }` — Submitted checkpoint's score does not exceed the current leader. Message: `"Score not higher: current {current}, submitted {submitted}"`.
- `EpochMismatch { expected: u64, got: u64 }` — Submission's epoch field doesn't match the competition's epoch. Message: `"Epoch mismatch: expected {expected}, got {got}"`.
- `AlreadyFinalized` — Competition has already been finalized; no more submissions accepted. Message: `"Competition already finalized"`.
- `NotStarted` — Competition hasn't been started yet (still in Pending state). Message: `"Competition not started"`.

CheckpointCompetitionError MUST derive Debug, Clone, and implement `thiserror::Error`.

**Spec reference:** SPEC Section 10.2

### ERR-003: Error Conversions and Display

EpochError MUST implement `From<CheckpointCompetitionError>` via the `#[from]` attribute on the `Competition` variant. This allows `?` operator propagation from competition errors into epoch errors.

Both EpochError and CheckpointCompetitionError MUST implement `Display` via `thiserror::Error` `#[error(...)]` attributes. Each variant's display message MUST match the messages specified in ERR-001 and ERR-002.

**Spec reference:** SPEC Section 10
