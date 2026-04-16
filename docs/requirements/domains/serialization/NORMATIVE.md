# Serialization - Normative Requirements

> **Domain:** serialization
> **Prefix:** SER
> **Spec reference:** [SPEC.md - Sections 11.1, 11.2, 14.9](../../../resources/SPEC.md)

## Requirements

### SER-001: Bincode Serialization for All Types

All epoch types MUST use bincode for serialization: `EpochInfo`, `EpochSummary`, `DfspCloseSnapshot`, `CheckpointCompetition`, `RewardDistribution`, `EpochCheckpointData`. Bincode produces compact binary output with no schema overhead, making it suitable for high-throughput epoch processing and network transmission. All six types MUST derive `Serialize` + `Deserialize` from `serde`.

**Spec reference:** SPEC Section 11.1

### SER-002: to_bytes and from_bytes Conventions

`to_bytes()` MUST be infallible (panics on failure, which should never happen with well-formed types). `from_bytes()` MUST be fallible, returning an appropriate error on deserialization failure. This matches the dig-block serialization convention.

**Spec reference:** SPEC Section 11.2

### SER-003: Round-Trip Integrity

For all serializable types: `from_bytes(to_bytes(x))` MUST equal `x`. No data loss MUST occur through serialization round-trips. This is a property test requirement covering all 6 serializable types.

**Spec reference:** SPEC Sections 14.9, 14.10
