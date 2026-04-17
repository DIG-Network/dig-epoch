/// STR-003 — Public re-exports at the crate root.
///
/// Normative: docs/requirements/domains/crate_structure/NORMATIVE.md §STR-003
/// Spec ref:  docs/resources/SPEC.md §12
///
/// Each test references symbols via `dig_epoch::<name>` (not via sub-module
/// paths). DFSP free functions are deferred per IMPLEMENTATION_ORDER.md, so
/// they are not covered here — only implemented requirements are asserted.

// -- Manager + types re-exports --
#[test]
fn test_manager_reexport() {
    let _ = dig_epoch::EpochManager::new(
        chia_protocol::Bytes32::new([0; 32]),
        100,
        chia_protocol::Bytes32::new([0; 32]),
    );
}

#[test]
fn test_type_reexports() {
    // Each import site proves the name is re-exported.
    use dig_epoch::{
        CheckpointCompetition, CompetitionStatus, DfspCloseSnapshot, EpochBlockLink,
        EpochCheckpointData, EpochCheckpointSignMaterial, EpochEvent, EpochInfo, EpochPhase,
        EpochStats, EpochSummary, PhaseTransition, RewardDistribution,
    };
    let _: EpochPhase = EpochPhase::BlockProduction;
    let _: PhaseTransition = PhaseTransition {
        epoch: 0,
        from: EpochPhase::BlockProduction,
        to: EpochPhase::Checkpoint,
        l1_height: 0,
    };
    let info = EpochInfo::new(0, 100, 1, chia_protocol::Bytes32::new([0; 32]));
    let _: EpochSummary = info.clone().into();
    let _: DfspCloseSnapshot = DfspCloseSnapshot {
        collateral_registry_root: chia_protocol::Bytes32::new([0; 32]),
        cid_state_root: chia_protocol::Bytes32::new([0; 32]),
        node_registry_root: chia_protocol::Bytes32::new([0; 32]),
        namespace_epoch_root: chia_protocol::Bytes32::new([0; 32]),
        dfsp_issuance_total: 0,
        active_cid_count: 0,
        active_node_count: 0,
    };
    let _: EpochStats = EpochStats::default();
    let _: EpochEvent = EpochEvent::EpochStarted {
        epoch: 0,
        l1_height: 0,
    };
    let _: CheckpointCompetition = CheckpointCompetition::new(0);
    let _: CompetitionStatus = CompetitionStatus::Pending;
    let _: RewardDistribution = RewardDistribution {
        epoch: 0,
        proposer_reward: 0,
        attester_reward: 0,
        ef_spawner_reward: 0,
        score_submitter_reward: 0,
        finalizer_reward: 0,
        proposer_fee_share: 0,
        burned_fees: 0,
    };
    let _: EpochCheckpointData = EpochCheckpointData {
        network_id: chia_protocol::Bytes32::new([0; 32]),
        epoch: 0,
        block_root: chia_protocol::Bytes32::new([0; 32]),
        state_root: chia_protocol::Bytes32::new([0; 32]),
        withdrawals_root: chia_protocol::Bytes32::new([0; 32]),
        checkpoint_hash: chia_protocol::Bytes32::new([0; 32]),
    };
    let _: EpochCheckpointSignMaterial = EpochCheckpointSignMaterial {
        checkpoint: EpochCheckpointData {
            network_id: chia_protocol::Bytes32::new([0; 32]),
            epoch: 0,
            block_root: chia_protocol::Bytes32::new([0; 32]),
            state_root: chia_protocol::Bytes32::new([0; 32]),
            withdrawals_root: chia_protocol::Bytes32::new([0; 32]),
            checkpoint_hash: chia_protocol::Bytes32::new([0; 32]),
        },
        score: 0,
        signing_digest: chia_protocol::Bytes32::new([0; 32]),
    };
    let _: EpochBlockLink = EpochBlockLink {
        parent_hash: chia_protocol::Bytes32::new([0; 32]),
        block_hash: chia_protocol::Bytes32::new([0; 32]),
    };
}

// -- Constants re-exports --
#[test]
fn test_constant_reexports() {
    let _ = dig_epoch::BLOCKS_PER_EPOCH;
    let _ = dig_epoch::EPOCH_L1_BLOCKS;
    let _ = dig_epoch::GENESIS_HEIGHT;
    let _ = dig_epoch::EMPTY_ROOT;
    let _ = dig_epoch::INITIAL_BLOCK_REWARD;
    let _ = dig_epoch::MINIMUM_EPOCH_REWARD;
}

// -- Arithmetic re-exports --
#[test]
fn test_arithmetic_reexports() {
    let _ = dig_epoch::epoch_for_block_height(1);
    let _ = dig_epoch::first_height_in_epoch(0);
    let _ = dig_epoch::epoch_checkpoint_height(0);
    let _ = dig_epoch::last_committed_height_in_epoch(0, 10);
    let _ = dig_epoch::is_genesis_checkpoint_block(1);
    let _ = dig_epoch::is_epoch_checkpoint_block(32);
    let _ = dig_epoch::is_checkpoint_class_block(1);
    let _ = dig_epoch::is_first_block_after_epoch_checkpoint(33);
    let _ = dig_epoch::l1_range_for_epoch(100, 0);
    let _ = dig_epoch::ensure_checkpoint_block_empty(1, 0, 0, 0);
}

// -- Phase re-export --
#[test]
fn test_phase_reexport() {
    let _ = dig_epoch::l1_progress_phase_for_network_epoch(100, 0, 100);
}

// -- Reward re-exports --
#[test]
fn test_reward_reexports() {
    let _ = dig_epoch::block_reward_at_height(1);
    let _ = dig_epoch::total_block_reward(1, false);
    let _ = dig_epoch::proposer_fee_share(1000);
    let _ = dig_epoch::burned_fee_remainder(1000);
    let _ = dig_epoch::compute_reward_distribution(0, 1000, 100);
    let _ = dig_epoch::epoch_reward_with_floor(0);
}

// -- Verification re-exports --
#[test]
fn test_verification_reexports() {
    let _ = dig_epoch::compute_epoch_block_root(&[]);
    let _ = dig_epoch::compute_epoch_withdrawals_root(&[]);
    let _: Option<_> = dig_epoch::epoch_block_inclusion_proof(&[], 0);
    let _ = dig_epoch::epoch_checkpoint_sign_material_from_l2_blocks(
        chia_protocol::Bytes32::new([0; 32]),
        0,
        &[],
        chia_protocol::Bytes32::new([0; 32]),
        &[],
        chia_protocol::Bytes32::new([0; 32]),
        0,
        0,
        0,
    );
}

// -- Error re-exports --
#[test]
fn test_error_reexports() {
    let _: dig_epoch::EpochError = dig_epoch::EpochError::InvalidHeight(0);
    let _: dig_epoch::CheckpointCompetitionError =
        dig_epoch::CheckpointCompetitionError::NotStarted;
}

// -- Wildcard import --
#[test]
fn test_wildcard_import() {
    use dig_epoch::*;
    let _ = EpochManager::new(
        chia_protocol::Bytes32::new([0; 32]),
        100,
        chia_protocol::Bytes32::new([0; 32]),
    );
    let _ = BLOCKS_PER_EPOCH;
    let _ = epoch_for_block_height(1);
    let _: EpochPhase = EpochPhase::BlockProduction;
}
