/// STR-005 — Test infrastructure helpers.
///
/// Normative: docs/requirements/domains/crate_structure/NORMATIVE.md §STR-005
/// Spec ref:  docs/resources/SPEC.md §14
use dig_epoch::constants::BLOCKS_PER_EPOCH;
use dig_epoch::test_helpers::{
    advance_through_phases, build_n_block_epoch, mock_checkpoint_submission, test_epoch_manager,
    test_initial_state_root, test_network_id, TEST_GENESIS_L1_HEIGHT,
};
use dig_epoch::types::epoch_phase::EpochPhase;

#[test]
fn test_helper_epoch_manager() {
    let m = test_epoch_manager();
    assert_eq!(m.current_epoch(), 0);
    assert_eq!(m.current_phase(), EpochPhase::BlockProduction);
    assert_eq!(m.network_id(), test_network_id());
    assert_eq!(m.genesis_l1_height(), TEST_GENESIS_L1_HEIGHT);
    assert_eq!(
        m.current_epoch_info().start_state_root,
        test_initial_state_root()
    );
}

#[test]
fn test_helper_advance_phases() {
    let m = test_epoch_manager();
    let ts = advance_through_phases(&m);
    assert_eq!(ts.len(), 3);
    assert_eq!(ts[0].from, EpochPhase::BlockProduction);
    assert_eq!(ts[0].to, EpochPhase::Checkpoint);
    assert_eq!(ts[1].from, EpochPhase::Checkpoint);
    assert_eq!(ts[1].to, EpochPhase::Finalization);
    assert_eq!(ts[2].from, EpochPhase::Finalization);
    assert_eq!(ts[2].to, EpochPhase::Complete);
    assert_eq!(m.current_phase(), EpochPhase::Complete);
}

#[test]
fn test_helper_mock_submission() {
    let s = mock_checkpoint_submission(1, 75, 32);
    assert_eq!(s.checkpoint.epoch, 1);
    assert_eq!(s.checkpoint.block_count, 32);
    assert_eq!(s.score, 75 * 32);
}

#[test]
fn test_helper_build_epoch() {
    let m = test_epoch_manager();
    let (fees, txns) = build_n_block_epoch(&m, 32, 1_000, 5);
    assert_eq!(fees, 32_000);
    assert_eq!(txns, 160);
    let info = m.current_epoch_info();
    assert_eq!(info.blocks_produced, 32);
    assert_eq!(info.total_fees, 32_000);
    assert_eq!(info.total_transactions, 160);
}

/// Helpers produce identical results across invocations.
#[test]
fn test_helper_deterministic() {
    let m1 = test_epoch_manager();
    let m2 = test_epoch_manager();
    assert_eq!(m1.network_id(), m2.network_id());
    assert_eq!(m1.genesis_l1_height(), m2.genesis_l1_height());
    let s1 = mock_checkpoint_submission(4, 60, 10);
    let s2 = mock_checkpoint_submission(4, 60, 10);
    assert_eq!(s1.score, s2.score);
    assert_eq!(s1.checkpoint.epoch, s2.checkpoint.epoch);
    assert_eq!(s1.checkpoint.block_count, s2.checkpoint.block_count);
}

/// n > BLOCKS_PER_EPOCH panics.
#[test]
#[should_panic(expected = "exceeds BLOCKS_PER_EPOCH")]
fn test_helper_build_epoch_overflow() {
    let m = test_epoch_manager();
    let _ = build_n_block_epoch(&m, (BLOCKS_PER_EPOCH as u32) + 1, 1, 1);
}
