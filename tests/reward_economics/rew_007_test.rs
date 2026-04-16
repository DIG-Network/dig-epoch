/// REW-007 — RewardDistribution struct
///
/// Normative: docs/requirements/domains/reward_economics/NORMATIVE.md §REW-007
/// Spec ref:  docs/resources/SPEC.md §3.12
use dig_epoch::types::reward::RewardDistribution;

/// All 8 fields exist with correct types and are publicly accessible.
#[test]
fn test_reward_distribution_fields() {
    let d = RewardDistribution {
        epoch: 1u64,
        proposer_reward: 100u64,
        attester_reward: 800u64,
        ef_spawner_reward: 30u64,
        score_submitter_reward: 40u64,
        finalizer_reward: 30u64,
        proposer_fee_share: 50u64,
        burned_fees: 50u64,
    };
    assert_eq!(d.epoch, 1);
    assert_eq!(d.proposer_reward, 100);
    assert_eq!(d.attester_reward, 800);
    assert_eq!(d.ef_spawner_reward, 30);
    assert_eq!(d.score_submitter_reward, 40);
    assert_eq!(d.finalizer_reward, 30);
    assert_eq!(d.proposer_fee_share, 50);
    assert_eq!(d.burned_fees, 50);
}

/// Debug output is non-empty.
#[test]
fn test_reward_distribution_debug() {
    let d = RewardDistribution {
        epoch: 0,
        proposer_reward: 0,
        attester_reward: 0,
        ef_spawner_reward: 0,
        score_submitter_reward: 0,
        finalizer_reward: 0,
        proposer_fee_share: 0,
        burned_fees: 0,
    };
    assert!(!format!("{d:?}").is_empty());
}

/// Clone produces an independent copy.
#[test]
fn test_reward_distribution_clone() {
    let d = RewardDistribution {
        epoch: 5,
        proposer_reward: 1_000,
        attester_reward: 8_000,
        ef_spawner_reward: 300,
        score_submitter_reward: 400,
        finalizer_reward: 300,
        proposer_fee_share: 500,
        burned_fees: 500,
    };
    let c = d.clone();
    assert_eq!(c.epoch, d.epoch);
    assert_eq!(c.attester_reward, d.attester_reward);
}
