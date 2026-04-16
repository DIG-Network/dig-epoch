# dig-epoch Requirements

This directory contains the formal requirements for the dig-epoch crate,
following the same two-tier requirements structure as dig-gossip, dig-block,
and dig-blockstore with full traceability.

## Quick Links

- [SCHEMA.md](SCHEMA.md) — Data model and conventions
- [REQUIREMENTS_REGISTRY.yaml](REQUIREMENTS_REGISTRY.yaml) — Central domain registry
- [domains/](domains/) — All requirement domains

## Structure

```
requirements/
├── README.md                    # This file
├── SCHEMA.md                    # Data model and conventions
├── REQUIREMENTS_REGISTRY.yaml   # Central registry
├── IMPLEMENTATION_ORDER.md      # Phased implementation checklist
└── domains/
    ├── crate_structure/         # STR-* Crate layout, dependencies, test infra
    ├── constants/               # CON-* Epoch geometry, phases, rewards, DFSP, consensus
    ├── epoch_types/             # TYP-* EpochPhase, EpochInfo, EpochSummary, events
    ├── height_arithmetic/       # HEA-* Height-epoch conversions, checkpoint detection
    ├── phase_machine/           # PHS-* L1-driven phase state machine, transitions
    ├── epoch_manager/           # MGR-* EpochManager, block recording, advancement
    ├── checkpoint_competition/  # CKP-* Competition lifecycle, submissions, scoring
    ├── reward_economics/        # REW-* Block rewards, halvings, fee splits, distribution
    ├── verification/            # VER-* Block roots, inclusion proofs, checkpoint signing
    ├── dfsp_processing/         # DFS-* DFSP 7-stage pipeline, burn policy, roots
    ├── error_types/             # ERR-* EpochError, CheckpointCompetitionError
    └── serialization/           # SER-* Bincode format, round-trips
```

## Three-Document Pattern

Each domain contains:

| File | Purpose |
|------|---------|
| `NORMATIVE.md` | Authoritative requirement statements (MUST/SHOULD/MAY) |
| `VERIFICATION.md` | QA approach and status per requirement |
| `TRACKING.yaml` | Machine-readable status, tests, and notes |

## Reference Document

All requirements are derived from:
- [SPEC.md](../resources/SPEC.md) — dig-epoch specification

## Requirement Count

| Domain | Prefix | Count |
|--------|--------|-------|
| Crate Structure | STR | 5 |
| Constants | CON | 5 |
| Epoch Types | TYP | 6 |
| Height-Epoch Arithmetic | HEA | 5 |
| Phase State Machine | PHS | 4 |
| Epoch Manager | MGR | 7 |
| Checkpoint Competition | CKP | 5 |
| Reward Economics | REW | 6 |
| Verification | VER | 5 |
| DFSP Processing | DFS | 6 |
| Error Types | ERR | 3 |
| Serialization | SER | 3 |
| **Total** | | **60** |
