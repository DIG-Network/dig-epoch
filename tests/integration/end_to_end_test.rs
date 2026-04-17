use dig_epoch::test_helpers::{mock_checkpoint_submission, test_epoch_manager};
/// End-to-end cohesion test — drives a full epoch lifecycle using only the
/// crate's public API (flat `dig_epoch::*` surface).
///
/// Exercised surface:
/// - construction (STR-004 / MGR-001)
/// - record_block + set_current_epoch_chain_totals (MGR-002/003)
/// - phase tracking + should_advance (PHS-002/003, MGR-008)
/// - start_checkpoint_competition, submit_checkpoint, finalize_competition
///   (CKP-002/003/004)
/// - set_current_epoch_dfsp_close_snapshot (MGR-006)
/// - advance_epoch + summary archive (MGR-004/007)
/// - query methods get_epoch_info/_summary, recent_summaries, total_stats,
///   get_rewards (MGR-005)
/// - store_rewards (MGR-008) + compute_reward_distribution (REW-004)
/// - verification helpers (VER-001/002/003/004) and inclusion-proof verify
/// - serialization round-trip (SER-001/002/003)
use dig_epoch::{
    block_reward_at_height, burned_fee_remainder, compute_epoch_block_root,
    compute_epoch_withdrawals_root, compute_reward_distribution, epoch_block_inclusion_proof,
    epoch_checkpoint_sign_material_from_l2_blocks, epoch_reward_with_floor, l1_range_for_epoch,
    proposer_fee_share, total_block_reward, verify_block_inclusion_proof, CheckpointCompetition,
    CompetitionStatus, DfspCloseSnapshot, EpochPhase, EpochSummary, BLOCKS_PER_EPOCH,
    GENESIS_HEIGHT,
};

// STR-003 cohesion: single-crate imports only. `Bytes32` is re-exported
// from `dig_epoch`, so consumers do not need a direct `chia_protocol`
// dependency for the basic surface.
use dig_epoch::Bytes32;

fn h(x: u8) -> Bytes32 {
    Bytes32::new([x; 32])
}

/// Drive epoch 0 from BlockProduction through Finalized + advance.
#[test]
fn test_full_epoch_lifecycle_via_public_api() {
    let m = test_epoch_manager();
    assert_eq!(m.current_epoch(), 0);
    assert_eq!(m.current_phase(), EpochPhase::BlockProduction);

    // -- Record 16 blocks (half epoch) --
    let block_hashes: Vec<Bytes32> = (1..=16u8).map(h).collect();
    let mut total_fees = 0u64;
    let mut total_txns = 0u64;
    for _ in 0..16 {
        m.record_block(500, 2).unwrap();
        total_fees += 500;
        total_txns += 2;
    }
    let info = m.current_epoch_info();
    assert_eq!(info.blocks_produced, 16);
    assert_eq!(info.total_fees, total_fees);
    assert_eq!(info.total_transactions, total_txns);

    // -- Cross into Checkpoint phase --
    let (_, end_l1) = m.l1_range_for_epoch(0);
    m.update_phase(end_l1 / 2 + GENESIS_HEIGHT as u32); // safely inside 50%
                                                        // Force Checkpoint via explicit boundary: start + EPOCH_L1_BLOCKS/2 = 100+16 = 116.
    m.update_phase(116);
    assert_eq!(m.current_phase(), EpochPhase::Checkpoint);

    // -- Start competition, submit, confirm WinnerSelected --
    m.start_checkpoint_competition().unwrap();
    assert_eq!(m.competition().status, CompetitionStatus::Collecting);
    let sub_low = mock_checkpoint_submission(0, 10, 16);
    let sub_high = mock_checkpoint_submission(0, 100, 16);
    let became_leader_low = m.submit_checkpoint(sub_low).unwrap();
    assert!(became_leader_low);
    let became_leader_high = m.submit_checkpoint(sub_high).unwrap();
    assert!(became_leader_high);
    let c = m.competition();
    assert!(matches!(c.status, CompetitionStatus::WinnerSelected { .. }));
    assert_eq!(c.submissions.len(), 2);

    // -- Advance to Finalization, attach DFSP close snapshot --
    m.update_phase(124); // 75% boundary
    assert_eq!(m.current_phase(), EpochPhase::Finalization);
    let snap = DfspCloseSnapshot {
        collateral_registry_root: h(0xC0),
        cid_state_root: h(0xC1),
        node_registry_root: h(0xC2),
        namespace_epoch_root: h(0xC3),
        dfsp_issuance_total: 10_000,
        active_cid_count: 5,
        active_node_count: 9,
    };
    m.set_current_epoch_dfsp_close_snapshot(snap).unwrap();

    // -- Finalize the competition --
    let winner = m.finalize_competition(0, 132).unwrap();
    assert!(winner.is_some());
    let c = m.competition();
    assert!(c.is_finalized());
    let info = m.current_epoch_info();
    assert!(info.checkpoint.is_some());
    assert_eq!(info.collateral_registry_root, h(0xC0));
    assert_eq!(info.dfsp_issuance_total, 10_000);

    // -- Cross to Complete + compute + store rewards before advancing --
    m.update_phase(133); // past 100%
    assert_eq!(m.current_phase(), EpochPhase::Complete);
    assert!(m.should_advance(133));

    // Reward distribution derived from protocol rules.
    let base_reward: u64 = (1..=BLOCKS_PER_EPOCH)
        .map(|h| total_block_reward(h, h == 1))
        .sum();
    let total_reward = epoch_reward_with_floor(base_reward);
    let dist = compute_reward_distribution(0, total_reward, total_fees);
    assert_eq!(dist.epoch, 0);
    assert_eq!(
        dist.proposer_reward
            + dist.attester_reward
            + dist.ef_spawner_reward
            + dist.score_submitter_reward
            + dist.finalizer_reward,
        total_reward
    );
    assert_eq!(dist.proposer_fee_share, proposer_fee_share(total_fees));
    assert_eq!(dist.burned_fees, burned_fee_remainder(total_fees));
    m.store_rewards(dist.clone());
    let stored = m.get_rewards(0).unwrap();
    assert_eq!(stored.attester_reward, dist.attester_reward);

    // -- advance_epoch archives summary and rolls forward --
    let next = m.advance_epoch(133, h(0xFE)).unwrap();
    assert_eq!(next, 1);
    assert_eq!(m.current_epoch(), 1);
    assert_eq!(m.current_phase(), EpochPhase::BlockProduction);
    let summary = m.get_epoch_summary(0).unwrap();
    assert_eq!(summary.epoch, 0);
    assert_eq!(summary.blocks, 16);
    assert_eq!(summary.fees, total_fees);
    assert!(summary.finalized);
    // Competition reset.
    let c1 = m.competition();
    assert_eq!(c1.epoch, 1);
    assert_eq!(c1.status, CompetitionStatus::Pending);

    // -- Query methods --
    let stats = m.total_stats();
    assert!(stats.total_epochs >= 2);
    assert!(stats.total_blocks >= 16);
    let recent = m.recent_summaries(5);
    assert_eq!(recent.len(), 1);
    assert_eq!(recent[0].epoch, 0);
    assert!(m.get_competition(0).is_none()); // previous competition not retained
    assert!(m.get_competition(1).is_some());

    // -- Verification pipeline over the archived block hashes --
    let block_root = compute_epoch_block_root(&block_hashes);
    assert_ne!(block_root, h(0));
    let withdrawals = vec![h(0xD1), h(0xD2), h(0xD3)];
    let wroot = compute_epoch_withdrawals_root(&withdrawals);
    assert_ne!(wroot, h(0));
    // Inclusion proof + verify.
    let proof = epoch_block_inclusion_proof(&block_hashes, 7).unwrap();
    assert!(verify_block_inclusion_proof(
        block_hashes[7],
        &proof,
        block_root
    ));
    // Sign material binds network_id and carries expected roots.
    let mat = epoch_checkpoint_sign_material_from_l2_blocks(
        m.network_id(),
        0,
        &block_hashes,
        h(0xFE),
        &withdrawals,
        h(0),
        total_fees,
        total_txns,
        75,
    );
    assert_eq!(mat.checkpoint.block_root, block_root);
    assert_eq!(mat.checkpoint.withdrawals_root, wroot);
    assert_eq!(mat.score, 75 * 16);

    // -- Serialization round-trip on the archived summary --
    let s_bytes = summary.to_bytes();
    let s_back = EpochSummary::from_bytes(&s_bytes).unwrap();
    assert_eq!(s_back.epoch, summary.epoch);
    assert_eq!(s_back.fees, summary.fees);
    assert_eq!(
        s_back.collateral_registry_root,
        summary.collateral_registry_root
    );

    // -- Serialization round-trip on the (finalized) competition --
    // Note: fetch from manager ERASED state, so rebuild a representative one.
    let mut c = CheckpointCompetition::new(42);
    c.status = CompetitionStatus::Finalized {
        winner_hash: h(0xAB),
        l1_height: 999,
    };
    let c_bytes = c.to_bytes();
    let c_back = CheckpointCompetition::from_bytes(&c_bytes).unwrap();
    assert_eq!(c_back.epoch, 42);
    assert!(c_back.is_finalized());
}

/// Halving schedule boundary sanity — block_reward_at_height matches expected
/// values at each halving boundary.
#[test]
fn test_reward_schedule_boundaries_public_api() {
    // Initial reward at height 1.
    assert_eq!(block_reward_at_height(1), dig_epoch::INITIAL_BLOCK_REWARD);
    // After 4 halvings, should be TAIL_BLOCK_REWARD.
    let after_tail = 4 * dig_epoch::HALVING_INTERVAL_BLOCKS + 1;
    assert_eq!(
        block_reward_at_height(after_tail),
        dig_epoch::TAIL_BLOCK_REWARD
    );
}

/// Arithmetic round-trip via the flat public API.
#[test]
fn test_arithmetic_identity_public_api() {
    for epoch in [0u64, 1, 5, 100, 9999] {
        let first = dig_epoch::first_height_in_epoch(epoch);
        let checkpoint = dig_epoch::epoch_checkpoint_height(epoch);
        assert_eq!(dig_epoch::epoch_for_block_height(first), epoch);
        assert_eq!(dig_epoch::epoch_for_block_height(checkpoint), epoch);
        assert!(dig_epoch::is_epoch_checkpoint_block(checkpoint) || checkpoint == 0);
    }
    // l1_range_for_epoch width matches EPOCH_L1_BLOCKS.
    let (start, end) = l1_range_for_epoch(100, 3);
    assert_eq!(end - start + 1, dig_epoch::EPOCH_L1_BLOCKS);
}
