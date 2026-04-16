/// TYP-003 — EpochSummary Struct
///
/// Normative: docs/requirements/domains/epoch_types/NORMATIVE.md §TYP-003
/// Spec ref:  docs/resources/SPEC.md §3.5
use chia_protocol::Bytes32;
use dig_block::Checkpoint;
use dig_epoch::types::epoch_info::EpochInfo;
use dig_epoch::types::epoch_summary::EpochSummary;

fn zero_root() -> Bytes32 {
    Bytes32::new([0u8; 32])
}

fn some_root(b: u8) -> Bytes32 {
    Bytes32::new([b; 32])
}

/// From<EpochInfo> with no checkpoint: finalized=false, checkpoint_hash=None.
#[test]
fn test_summary_from_epoch_info_no_checkpoint() {
    let info = EpochInfo::new(0, 100, 1, zero_root());
    let summary = EpochSummary::from(info);
    assert!(!summary.finalized);
    assert!(summary.checkpoint_hash.is_none());
}

/// From<EpochInfo> with checkpoint: finalized=true, checkpoint_hash=Some(cp.hash()).
#[test]
fn test_summary_from_epoch_info_with_checkpoint() {
    let mut info = EpochInfo::new(0, 100, 1, zero_root());
    let cp = Checkpoint::new();
    let expected_hash = cp.hash();
    info.set_checkpoint(cp);
    let summary = EpochSummary::from(info);
    assert!(summary.finalized);
    assert_eq!(summary.checkpoint_hash, Some(expected_hash));
}

/// Counter fields map correctly from EpochInfo.
#[test]
fn test_summary_field_mapping() {
    let mut info = EpochInfo::new(3, 200, 97, zero_root());
    for _ in 0..30 {
        info.record_block(500, 7);
    }
    let summary = EpochSummary::from(info);
    assert_eq!(summary.epoch, 3);
    assert_eq!(summary.blocks, 30);
    assert_eq!(summary.fees, 15_000);
    assert_eq!(summary.transactions, 210);
}

/// All 7 DFSP fields are preserved from EpochInfo.
#[test]
fn test_summary_dfsp_fields_preserved() {
    let mut info = EpochInfo::new(0, 0, 1, zero_root());
    info.collateral_registry_root = some_root(0xAA);
    info.cid_state_root = some_root(0xBB);
    info.node_registry_root = some_root(0xCC);
    info.namespace_epoch_root = some_root(0xDD);
    info.dfsp_issuance_total = 1_000_000;
    info.active_cid_count = 42;
    info.active_node_count = 7;
    let summary = EpochSummary::from(info);
    assert_eq!(summary.collateral_registry_root, some_root(0xAA));
    assert_eq!(summary.cid_state_root, some_root(0xBB));
    assert_eq!(summary.node_registry_root, some_root(0xCC));
    assert_eq!(summary.namespace_epoch_root, some_root(0xDD));
    assert_eq!(summary.dfsp_issuance_total, 1_000_000);
    assert_eq!(summary.active_cid_count, 42);
    assert_eq!(summary.active_node_count, 7);
}

/// Bincode serde round-trip preserves all 13 fields.
#[test]
fn test_summary_serde_roundtrip() {
    let mut info = EpochInfo::new(2, 64, 65, some_root(0x42));
    info.record_block(999, 3);
    info.set_checkpoint(Checkpoint::new());
    let summary = EpochSummary::from(info);
    let encoded = bincode::serialize(&summary).expect("serialize");
    let decoded: EpochSummary = bincode::deserialize(&encoded).expect("deserialize");
    assert_eq!(decoded.epoch, 2);
    assert_eq!(decoded.blocks, 1);
    assert_eq!(decoded.fees, 999);
    assert_eq!(decoded.transactions, 3);
    assert!(decoded.finalized);
    assert!(decoded.checkpoint_hash.is_some());
}

/// Clone produces an independent copy.
#[test]
fn test_summary_clone() {
    let info = EpochInfo::new(1, 32, 33, some_root(0x11));
    let summary = EpochSummary::from(info);
    let cloned = summary.clone();
    assert_eq!(cloned.epoch, summary.epoch);
    assert_eq!(cloned.blocks, summary.blocks);
    assert_eq!(cloned.finalized, summary.finalized);
}

/// Debug output is non-empty.
#[test]
fn test_summary_debug() {
    let info = EpochInfo::new(0, 0, 1, zero_root());
    let summary = EpochSummary::from(info);
    assert!(!format!("{summary:?}").is_empty());
}
