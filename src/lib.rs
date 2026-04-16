//! # dig-epoch
//!
//! Epoch geometry, phase machine, manager, and checkpoint-competition types
//! for the DIG L2.
//!
//! ## Status of this file
//!
//! This is the **initial stub** introduced by requirement
//! [`STR-001`](../docs/requirements/domains/crate_structure/specs/STR-001.md).
//! STR-001 only requires that the crate compiles and exposes a library
//! target; the real module tree (constants, types, arithmetic, phase,
//! rewards, manager, verification, dfsp, error) is introduced incrementally
//! in Phase 0 requirements STR-002 … STR-005 and beyond.
//!
//! Specifically, **no module declarations live here yet** — STR-002 is the
//! requirement that materialises the module hierarchy laid out in
//! [`NORMATIVE.md § STR-002`](../docs/requirements/domains/crate_structure/NORMATIVE.md)
//! and [`SPEC.md § 13`](../docs/resources/SPEC.md).
//!
//! ## Philosophy
//!
//! `dig-epoch` is intentionally thin: all primitive types (blocks,
//! checkpoints, hashes, BLS signatures, Merkle roots) come from the shared
//! DIG / Chia ecosystem crates listed in `Cargo.toml`. This crate
//! contributes the epoch-level *structure* (phases, arithmetic, reward
//! splits, the checkpoint competition) that sits above those primitives.
//!
//! See [`docs/resources/SPEC.md`](../docs/resources/SPEC.md) for the
//! authoritative crate specification.
//!
//! ## Re-exports
//!
//! None yet. Public re-exports arrive in
//! [`STR-003`](../docs/requirements/domains/crate_structure/specs/STR-003.md),
//! after the module hierarchy is in place.

// `deny(missing_docs)` would be ideal for a public API crate, but turning it
// on right now would force boilerplate docs on every subsequent requirement.
// We instead enable the softer `warn` so that CI surfaces missing docs without
// breaking the build while the module tree is still being fleshed out. This
// is consistent with the rest of the DIG/Chia ecosystem where crate-level
// docs are gated to the library target.
#![warn(missing_docs)]
