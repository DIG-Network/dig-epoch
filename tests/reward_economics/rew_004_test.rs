/// REW-004 — compute_reward_distribution (5-role split + RewardDistribution struct)
///
/// Normative: docs/requirements/domains/reward_economics/NORMATIVE.md §REW-004
/// Spec ref:  docs/resources/SPEC.md §8.4
use dig_epoch::rewards::compute_reward_distribution;

/// Standard 5-role split with total_reward=10000.
#[test]
fn test_distribution_percentages() {
    let d = compute_reward_distribution(0, 10_000, 0);
    assert_eq!(d.proposer_reward, 1_000);
    assert_eq!(d.attester_reward, 8_000);
    assert_eq!(d.ef_spawner_reward, 300);
    assert_eq!(d.score_submitter_reward, 400);
    assert_eq!(d.finalizer_reward, 300);
}

/// Sum of all 5 reward shares == total_reward.
#[test]
fn test_distribution_sum() {
    for total in [0u64, 1, 100, 10_000, 999_999, 1_000_000_000] {
        let d = compute_reward_distribution(0, total, 0);
        let sum = d.proposer_reward
            + d.attester_reward
            + d.ef_spawner_reward
            + d.score_submitter_reward
            + d.finalizer_reward;
        assert_eq!(sum, total, "total={total}");
    }
}

/// Fee fields computed from total_fees.
#[test]
fn test_distribution_with_fees() {
    let d = compute_reward_distribution(0, 0, 2_000);
    assert_eq!(d.proposer_fee_share, 1_000);
    assert_eq!(d.burned_fees, 1_000);
}

/// Zero reward: all reward shares are 0.
#[test]
fn test_distribution_zero_reward() {
    let d = compute_reward_distribution(0, 0, 0);
    assert_eq!(d.proposer_reward, 0);
    assert_eq!(d.attester_reward, 0);
    assert_eq!(d.ef_spawner_reward, 0);
    assert_eq!(d.score_submitter_reward, 0);
    assert_eq!(d.finalizer_reward, 0);
}

/// Zero fees: fee fields are 0.
#[test]
fn test_distribution_zero_fees() {
    let d = compute_reward_distribution(0, 1_000, 0);
    assert_eq!(d.proposer_fee_share, 0);
    assert_eq!(d.burned_fees, 0);
}

/// Rounding: total_reward=1 → sum still equals 1.
#[test]
fn test_distribution_rounding() {
    let d = compute_reward_distribution(0, 1, 0);
    let sum = d.proposer_reward
        + d.attester_reward
        + d.ef_spawner_reward
        + d.score_submitter_reward
        + d.finalizer_reward;
    assert_eq!(sum, 1);
}

/// epoch field matches input.
#[test]
fn test_distribution_epoch() {
    let d = compute_reward_distribution(42, 0, 0);
    assert_eq!(d.epoch, 42);
}

/// All 8 fields of RewardDistribution are accessible.
#[test]
fn test_struct_fields() {
    let d = compute_reward_distribution(1, 1_000, 500);
    let _ = d.epoch;
    let _ = d.proposer_reward;
    let _ = d.attester_reward;
    let _ = d.ef_spawner_reward;
    let _ = d.score_submitter_reward;
    let _ = d.finalizer_reward;
    let _ = d.proposer_fee_share;
    let _ = d.burned_fees;
}
