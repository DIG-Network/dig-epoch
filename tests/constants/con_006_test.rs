/// CON-006 — Sentinel Constants
///
/// Normative: docs/requirements/domains/constants/NORMATIVE.md §CON-006
/// Spec ref:  docs/resources/SPEC.md §7.1, 7.3, 14.1
use chia_protocol::Bytes32;
use chia_sha2::Sha256;
use dig_epoch::constants::EMPTY_ROOT;

/// EMPTY_ROOT matches the canonical SHA-256 of the empty string (hex literal).
#[test]
fn test_empty_root_value() {
    let expected = Bytes32::new([
        0xe3, 0xb0, 0xc4, 0x42, 0x98, 0xfc, 0x1c, 0x14, 0x9a, 0xfb, 0xf4, 0xc8, 0x99, 0x6f, 0xb9,
        0x24, 0x27, 0xae, 0x41, 0xe4, 0x64, 0x9b, 0x93, 0x4c, 0xa4, 0x95, 0x99, 0x1b, 0x78, 0x52,
        0xb8, 0x55,
    ]);
    assert_eq!(EMPTY_ROOT, expected);
}

/// EMPTY_ROOT equals the runtime SHA-256 of b"" (independent verification).
#[test]
fn test_empty_root_runtime_sha256() {
    let mut hasher = Sha256::new();
    hasher.update(b"");
    let computed = Bytes32::new(hasher.finalize());
    assert_eq!(
        EMPTY_ROOT, computed,
        "EMPTY_ROOT must equal runtime SHA-256 of the empty string",
    );
}

/// EMPTY_ROOT is accessible from the crate root (import succeeds, value is Bytes32).
#[test]
fn test_empty_root_accessible() {
    let _: Bytes32 = EMPTY_ROOT;
}
