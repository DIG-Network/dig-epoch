# Requirements Schema

This document defines the data model and conventions for all requirements in the
dig-epoch project.

---

## Three-Document Pattern

Each domain has exactly three files in `docs/requirements/domains/{domain}/`:

| File | Purpose |
|------|---------|
| `NORMATIVE.md` | Authoritative requirement statements with MUST/SHOULD/MAY keywords |
| `VERIFICATION.md` | QA approach and verification status per requirement |
| `TRACKING.yaml` | Machine-readable status, test references, and implementation notes |

Each requirement also has a dedicated specification file in
`docs/requirements/domains/{domain}/specs/{PREFIX-NNN}.md`.

---

## Requirement ID Format

**Pattern:** `{PREFIX}-{NNN}`

| Domain | Directory | Prefix | Description |
|--------|-----------|--------|-------------|
| Crate Structure | `crate_structure/` | `STR` | Cargo.toml, module hierarchy, re-exports, test infrastructure |
| Constants | `constants/` | `CON` | Epoch geometry, phase boundaries, reward economics, DFSP, consensus |
| Epoch Types | `epoch_types/` | `TYP` | EpochPhase, EpochInfo, EpochSummary, DfspCloseSnapshot, events |
| Height-Epoch Arithmetic | `height_arithmetic/` | `HEA` | Height-epoch conversions, checkpoint detection, boundary guards |
| Phase State Machine | `phase_machine/` | `PHS` | L1-driven phase calculation, transitions, enforcement |
| Epoch Manager | `epoch_manager/` | `MGR` | EpochManager, block recording, advancement, queries |
| Checkpoint Competition | `checkpoint_competition/` | `CKP` | Competition lifecycle, submissions, scoring, finalization |
| Reward Economics | `reward_economics/` | `REW` | Block rewards, halvings, fee splits, epoch distribution |
| Verification | `verification/` | `VER` | Block roots, inclusion proofs, checkpoint signing, aggregation |
| DFSP Processing | `dfsp_processing/` | `DFS` | 7-stage pipeline, burn policy, roots, activation control |
| Error Types | `error_types/` | `ERR` | EpochError, CheckpointCompetitionError |
| Serialization | `serialization/` | `SER` | Bincode format, round-trips |

**Immutability:** Requirement IDs are permanent. Deprecate rather than renumber.

---

## Requirement Keywords (RFC 2119)

| Keyword | Meaning | Impact |
|---------|---------|--------|
| **MUST** | Absolute requirement | Blocks "done" status |
| **MUST NOT** | Absolute prohibition | Blocks "done" status |
| **SHOULD** | Expected; may defer with rationale | Phase 2+ polish |
| **MAY** | Optional, nice-to-have | Stretch goals |

---

## Status Values

| Status | Description |
|--------|-------------|
| `gap` | Not implemented |
| `partial` | In progress or incomplete |
| `implemented` | Code complete, awaiting verification |
| `verified` | Implemented and verified |
| `deferred` | Explicitly postponed with rationale |

---

## Testing Requirements

### 1. Unit Tests (MUST)
All epoch types, arithmetic, phase, reward, and manager paths MUST be tested.

### 2. Integration Tests (MUST for multi-domain)
Full epoch lifecycle, multi-epoch advancement, checkpoint competition with BLS.

### 3. Property Tests (SHOULD)
Height-epoch round-trips, phase monotonicity, reward non-negativity, determinism.

### 4. Required Test Infrastructure

```toml
[dev-dependencies]
tempfile = "3"
rand = "0.8"
```

```rust
use dig_epoch::{EpochManager, EpochPhase, EpochInfo, EpochSummary};
use dig_epoch::{CheckpointCompetition, CompetitionStatus, RewardDistribution};
use dig_epoch::{BLOCKS_PER_EPOCH, EPOCH_L1_BLOCKS, GENESIS_HEIGHT};
use chia_protocol::Bytes32;
use chia_bls::{Signature, PublicKey};
```

---

## Master Spec Reference

[SPEC.md](../resources/SPEC.md)
