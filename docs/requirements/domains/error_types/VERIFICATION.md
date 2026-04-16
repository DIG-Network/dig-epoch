# Error Types - Verification Matrix

> **Domain:** error_types
> **Prefix:** ERR
> **Normative:** [NORMATIVE.md](./NORMATIVE.md)
> **Tracking:** [TRACKING.yaml](./TRACKING.yaml)

| ID | Status | Summary | Verification Approach |
|----|--------|---------|----------------------|
| ERR-001 | verified | EpochError Enum | Unit test: construct all 9 EpochError variants (EpochNotComplete, NoFinalizedCheckpoint, CheckpointBlockNotEmpty, PhaseMismatch, EpochMismatch, InvalidHeight, DfspNotActive, DfspBoundary, Competition). Verify Debug and Clone derives. Verify Display output matches specified error messages. Verify pattern matching on each variant extracts correct fields. **Evidence:** `tests/error_types/err_001_test.rs` (12/12 passing, 2026-04-16). |
| ERR-002 | verified | CheckpointCompetitionError Enum | Unit test: construct all 6 CheckpointCompetitionError variants (InvalidData, NotFound, ScoreNotHigher, EpochMismatch, AlreadyFinalized, NotStarted). Verify Debug and Clone derives. Verify Display output matches specified error messages. Verify pattern matching on each variant extracts correct fields. **Evidence:** `tests/error_types/err_002_test.rs` (9/9 passing, 2026-04-16). |
| ERR-003 | verified | Error Conversions and Display | Unit test: verify From<CheckpointCompetitionError> for EpochError converts to Competition variant. Verify ? operator propagation works. Verify Display implementations for both error types produce correct messages for all variants. **Evidence:** `tests/error_types/err_003_test.rs` (7/7 passing, 2026-04-16). |
