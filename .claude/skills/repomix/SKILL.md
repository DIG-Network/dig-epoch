# Repomix — Context Packing Skill

## When to Use

Use Repomix **before implementing any requirement**. Pack the relevant scope so the LLM has full awareness of the code being modified.

## HARD RULE

**MUST pack context before writing implementation code.** Fresh context prevents redundant work and missed patterns.

## Commands

### Pack Implementation

```bash
npx repomix@latest src -o .repomix/pack-src.xml
```

### Pack Tests (CRITICAL for TDD)

```bash
npx repomix@latest tests -o .repomix/pack-tests.xml
```

### Pack Requirements by Domain

```bash
# checkpoint_competition
npx repomix@latest docs/requirements/domains/checkpoint_competition -o .repomix/pack-checkpoint-competition-reqs.xml

# constants
npx repomix@latest docs/requirements/domains/constants -o .repomix/pack-constants-reqs.xml

# crate_structure
npx repomix@latest docs/requirements/domains/crate_structure -o .repomix/pack-crate-structure-reqs.xml

# dfsp_processing
npx repomix@latest docs/requirements/domains/dfsp_processing -o .repomix/pack-dfsp-processing-reqs.xml

# epoch_manager
npx repomix@latest docs/requirements/domains/epoch_manager -o .repomix/pack-epoch-manager-reqs.xml

# epoch_types
npx repomix@latest docs/requirements/domains/epoch_types -o .repomix/pack-epoch-types-reqs.xml

# error_types
npx repomix@latest docs/requirements/domains/error_types -o .repomix/pack-error-types-reqs.xml

# height_arithmetic
npx repomix@latest docs/requirements/domains/height_arithmetic -o .repomix/pack-height-arithmetic-reqs.xml

# phase_machine
npx repomix@latest docs/requirements/domains/phase_machine -o .repomix/pack-phase-machine-reqs.xml

# reward_economics
npx repomix@latest docs/requirements/domains/reward_economics -o .repomix/pack-reward-economics-reqs.xml

# serialization
npx repomix@latest docs/requirements/domains/serialization -o .repomix/pack-serialization-reqs.xml

# verification
npx repomix@latest docs/requirements/domains/verification -o .repomix/pack-verification-reqs.xml

# All requirements at once
npx repomix@latest docs/requirements -o .repomix/pack-requirements.xml
```

### Pack the Full Spec

```bash
npx repomix@latest docs/resources -o .repomix/pack-spec.xml
```

### Pack with Compression

```bash
npx repomix@latest src --compress -o .repomix/pack-src-compressed.xml
```

### Pack Multiple Scopes

```bash
npx repomix@latest src tests -o .repomix/pack-impl-and-tests.xml
```

## Workflow Integration

| Step | Pack Command |
|------|-------------|
| Before writing tests | `npx repomix@latest tests -o .repomix/pack-tests.xml` |
| Before implementing | `npx repomix@latest src -o .repomix/pack-src.xml` |
| Cross-domain work | Pack both domains' requirements |

## Notes

- `.repomix/` is gitignored — pack files are never committed
- Regenerate packs when switching requirements
- Use `--compress` for large scopes to manage token count
- Pack requirements alongside code for spec compliance checks

## Full Documentation

See `docs/prompt/tools/repomix.md` for complete reference.
