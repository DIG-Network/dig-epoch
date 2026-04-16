/// TYP-006 — EpochBlockLink Struct
///
/// Normative: docs/requirements/domains/epoch_types/NORMATIVE.md §TYP-006
/// Spec ref:  docs/resources/SPEC.md §3.13
use chia_protocol::Bytes32;
use dig_epoch::types::verification::EpochBlockLink;

fn make_hash(b: u8) -> Bytes32 {
    Bytes32::new([b; 32])
}

/// Fields are accessible with correct values.
#[test]
fn test_epoch_block_link_fields() {
    let link = EpochBlockLink {
        parent_hash: make_hash(0xAA),
        block_hash: make_hash(0xBB),
    };
    assert_eq!(link.parent_hash, make_hash(0xAA));
    assert_eq!(link.block_hash, make_hash(0xBB));
}

/// Clone produces identical values.
#[test]
fn test_epoch_block_link_clone() {
    let link = EpochBlockLink {
        parent_hash: make_hash(1),
        block_hash: make_hash(2),
    };
    let cloned = link.clone();
    assert_eq!(link.parent_hash, cloned.parent_hash);
    assert_eq!(link.block_hash, cloned.block_hash);
}

/// Debug formatting produces non-empty output.
#[test]
fn test_epoch_block_link_debug() {
    let link = EpochBlockLink {
        parent_hash: make_hash(0),
        block_hash: make_hash(0),
    };
    assert!(!format!("{link:?}").is_empty());
}

/// Serde round-trip preserves both fields.
#[test]
fn test_epoch_block_link_serde_roundtrip() {
    let link = EpochBlockLink {
        parent_hash: make_hash(0x11),
        block_hash: make_hash(0x22),
    };
    let encoded = bincode::serialize(&link).expect("serialize");
    let decoded: EpochBlockLink = bincode::deserialize(&encoded).expect("deserialize");
    assert_eq!(link.parent_hash, decoded.parent_hash);
    assert_eq!(link.block_hash, decoded.block_hash);
}
