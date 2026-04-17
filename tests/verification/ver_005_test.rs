/// VER-005 — stored_checkpoint_from_epoch_sign_material_with_aggregate_v1.
///
/// Normative: docs/requirements/domains/verification/NORMATIVE.md §VER-005
/// Spec ref:  docs/resources/SPEC.md §7.6
use chia_bls::{sign, SecretKey, Signature};
use chia_protocol::Bytes32;
use dig_epoch::types::verification::{EpochCheckpointData, EpochCheckpointSignMaterial};
use dig_epoch::verification::stored_checkpoint_from_epoch_sign_material_with_aggregate_v1;

fn h(x: u8) -> Bytes32 {
    Bytes32::new([x; 32])
}

fn sk(seed: u8) -> SecretKey {
    // 32-byte deterministic seed.
    SecretKey::from_seed(&[seed; 32])
}

fn sample_material(score: u64) -> EpochCheckpointSignMaterial {
    let data = EpochCheckpointData {
        network_id: h(0xA1),
        epoch: 7,
        block_root: h(1),
        state_root: h(2),
        withdrawals_root: h(3),
        checkpoint_hash: h(4),
    };
    let signing_digest = data.signing_digest();
    EpochCheckpointSignMaterial {
        checkpoint: data,
        score,
        signing_digest,
    }
}

#[test]
fn test_single_signer() {
    let mat = sample_material(100);
    let k0 = sk(1);
    let pk0 = k0.public_key();
    let s0: Signature = sign(&k0, mat.signing_digest.as_ref());
    let validator_set = vec![(0u32, pk0)];
    let per_validator = vec![(0u32, pk0, s0)];
    let sub = stored_checkpoint_from_epoch_sign_material_with_aggregate_v1(
        &mat,
        &validator_set,
        &per_validator,
        0,
    )
    .unwrap();
    assert_eq!(sub.score, 100);
    assert_eq!(sub.submitter, 0);
    assert!(sub.submission_height.is_none());
    assert!(sub.submission_coin.is_none());
}

#[test]
fn test_multiple_signers_aggregate_verifies() {
    let mat = sample_material(50);
    let k0 = sk(1);
    let k1 = sk(2);
    let k2 = sk(3);
    let pk0 = k0.public_key();
    let pk1 = k1.public_key();
    let pk2 = k2.public_key();
    let s0: Signature = sign(&k0, mat.signing_digest.as_ref());
    let s1: Signature = sign(&k1, mat.signing_digest.as_ref());
    let s2: Signature = sign(&k2, mat.signing_digest.as_ref());
    let validator_set = vec![(0, pk0), (1, pk1), (2, pk2)];
    let per_validator = vec![(0, pk0, s0), (1, pk1, s1), (2, pk2, s2)];
    let sub = stored_checkpoint_from_epoch_sign_material_with_aggregate_v1(
        &mat,
        &validator_set,
        &per_validator,
        1,
    )
    .unwrap();
    // Aggregate verification via chia-bls::aggregate_verify on the single message.
    let pks: Vec<_> = per_validator.iter().map(|(_, pk, _)| *pk).collect();
    let pk_refs: Vec<&chia_bls::PublicKey> = pks.iter().collect();
    let msgs: Vec<&[u8]> = vec![mat.signing_digest.as_ref(); 3];
    assert!(chia_bls::aggregate_verify(
        &sub.aggregate_signature,
        pk_refs.into_iter().zip(msgs.into_iter()),
    ));
}

#[test]
fn test_score_preserved() {
    let mat = sample_material(777);
    let k = sk(5);
    let pk = k.public_key();
    let sig = sign(&k, mat.signing_digest.as_ref());
    let sub = stored_checkpoint_from_epoch_sign_material_with_aggregate_v1(
        &mat,
        &[(0, pk)],
        &[(0, pk, sig)],
        0,
    )
    .unwrap();
    assert_eq!(sub.score, 777);
}

#[test]
fn test_submitter_stored() {
    let mat = sample_material(1);
    let k = sk(9);
    let pk = k.public_key();
    let sig = sign(&k, mat.signing_digest.as_ref());
    let sub = stored_checkpoint_from_epoch_sign_material_with_aggregate_v1(
        &mat,
        &[(0, pk)],
        &[(0, pk, sig)],
        42,
    )
    .unwrap();
    assert_eq!(sub.submitter, 42);
}

#[test]
fn test_submission_is_none() {
    let mat = sample_material(1);
    let k = sk(11);
    let pk = k.public_key();
    let sig = sign(&k, mat.signing_digest.as_ref());
    let sub = stored_checkpoint_from_epoch_sign_material_with_aggregate_v1(
        &mat,
        &[(0, pk)],
        &[(0, pk, sig)],
        0,
    )
    .unwrap();
    assert!(sub.submission_height.is_none());
    assert!(sub.submission_coin.is_none());
}

#[test]
fn test_empty_per_validator_errors() {
    let mat = sample_material(1);
    let k = sk(12);
    let pk = k.public_key();
    let validator_set = vec![(0, pk)];
    let per_validator: Vec<(u32, chia_bls::PublicKey, Signature)> = vec![];
    let r = stored_checkpoint_from_epoch_sign_material_with_aggregate_v1(
        &mat,
        &validator_set,
        &per_validator,
        0,
    );
    assert!(r.is_err());
}

#[test]
fn test_bitmap_reflects_signers() {
    let mat = sample_material(1);
    let k0 = sk(20);
    let k1 = sk(21);
    let k2 = sk(22);
    let pk0 = k0.public_key();
    let pk1 = k1.public_key();
    let pk2 = k2.public_key();
    let s1 = sign(&k1, mat.signing_digest.as_ref());
    // Validator set has 3 validators but only validator 1 signs.
    let validator_set = vec![(10, pk0), (11, pk1), (12, pk2)];
    let per_validator = vec![(11, pk1, s1)];
    let sub = stored_checkpoint_from_epoch_sign_material_with_aggregate_v1(
        &mat,
        &validator_set,
        &per_validator,
        0,
    )
    .unwrap();
    assert_eq!(sub.signer_bitmap.validator_count(), 3);
    assert_eq!(sub.signer_bitmap.signing_percentage(), 33);
}
