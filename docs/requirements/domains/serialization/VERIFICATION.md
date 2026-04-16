# Serialization - Verification Matrix

> **Domain:** serialization
> **Prefix:** SER
> **Normative:** [NORMATIVE.md](./NORMATIVE.md)
> **Tracking:** [TRACKING.yaml](./TRACKING.yaml)

| ID | Status | Summary | Verification Approach |
|----|--------|---------|----------------------|
| SER-001 | gap | Bincode Serialization for All Types | Unit test: verify each type (EpochInfo, EpochSummary, DfspCloseSnapshot, CheckpointCompetition, RewardDistribution, EpochCheckpointData) serializes with bincode. Verify output is compact binary (no JSON/schema overhead). Code review: confirm bincode is the only serialization backend. Verify all six types derive Serialize + Deserialize. |
| SER-002 | gap | to_bytes and from_bytes Conventions | Unit test: to_bytes() returns Vec<u8> without error for valid types. from_bytes() with valid bytes returns Ok. from_bytes() with invalid/truncated bytes returns Err(EpochError). Verify to_bytes does not return Result (infallible). Verify from_bytes returns Result<Self, EpochError>. |
| SER-003 | gap | Round-Trip Integrity | Property test: for each of the 6 serializable types, generate random instances and verify from_bytes(to_bytes(x)) == x. Use proptest with Arbitrary implementations. Cover edge cases: empty optional fields, maximum values, zero values, None variants. |
