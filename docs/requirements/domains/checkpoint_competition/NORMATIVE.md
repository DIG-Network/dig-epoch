# Checkpoint Competition - Normative Requirements

> **Domain:** checkpoint_competition
> **Prefix:** CKP
> **Spec reference:** [SPEC.md - Sections 3.9, 3.10, 6.5](../../../resources/SPEC.md)

## Requirements

### CKP-001: CheckpointCompetition and CompetitionStatus

`CheckpointCompetition` MUST contain the following fields:

- `epoch`: `u64`
- `submissions`: `Vec<CheckpointSubmission>`
- `status`: `CompetitionStatus`
- `current_winner`: `Option<usize>`

`CompetitionStatus` MUST define the following variants:

- `Pending`
- `Collecting`
- `WinnerSelected { winner_hash: Bytes32, winner_score: u64 }`
- `Finalized { winner_hash: Bytes32, l1_height: u32 }`
- `Failed`

**Spec reference:** SPEC Section 3.9, 3.10

### CKP-002: start_checkpoint_competition

`start_checkpoint_competition(&self) -> Result<()>` MUST transition competition from `Pending` to `Collecting`.

- MUST require `phase == Checkpoint` (returns `PhaseMismatch` otherwise).
- MUST reject if already started (status is not `Pending`).

**Spec reference:** SPEC Section 6.5

### CKP-003: submit_checkpoint with Score

`submit_checkpoint(&self, submission: CheckpointSubmission) -> Result<()>` MUST:

- Verify `status == Collecting`.
- Verify `epoch` matches the current competition epoch.
- Compute `score = stake_percentage * block_count`.
- If `score > current_winner_score`, update `current_winner` and transition to `WinnerSelected`.
- Reject with `ScoreNotHigher` if not beating the current leader.

**Spec reference:** SPEC Section 6.5

### CKP-004: finalize_competition

`finalize_competition(&self, l1_height: u32) -> Result<Bytes32>` MUST:

- Transition from `WinnerSelected` to `Finalized { winner_hash, l1_height }`.
- Set checkpoint on current `EpochInfo`.
- `get_competition()` returns the current competition state.

**Spec reference:** SPEC Section 6.5

### CKP-005: Competition Lifecycle

Full lifecycle MUST follow this state machine:

- `Pending` (created with epoch)
- `Collecting` (via `start_checkpoint_competition`)
- `WinnerSelected` (first valid submission or higher score)
- `Finalized` (via `finalize_competition`) or `Failed` (timeout/error)

Each transition MUST be validated. Invalid transitions MUST be rejected.

**Spec reference:** SPEC Section 3.10
