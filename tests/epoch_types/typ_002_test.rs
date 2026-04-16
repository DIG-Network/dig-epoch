/// TYP-002 — EpochInfo Struct
///
/// Normative: docs/requirements/domains/epoch_types/NORMATIVE.md §TYP-002
/// Spec ref:  docs/resources/SPEC.md §3.4
use chia_protocol::Bytes32;
use dig_block::Checkpoint;
use dig_epoch::constants::{BLOCKS_PER_EPOCH, EMPTY_ROOT, EPOCH_L1_BLOCKS};
use dig_epoch::types::epoch_info::EpochInfo;
use dig_epoch::types::epoch_phase::EpochPhase;

fn zero_root() -> Bytes32 {
    Bytes32::new([0u8; 32])
}

fn some_root(b: u8) -> Bytes32 {
    Bytes32::new([b; 32])
}

/// new() defaults: phase=BlockProduction, counters=0, checkpoint=None, end_l1_height=start+EPOCH_L1_BLOCKS.
#[test]
fn test_epoch_info_new_defaults() {
    let info = EpochInfo::new(0, 100, 1, zero_root());
    assert_eq!(info.phase, EpochPhase::BlockProduction);
    assert_eq!(info.blocks_produced, 0);
    assert_eq!(info.total_fees, 0);
    assert_eq!(info.total_transactions, 0);
    assert!(info.checkpoint.is_none());
    assert_eq!(info.end_l1_height, 100 + EPOCH_L1_BLOCKS);
}

/// new() DFSP defaults: all four roots = EMPTY_ROOT, counts = 0.
#[test]
fn test_epoch_info_new_dfsp_defaults() {
    let info = EpochInfo::new(0, 100, 1, zero_root());
    assert_eq!(info.collateral_registry_root, EMPTY_ROOT);
    assert_eq!(info.cid_state_root, EMPTY_ROOT);
    assert_eq!(info.node_registry_root, EMPTY_ROOT);
    assert_eq!(info.namespace_epoch_root, EMPTY_ROOT);
    assert_eq!(info.dfsp_issuance_total, 0);
    assert_eq!(info.active_cid_count, 0);
    assert_eq!(info.active_node_count, 0);
}

/// Identity fields are stored correctly.
#[test]
fn test_epoch_info_identity_fields() {
    let info = EpochInfo::new(5, 260, 161, some_root(0xAB));
    assert_eq!(info.epoch, 5);
    assert_eq!(info.start_l1_height, 260);
    assert_eq!(info.start_l2_height, 161);
    assert_eq!(info.end_l1_height, 260 + EPOCH_L1_BLOCKS);
    assert_eq!(info.start_state_root, some_root(0xAB));
}

/// target_blocks() returns BLOCKS_PER_EPOCH.
#[test]
fn test_target_blocks() {
    let info = EpochInfo::new(0, 0, 1, zero_root());
    assert_eq!(info.target_blocks(), BLOCKS_PER_EPOCH);
}

/// can_produce_blocks() true only in BlockProduction.
#[test]
fn test_can_produce_blocks() {
    let mut info = EpochInfo::new(0, 0, 1, zero_root());
    assert!(info.can_produce_blocks());
    info.phase = EpochPhase::Checkpoint;
    assert!(!info.can_produce_blocks());
}

/// can_submit_checkpoint() true only in Checkpoint phase.
#[test]
fn test_can_submit_checkpoint() {
    let mut info = EpochInfo::new(0, 0, 1, zero_root());
    assert!(!info.can_submit_checkpoint());
    info.phase = EpochPhase::Checkpoint;
    assert!(info.can_submit_checkpoint());
    info.phase = EpochPhase::Finalization;
    assert!(!info.can_submit_checkpoint());
}

/// is_complete() true only in Complete phase.
#[test]
fn test_is_complete() {
    let mut info = EpochInfo::new(0, 0, 1, zero_root());
    assert!(!info.is_complete());
    info.phase = EpochPhase::Complete;
    assert!(info.is_complete());
}

/// is_finalized() true only when checkpoint is Some.
#[test]
fn test_is_finalized() {
    let mut info = EpochInfo::new(0, 0, 1, zero_root());
    assert!(!info.is_finalized());
    info.set_checkpoint(Checkpoint::new());
    assert!(info.is_finalized());
}

/// record_block() increments counters correctly.
#[test]
fn test_record_block() {
    let mut info = EpochInfo::new(0, 0, 1, zero_root());
    info.record_block(500, 10);
    assert_eq!(info.blocks_produced, 1);
    assert_eq!(info.total_fees, 500);
    assert_eq!(info.total_transactions, 10);
    info.record_block(300, 5);
    assert_eq!(info.blocks_produced, 2);
    assert_eq!(info.total_fees, 800);
    assert_eq!(info.total_transactions, 15);
}

/// set_checkpoint() stores the checkpoint.
#[test]
fn test_set_checkpoint() {
    let mut info = EpochInfo::new(0, 0, 1, zero_root());
    let cp = Checkpoint::new();
    info.set_checkpoint(cp.clone());
    assert_eq!(info.checkpoint, Some(cp));
}

/// calculate_phase() returns BlockProduction for progress < 50.
#[test]
fn test_calculate_phase_block_production() {
    let info = EpochInfo::new(0, 100, 1, zero_root());
    // progress = (116 - 100) * 100 / 32 = 50 → Checkpoint starts at 50
    // Just under 50: l1_height = 115 → progress = 15 * 100 / 32 = 46
    assert_eq!(info.calculate_phase(115), EpochPhase::BlockProduction);
}

/// calculate_phase() returns Checkpoint for 50 <= progress < 75.
#[test]
fn test_calculate_phase_checkpoint() {
    let info = EpochInfo::new(0, 100, 1, zero_root());
    // l1_height = 116 → progress = 16 * 100 / 32 = 50
    assert_eq!(info.calculate_phase(116), EpochPhase::Checkpoint);
}

/// calculate_phase() returns Finalization for 75 <= progress < 100.
#[test]
fn test_calculate_phase_finalization() {
    let info = EpochInfo::new(0, 100, 1, zero_root());
    // l1_height = 124 → progress = 24 * 100 / 32 = 75
    assert_eq!(info.calculate_phase(124), EpochPhase::Finalization);
}

/// calculate_phase() returns Complete for progress >= 100.
#[test]
fn test_calculate_phase_complete() {
    let info = EpochInfo::new(0, 100, 1, zero_root());
    // l1_height = 132 → progress = 32 * 100 / 32 = 100
    assert_eq!(info.calculate_phase(132), EpochPhase::Complete);
}

/// progress_percentage() returns 0-100 based on L1 progress.
#[test]
fn test_progress_percentage() {
    let info = EpochInfo::new(0, 100, 1, zero_root());
    assert_eq!(info.progress_percentage(100), 0);
    assert_eq!(info.progress_percentage(116), 50);
    assert_eq!(info.progress_percentage(132), 100);
}

/// Clone produces an independent copy.
#[test]
fn test_epoch_info_clone() {
    let mut info = EpochInfo::new(0, 0, 1, zero_root());
    let cloned = info.clone();
    info.record_block(999, 1);
    assert_eq!(cloned.blocks_produced, 0);
    assert_eq!(cloned.total_fees, 0);
}

/// Bincode serde round-trip preserves all fields.
#[test]
fn test_epoch_info_serde_roundtrip() {
    let mut info = EpochInfo::new(1, 50, 33, some_root(0x42));
    info.record_block(1000, 5);
    info.set_checkpoint(Checkpoint::new());
    let encoded = bincode::serialize(&info).expect("serialize");
    let decoded: EpochInfo = bincode::deserialize(&encoded).expect("deserialize");
    assert_eq!(decoded.epoch, 1);
    assert_eq!(decoded.start_l1_height, 50);
    assert_eq!(decoded.blocks_produced, 1);
    assert_eq!(decoded.total_fees, 1000);
    assert!(decoded.checkpoint.is_some());
}
