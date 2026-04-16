//! # STR-001 — Cargo.toml Dependencies (Integration Test)
//!
//! **Requirement under test:** `STR-001 — Cargo.toml dependencies`
//!
//! **Spec (authoritative):**
//! [`docs/requirements/domains/crate_structure/specs/STR-001.md`](../../docs/requirements/domains/crate_structure/specs/STR-001.md)
//!
//! **Normative language:**
//! [`docs/requirements/domains/crate_structure/NORMATIVE.md`](../../docs/requirements/domains/crate_structure/NORMATIVE.md)
//!
//! **Verification matrix:**
//! [`docs/requirements/domains/crate_structure/VERIFICATION.md`](../../docs/requirements/domains/crate_structure/VERIFICATION.md)
//!
//! ## How this file proves STR-001 is satisfied
//!
//! STR-001 mandates a specific set of 13 dependencies pinned to specific versions
//! (DIG crates at `0.1`, Chia low-level crates at `0.26`, Chia SDK crates at `0.30`,
//! plus utility crates). It also mandates that `serde` is enabled with the `derive`
//! feature.
//!
//! This test file verifies the requirement along two complementary axes:
//!
//! 1. **Textual presence / version pinning** — Static assertions over the crate's
//!    `Cargo.toml` confirm each required dependency is declared with the exact
//!    version pin the spec calls out. We read `Cargo.toml` at test time via
//!    `env!("CARGO_MANIFEST_DIR")` so the file under inspection is the one we
//!    actually build against. This guards against accidental version drift
//!    (e.g., a well-meaning upgrade of `chia-bls` from `0.26` to `0.27`).
//! 2. **Link-time resolvability / import semantics** — Each test that references
//!    a foreign type (e.g., `chia_bls::Signature`, `parking_lot::RwLock`) can
//!    only compile if the dependency graph resolves AND the crate exposes the
//!    named item. Compilation of this integration test therefore doubles as a
//!    live `cargo check` over the declared dependency set. This is the same
//!    strategy STR-001's "Test Plan" table refers to as a "Build" test.
//!
//! Together the textual and compilation-level checks give us high confidence
//! that STR-001's acceptance criteria are met:
//! - Cargo.toml contains all 13 listed dependencies.
//! - DIG crates are pinned to `0.1`.
//! - Chia crates are pinned to the specified versions (`0.26` or `0.30`).
//! - `serde` includes the `derive` feature.
//! - `cargo check` completes successfully with all dependencies resolved.
//! - No dependency is duplicated or unnecessary (each is exercised here).
//!
//! ## Test Plan coverage map
//!
//! Each `#[test]` below is tagged with the row of the STR-001 spec Test Plan
//! it implements. The Test Plan table is reproduced here for quick reference:
//!
//! | Row | Test                        | Purpose                                   |
//! |-----|-----------------------------|-------------------------------------------|
//! | 1   | `test_cargo_check`          | `cargo check` succeeds                    |
//! | 2   | `test_dig_block_import`     | `dig_block::Checkpoint/CheckpointSubmission` usable |
//! | 3   | `test_chia_protocol_import` | `chia_protocol::Bytes32` usable           |
//! | 4   | `test_chia_bls_import`      | `chia_bls::Signature`, `PublicKey` usable |
//! | 5   | `test_chia_sdk_types_import`| `chia_sdk_types::MerkleTree`, `MerkleProof` usable |
//! | 6   | `test_serde_derive`         | `#[derive(Serialize, Deserialize)]` works |
//! | 7   | `test_parking_lot_import`   | `parking_lot::RwLock` usable              |
//!
//! ## Note on TDD status
//!
//! This file is intentionally authored BEFORE `Cargo.toml` declares any of the
//! crates it references. On first run (no Cargo.toml, no deps) the test binary
//! fails to build — that is the "RED" state of TDD for STR-001. Once
//! `Cargo.toml` is created with the spec'd dependencies, the binary compiles
//! and every `#[test]` passes — that is the "GREEN" state, which is the
//! evidence STR-001 is satisfied.

use std::fs;
use std::path::{Path, PathBuf};

// ---------------------------------------------------------------------------
// Test Plan Row 1: `test_cargo_check`
// ---------------------------------------------------------------------------
//
// The spec's Row 1 is "Run `cargo check` on the crate". Integration tests are
// executed only after `cargo check`/`cargo build` succeeds for the test
// binary, so the mere fact that this binary compiles and runs is evidence
// that `cargo check` passed against the Cargo.toml that declares STR-001's
// dependencies. We also perform a lightweight structural sanity check on
// Cargo.toml itself to guard against accidental blank/incomplete manifests.

/// Returns the contents of this crate's `Cargo.toml`.
///
/// `CARGO_MANIFEST_DIR` is set by Cargo to the package root at compile time,
/// so this path always resolves to the Cargo.toml whose `[dependencies]`
/// section we are trying to verify.
fn read_cargo_toml() -> String {
    let manifest_path: PathBuf = Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml");
    fs::read_to_string(&manifest_path).unwrap_or_else(|err| {
        panic!(
            "failed to read Cargo.toml at {}: {err}",
            manifest_path.display()
        )
    })
}

/// **STR-001 Test Plan Row 1:** `test_cargo_check`.
///
/// Proves the crate compiles under `cargo check` with the declared
/// dependencies. The mere existence of this compiled test binary is direct
/// evidence; we additionally assert that Cargo.toml declares a
/// `[dependencies]` table and contains all 13 mandated crate names so a
/// regression that removes one is caught even when no call site in this
/// file happens to reference the removed crate.
#[test]
fn test_cargo_check() {
    let cargo_toml = read_cargo_toml();

    // Sanity: the manifest must have a dependencies table.
    assert!(
        cargo_toml.contains("[dependencies]"),
        "Cargo.toml is missing a [dependencies] table — STR-001 requires 13 entries"
    );

    // All 13 dependency names mandated by STR-001.
    let required = [
        "dig-block",
        "dig-constants",
        "chia-protocol",
        "chia-bls",
        "chia-consensus",
        "chia-sdk-types",
        "chia-sdk-signer",
        "chia-sha2",
        "clvm-utils",
        "bincode",
        "serde",
        "thiserror",
        "parking_lot",
    ];
    for dep in required {
        assert!(
            cargo_toml.contains(dep),
            "Cargo.toml is missing required STR-001 dependency: {dep}"
        );
    }

    // Exact version pins called out in the spec. We accept two syntactic
    // shapes because DIG crates currently require a path-override while
    // unpublished (`{ version = "0.1", path = "..." }`), whereas the Chia
    // crates are published and use the short form (`= "0.26"`).
    let pinned = [
        ("dig-block", "0.1"),
        ("dig-constants", "0.1"),
        ("chia-protocol", "0.26"),
        ("chia-bls", "0.26"),
        ("chia-consensus", "0.26"),
        ("chia-sdk-types", "0.30"),
        ("chia-sdk-signer", "0.30"),
        ("chia-sha2", "0.26"),
        ("clvm-utils", "0.26"),
    ];
    for (dep, version) in pinned {
        let short = format!("{dep} = \"{version}\"");
        let long = format!("{dep} = {{ version = \"{version}\"");
        assert!(
            cargo_toml.contains(&short) || cargo_toml.contains(&long),
            "STR-001 requires `{dep}` pinned to `{version}` in Cargo.toml — \
             neither `{short}` nor `{long}...}}` was found"
        );
    }
}

// ---------------------------------------------------------------------------
// Test Plan Row 2: `test_dig_block_import`
// ---------------------------------------------------------------------------

/// **STR-001 Test Plan Row 2:** `test_dig_block_import`.
///
/// Imports `dig_block::Checkpoint` and `dig_block::CheckpointSubmission`. If
/// this compiles, the `dig-block = "0.1"` dependency both resolves AND exposes
/// the two checkpoint types that the dig-epoch crate depends on (re-use rule:
/// "Checkpoint/block types from dig-block — never redefine"). Runtime asserts
/// are trivial; the compile step is the real proof.
#[test]
fn test_dig_block_import() {
    // Name-bind the imported items so the compiler actually requires the
    // dependency to be resolvable — an unused `use` can sometimes be
    // optimised out of effective testing.
    use dig_block::{Checkpoint, CheckpointSubmission};

    // Use `std::mem::size_of` on the imported types to force monomorphisation
    // without needing a constructor (which may have non-trivial arg shapes).
    let _checkpoint_size: usize = std::mem::size_of::<Checkpoint>();
    let _submission_size: usize = std::mem::size_of::<CheckpointSubmission>();
}

// ---------------------------------------------------------------------------
// Test Plan Row 3: `test_chia_protocol_import`
// ---------------------------------------------------------------------------

/// **STR-001 Test Plan Row 3:** `test_chia_protocol_import`.
///
/// Verifies `chia_protocol::Bytes32` — the universal 32-byte hash type used
/// everywhere in dig-epoch for state roots, checkpoint digests, etc. — is
/// available. Default-constructs a value to guarantee the type is not merely
/// name-visible but actually usable at runtime.
#[test]
fn test_chia_protocol_import() {
    use chia_protocol::Bytes32;

    let zero = Bytes32::default();
    // `Bytes32` default-constructs to 32 zero bytes — this asserts shape, not
    // a cryptographic property.
    assert_eq!(zero.as_ref(), &[0u8; 32]);
}

// ---------------------------------------------------------------------------
// Test Plan Row 4: `test_chia_bls_import`
// ---------------------------------------------------------------------------

/// **STR-001 Test Plan Row 4:** `test_chia_bls_import`.
///
/// Verifies `chia_bls::Signature` and `chia_bls::PublicKey` — the BLS
/// primitives dig-epoch uses for checkpoint aggregation (`aggregate`,
/// `aggregate_verify`). Default values are the identity point; sufficient to
/// prove the types link.
#[test]
fn test_chia_bls_import() {
    use chia_bls::{PublicKey, Signature};

    let _sig: Signature = Signature::default();
    let _pk: PublicKey = PublicKey::default();
}

// ---------------------------------------------------------------------------
// Test Plan Row 5: `test_chia_sdk_types_import`
// ---------------------------------------------------------------------------

/// **STR-001 Test Plan Row 5:** `test_chia_sdk_types_import`.
///
/// Verifies `chia_sdk_types::MerkleTree` and `chia_sdk_types::MerkleProof` —
/// the Merkle tree primitives dig-epoch uses to compute block-root / proof
/// material (distinct from `chia_consensus::compute_merkle_set_root`, which
/// is used for withdrawals). Constructs a `MerkleTree` from an empty leaf
/// slice to prove both the type and a basic constructor resolve.
#[test]
fn test_chia_sdk_types_import() {
    use chia_sdk_types::{MerkleProof, MerkleTree};

    // `MerkleTree::new` builds a tree over a slice of leaf hashes. An empty
    // slice is a valid edge case and avoids pulling in `Bytes32`-typed leaf
    // fixtures that would bloat this import-only test.
    let tree = MerkleTree::new(&[]);
    // Sanity: we must be able to *name* MerkleProof too. `size_of` is enough
    // to force resolution without constructing one (MerkleProof's exact
    // constructor shape is an implementation detail at 0.30).
    let _proof_size: usize = std::mem::size_of::<MerkleProof>();
    // Use `tree` so the compiler does not elide the construction.
    let _ = tree.root();
}

// ---------------------------------------------------------------------------
// Test Plan Row 6: `test_serde_derive`
// ---------------------------------------------------------------------------

/// **STR-001 Test Plan Row 6:** `test_serde_derive`.
///
/// Proves `serde` is wired up with the `derive` feature (without it, the
/// `#[derive(Serialize, Deserialize)]` attributes below would be a compile
/// error). The round-trip through `bincode` simultaneously exercises the
/// `bincode` dependency — all epoch types are bincode-serialised on the wire.
#[test]
fn test_serde_derive() {
    use serde::{Deserialize, Serialize};

    /// Minimal record used only to exercise `derive(Serialize, Deserialize)`
    /// at compile time. Field names are arbitrary; we care that the proc
    /// macro is available and the resulting impls round-trip.
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct SerdeProbe {
        epoch: u64,
        state_root: [u8; 32],
    }

    let probe = SerdeProbe {
        epoch: 42,
        state_root: [7u8; 32],
    };

    // Cross-check via bincode (STR-001 also mandates `bincode`). Using
    // `bincode::serde::encode_to_vec` targets both bincode v1 and v2 surfaces
    // via the explicit `serde::encode_to_vec` re-export in v2; to stay
    // compatible with whatever bincode version cargo resolves, we fall back
    // to the v1 `serialize` / `deserialize` API which has been stable since
    // bincode 1.3. If bincode v2 is resolved, this test would not compile;
    // in that case the Cargo.toml must pin bincode to "1" explicitly.
    let encoded = bincode::serialize(&probe).expect("bincode::serialize must succeed");
    let decoded: SerdeProbe =
        bincode::deserialize(&encoded).expect("bincode::deserialize must succeed");
    assert_eq!(probe, decoded, "serde derive + bincode round-trip failed");
}

// ---------------------------------------------------------------------------
// Test Plan Row 7: `test_parking_lot_import`
// ---------------------------------------------------------------------------

/// **STR-001 Test Plan Row 7:** `test_parking_lot_import`.
///
/// `EpochManager` relies on `parking_lot::RwLock` for interior mutability
/// (see `start.md` Hard Requirement 12). This test proves `parking_lot` is
/// resolvable and its `RwLock` exposes the basic read/write API shape we
/// depend on, without allocating any epoch state.
#[test]
fn test_parking_lot_import() {
    use parking_lot::RwLock;

    let lock: RwLock<u64> = RwLock::new(0);
    {
        let mut w = lock.write();
        *w = 1;
    }
    let r = lock.read();
    assert_eq!(*r, 1, "parking_lot::RwLock must expose write then read");
}

// ---------------------------------------------------------------------------
// Additional low-cost checks for the STR-001 acceptance criteria that aren't
// explicit rows in the Test Plan but are called out in the normative spec.
// These strengthen (not replace) the seven Test Plan rows above.
// ---------------------------------------------------------------------------

/// Cross-check: serde `derive` feature is declared in Cargo.toml.
///
/// The proc-macro derives in `test_serde_derive` would already fail without
/// the `derive` feature, but making this a separate textual test surfaces
/// the root cause in Cargo.toml instead of a confusing macro error.
#[test]
fn str_001_serde_has_derive_feature() {
    let cargo_toml = read_cargo_toml();
    assert!(
        cargo_toml.contains("features = [\"derive\"]")
            || cargo_toml.contains("features=[\"derive\"]"),
        "serde must be declared with the `derive` feature (STR-001)"
    );
}

/// Cross-check: remaining STR-001 crates that do not have a named Test Plan
/// row still must appear in Cargo.toml. We import representative items to
/// force link-time resolution.
#[test]
fn str_001_other_required_crates_link() {
    // `chia-consensus` — Merkle-set root for withdrawals.
    use chia_consensus::merkle_tree::MerkleSet;
    let _ = std::mem::size_of::<MerkleSet>();

    // `chia-sdk-signer` — domain-separation constants for signing digest.
    use chia_sdk_signer::AggSigConstants;
    let _ = std::mem::size_of::<AggSigConstants>();

    // `chia-sha2` — hash primitive used for checkpoint/digest construction.
    use chia_sha2::Sha256;
    let mut h = Sha256::new();
    h.update(b"str-001");
    let _: [u8; 32] = h.finalize();

    // `clvm-utils` — CLVM tree-hash helper (used by verification layer).
    use clvm_utils::TreeHash;
    let _ = std::mem::size_of::<TreeHash>();

    // `thiserror` — error derivation. Exercising the derive here guards the
    // `thiserror = "*"` line in Cargo.toml.
    #[derive(thiserror::Error, Debug)]
    enum StrProbeError {
        #[error("probe")]
        Probe,
    }
    let _ = StrProbeError::Probe;
}

/// Cross-check: `dig-constants` is a declared dependency. We do not yet pick
/// a specific symbol (that belongs to later phases), but we force the crate
/// to be reachable via a `use` of its root module via an attribute-gated
/// `extern crate` pattern that always resolves in Rust 2021 when the crate
/// is in `[dependencies]`.
#[test]
fn str_001_dig_constants_resolves() {
    // The simplest way to force `dig-constants` to be linked by this test
    // binary without depending on a specific public item (which may change
    // between 0.1 patch releases) is to reference it via a no-op `use` of
    // its crate root. Rust 2021 treats `extern crate` as implicit, so a bare
    // `use dig_constants as _;` both silences the unused-import lint and
    // forces resolution.
    use dig_constants as _;
}
