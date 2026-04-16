/// TYP-004 — DfspCloseSnapshot Struct
///
/// Normative: docs/requirements/domains/epoch_types/NORMATIVE.md §TYP-004
/// Spec ref:  docs/resources/SPEC.md §3.6
use chia_protocol::Bytes32;
use dig_epoch::types::dfsp::DfspCloseSnapshot;

fn make_root(byte: u8) -> Bytes32 {
    Bytes32::new([byte; 32])
}

fn sample_snapshot() -> DfspCloseSnapshot {
    DfspCloseSnapshot {
        collateral_registry_root: make_root(1),
        cid_state_root: make_root(2),
        node_registry_root: make_root(3),
        namespace_epoch_root: make_root(4),
        dfsp_issuance_total: 1_000,
        active_cid_count: 50,
        active_node_count: 10,
    }
}

/// All 7 fields are accessible with correct values after construction.
#[test]
fn test_dfsp_snapshot_construction() {
    let s = sample_snapshot();
    assert_eq!(s.collateral_registry_root, make_root(1));
    assert_eq!(s.cid_state_root, make_root(2));
    assert_eq!(s.node_registry_root, make_root(3));
    assert_eq!(s.namespace_epoch_root, make_root(4));
    assert_eq!(s.dfsp_issuance_total, 1_000);
    assert_eq!(s.active_cid_count, 50);
    assert_eq!(s.active_node_count, 10);
}

/// Copy semantics: assigning creates an independent copy.
#[test]
fn test_dfsp_snapshot_copy() {
    let original = sample_snapshot();
    let mut copy = original;
    copy.active_cid_count = 99;
    assert_eq!(original.active_cid_count, 50);
    assert_eq!(copy.active_cid_count, 99);
}

/// Clone produces a value with identical fields.
#[test]
fn test_dfsp_snapshot_clone() {
    let s = sample_snapshot();
    let cloned = s.clone();
    assert_eq!(s.collateral_registry_root, cloned.collateral_registry_root);
    assert_eq!(s.dfsp_issuance_total, cloned.dfsp_issuance_total);
    assert_eq!(s.active_node_count, cloned.active_node_count);
}

/// Debug formatting produces non-empty output.
#[test]
fn test_dfsp_snapshot_debug() {
    let s = sample_snapshot();
    let dbg = format!("{s:?}");
    assert!(!dbg.is_empty());
}
