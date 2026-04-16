# Error Types - Verification Matrix

> **Domain:** error_types
> **Prefix:** ERR
> **Normative:** [NORMATIVE.md](./NORMATIVE.md)
> **Tracking:** [TRACKING.yaml](./TRACKING.yaml)

| ID | Status | Summary | Verification Approach |
|----|--------|---------|----------------------|
| ERR-001 | gap | EpochError Enum | Unit test: construct all 9 EpochError variants (EpochNotComplete, NoFinalizedCheckpoint, CheckpointBlockNotEmpty, PhaseMismatch, EpochMismatch, InvalidHeight, DfspNotActive, DfspBoundary, Competition). Verify Debug and Clone derives. Verify Display output matches specified error messages. Verify pattern matching on each variant extracts correct fields. |
| ERR-002 | gap | CheckpointCompetitionError Enum | Unit test: construct all 6 CheckpointCompetitionError variants (InvalidData, NotFound, ScoreNotHigher, EpochMismatch, AlreadyFinalized, NotStarted). Verify Debug and Clone derives. Verify Display output matches specified error messages. Verify pattern matching on each variant extracts correct fields. |
| ERR-003 | gap | Error Conversions and Display | Unit test: verify From<CheckpointCompetitionError> for EpochError converts to Competition variant. Verify ? operator propagation works. Verify Display implementations for both error types produce correct messages for all variants. |
