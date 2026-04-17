/// VER-004 — EpochCheckpointData + EpochCheckpointSignMaterial +
///           epoch_checkpoint_sign_material_from_l2_blocks.
///
/// Normative: docs/requirements/domains/verification/NORMATIVE.md §VER-004
/// Spec ref:  docs/resources/SPEC.md §7.4, §7.5
use chia_protocol::Bytes32;
use dig_epoch::types::verification::{EpochCheckpointData, EpochCheckpointSignMaterial};
use dig_epoch::verification::{
    compute_epoch_block_root, compute_epoch_withdrawals_root,
    epoch_checkpoint_sign_material_from_l2_blocks,
};

fn h(x: u8) -> Bytes32 {
    Bytes32::new([x; 32])
}

#[test]
fn test_checkpoint_data_fields() {
    let d = EpochCheckpointData {
        network_id: h(1),
        epoch: 42,
        block_root: h(2),
        state_root: h(3),
        withdrawals_root: h(4),
        checkpoint_hash: h(5),
    };
    assert_eq!(d.network_id, h(1));
    assert_eq!(d.epoch, 42);
    assert_eq!(d.block_root, h(2));
    assert_eq!(d.state_root, h(3));
    assert_eq!(d.withdrawals_root, h(4));
    assert_eq!(d.checkpoint_hash, h(5));
}

#[test]
fn test_sign_material_fields() {
    let d = EpochCheckpointData {
        network_id: h(0),
        epoch: 0,
        block_root: h(0),
        state_root: h(0),
        withdrawals_root: h(0),
        checkpoint_hash: h(0),
    };
    let m = EpochCheckpointSignMaterial {
        checkpoint: d.clone(),
        score: 100,
        signing_digest: h(7),
    };
    assert_eq!(m.checkpoint, d);
    assert_eq!(m.score, 100);
    assert_eq!(m.signing_digest, h(7));
}

#[test]
fn test_signing_digest_deterministic() {
    let d = EpochCheckpointData {
        network_id: h(9),
        epoch: 1,
        block_root: h(2),
        state_root: h(3),
        withdrawals_root: h(4),
        checkpoint_hash: h(5),
    };
    let a = d.signing_digest();
    let b = d.signing_digest();
    assert_eq!(a, b);
}

#[test]
fn test_from_l2_blocks_block_root_matches_helper() {
    let blocks = vec![h(1), h(2), h(3)];
    let withdrawals: Vec<Bytes32> = vec![];
    let mat = epoch_checkpoint_sign_material_from_l2_blocks(
        h(0),
        5,
        &blocks,
        h(10),
        &withdrawals,
        h(11),
        100,
        20,
        7,
    );
    assert_eq!(mat.checkpoint.block_root, compute_epoch_block_root(&blocks));
}

#[test]
fn test_from_l2_blocks_withdrawals_root_matches_helper() {
    let blocks: Vec<Bytes32> = vec![];
    let withdrawals = vec![h(20), h(21)];
    let mat = epoch_checkpoint_sign_material_from_l2_blocks(
        h(0),
        0,
        &blocks,
        h(0),
        &withdrawals,
        h(0),
        0,
        0,
        0,
    );
    assert_eq!(
        mat.checkpoint.withdrawals_root,
        compute_epoch_withdrawals_root(&withdrawals)
    );
}

#[test]
fn test_from_l2_blocks_network_id_preserved() {
    let mat = epoch_checkpoint_sign_material_from_l2_blocks(
        h(0xAB),
        3,
        &[h(1)],
        h(2),
        &[h(3)],
        h(4),
        1,
        2,
        3,
    );
    assert_eq!(mat.checkpoint.network_id, h(0xAB));
    assert_eq!(mat.checkpoint.epoch, 3);
}

#[test]
fn test_from_l2_blocks_score_formula() {
    let blocks = vec![h(1), h(2), h(3)];
    let mat = epoch_checkpoint_sign_material_from_l2_blocks(
        h(0),
        0,
        &blocks,
        h(0),
        &[],
        h(0),
        0,
        0,
        7, // stake_percentage
    );
    // score = stake_percentage * block_count = 7 * 3 = 21.
    assert_eq!(mat.score, 21);
}

#[test]
fn test_from_l2_blocks_signing_digest_matches_data() {
    let mat = epoch_checkpoint_sign_material_from_l2_blocks(
        h(1),
        1,
        &[h(1), h(2)],
        h(3),
        &[h(4)],
        h(5),
        100,
        10,
        5,
    );
    assert_eq!(mat.signing_digest, mat.checkpoint.signing_digest());
}
