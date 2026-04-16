# Checkpoint Competition - Verification Matrix

> **Domain:** checkpoint_competition
> **Prefix:** CKP
> **Normative:** [NORMATIVE.md](./NORMATIVE.md)
> **Tracking:** [TRACKING.yaml](./TRACKING.yaml)

| ID | Status | Summary | Verification Approach |
|----|--------|---------|----------------------|
| CKP-001 | gap | CheckpointCompetition and CompetitionStatus | Unit test: construct CheckpointCompetition with all fields; verify CompetitionStatus variants Pending, Collecting, WinnerSelected { winner_hash, winner_score }, Finalized { winner_hash, l1_height }, Failed. Verify current_winner is Option<usize>. |
| CKP-002 | gap | start_checkpoint_competition | Unit test: call start_checkpoint_competition in Pending state, verify transition to Collecting. Test PhaseMismatch when phase != Checkpoint. Test rejection when status is not Pending. |
| CKP-003 | gap | submit_checkpoint with Score | Unit test: submit checkpoint with score > current winner, verify current_winner updated. Test score = stake_percentage * block_count computation. Test ScoreNotHigher rejection when score <= current. Test epoch mismatch rejection. Test status must be Collecting. |
| CKP-004 | gap | finalize_competition | Unit test: call finalize_competition from WinnerSelected, verify transition to Finalized with correct winner_hash and l1_height. Verify checkpoint set on EpochInfo. Verify get_competition returns current state. |
| CKP-005 | gap | Competition Lifecycle | Integration test: exercise full lifecycle Pending -> Collecting -> WinnerSelected -> Finalized. Test Pending -> Collecting -> WinnerSelected -> Failed path. Verify invalid transitions are rejected at each state. |
