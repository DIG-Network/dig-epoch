/// CON-004 — Fee and Reward Distribution Constants
///
/// Normative: docs/requirements/domains/constants/NORMATIVE.md §CON-004
/// Spec ref:  docs/resources/SPEC.md §2.3
///
/// Verifies that fee split and 5-role epoch reward distribution constants are
/// declared with the correct u64 types, spec-mandated values, and that the
/// invariant sums hold (fee shares sum to 100, epoch reward shares sum to 100).
use dig_epoch::constants::{
    ATTESTER_REWARD_SHARE, EF_SPAWNER_REWARD_SHARE, FEE_BURN_SHARE_PCT, FEE_PROPOSER_SHARE_PCT,
    FINALIZER_REWARD_SHARE, PROPOSER_REWARD_SHARE, SCORE_SUBMITTER_REWARD_SHARE,
};

/// FEE_PROPOSER_SHARE_PCT is 50.
#[test]
fn test_fee_proposer_share_pct() {
    assert_eq!(FEE_PROPOSER_SHARE_PCT, 50u64);
}

/// FEE_BURN_SHARE_PCT is 50.
#[test]
fn test_fee_burn_share_pct() {
    assert_eq!(FEE_BURN_SHARE_PCT, 50u64);
}

/// Fee shares sum to 100.
#[test]
fn test_fee_shares_sum_to_100() {
    assert_eq!(
        FEE_PROPOSER_SHARE_PCT + FEE_BURN_SHARE_PCT,
        100,
        "FEE_PROPOSER_SHARE_PCT + FEE_BURN_SHARE_PCT must equal 100",
    );
}

/// PROPOSER_REWARD_SHARE is 10.
#[test]
fn test_proposer_reward_share() {
    assert_eq!(PROPOSER_REWARD_SHARE, 10u64);
}

/// ATTESTER_REWARD_SHARE is 80.
#[test]
fn test_attester_reward_share() {
    assert_eq!(ATTESTER_REWARD_SHARE, 80u64);
}

/// EF_SPAWNER_REWARD_SHARE is 3.
#[test]
fn test_ef_spawner_reward_share() {
    assert_eq!(EF_SPAWNER_REWARD_SHARE, 3u64);
}

/// SCORE_SUBMITTER_REWARD_SHARE is 4.
#[test]
fn test_score_submitter_reward_share() {
    assert_eq!(SCORE_SUBMITTER_REWARD_SHARE, 4u64);
}

/// FINALIZER_REWARD_SHARE is 3.
#[test]
fn test_finalizer_reward_share() {
    assert_eq!(FINALIZER_REWARD_SHARE, 3u64);
}

/// Five epoch reward distribution shares sum to exactly 100.
#[test]
fn test_epoch_reward_shares_sum_to_100() {
    let total = PROPOSER_REWARD_SHARE
        + ATTESTER_REWARD_SHARE
        + EF_SPAWNER_REWARD_SHARE
        + SCORE_SUBMITTER_REWARD_SHARE
        + FINALIZER_REWARD_SHARE;
    assert_eq!(
        total, 100,
        "epoch reward shares must sum to 100, got {}",
        total,
    );
}
