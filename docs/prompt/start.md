# Start

## Immediate Actions

1. **Sync**
   ```bash
   git fetch origin && git pull origin main
   ```

2. **Check tools — ALL THREE MUST BE FRESH**
   ```bash
   npx gitnexus status          # GitNexus index fresh?
   npx gitnexus analyze         # Update if stale
   codebase_status {}            # SocratiCode MCP status
   ```
   **Do not proceed until tools are confirmed operational.**

3. **Pick work** — open `docs/requirements/IMPLEMENTATION_ORDER.md`
   - Choose the first `- [ ]` item
   - Work phases in order: Phase 0 before Phase 1, etc.

4. **Pack context — BEFORE reading any code**
   ```bash
   npx repomix@latest src -o .repomix/pack-src.xml
   npx repomix@latest tests -o .repomix/pack-tests.xml
   ```

5. **Read spec** — follow the full trace:
   - `NORMATIVE.md#PREFIX-NNN` → authoritative requirement
   - `specs/PREFIX-NNN.md` → detailed specification + **test plan**
   - `VERIFICATION.md` → how to verify
   - `TRACKING.yaml` → current status

6. **Continue** → [dt-wf-select.md](tree/dt-wf-select.md)

---

## Hard Requirements

1. **Checkpoint/block types from dig-block** — never redefine Checkpoint, CheckpointSubmission, SignerBitmap, L2BlockHeader.
2. **Network constants from dig-constants** — use NetworkConstants directly.
3. **Bytes32 from chia-protocol** — all hash types.
4. **BLS from chia-bls** — Signature, PublicKey, aggregate(), aggregate_verify().
5. **Merkle roots from chia-consensus** — compute_merkle_set_root() for withdrawals.
6. **Merkle trees from chia-sdk-types** — MerkleTree/MerkleProof for block roots.
7. **SHA-256 from chia-sha2** — all hashing.
8. **Reward constants are compile-time** — no runtime configuration for economics.
9. **L1 height drives phase, not wall-clock time** — objective, consensus-observable.
10. **Checkpoint blocks must be empty** — zero SpendBundles, cost, fees.
11. **Height 1 is genesis** — not height 0.
12. **EpochManager uses interior mutability** — parking_lot::RwLock.
13. **TEST FIRST (TDD)** — write the failing test before implementation.
14. **One requirement per commit** — don't batch unrelated work.
15. **Update tracking after each requirement** — VERIFICATION.md, TRACKING.yaml, IMPLEMENTATION_ORDER.md.

---

## Tech Stack

| Component | Crate | Version |
|-----------|-------|---------|
| Block types | `dig-block` | 0.1 |
| Network constants | `dig-constants` | 0.1 |
| Hash type | `chia-protocol` | 0.26 |
| BLS cryptography | `chia-bls` | 0.26 |
| Merkle set roots | `chia-consensus` | 0.26 |
| Merkle trees | `chia-sdk-types` | 0.30 |
| Signature constants | `chia-sdk-signer` | 0.30 |
| SHA-256 | `chia-sha2` | 0.26 |
| CLVM tree hash | `clvm-utils` | 0.26 |
| Serialization | `bincode` | latest |
| Serde framework | `serde` | 1 |
| Error derivation | `thiserror` | latest |
| Concurrency | `parking_lot` | latest |
