/// REW-003 — Fee distribution (proposer_fee_share / burned_fee_remainder)
///
/// Normative: docs/requirements/domains/reward_economics/NORMATIVE.md §REW-003
/// Spec ref:  docs/resources/SPEC.md §8.3
use dig_epoch::constants::FEE_PROPOSER_SHARE_PCT;
use dig_epoch::rewards::{burned_fee_remainder, proposer_fee_share};

#[test]
fn test_zero_fees() {
    assert_eq!(proposer_fee_share(0), 0);
    assert_eq!(burned_fee_remainder(0), 0);
}

#[test]
fn test_even_split() {
    assert_eq!(proposer_fee_share(1000), 500);
    assert_eq!(burned_fee_remainder(1000), 500);
}

/// Odd fees: rounding goes to burn side.
#[test]
fn test_odd_fees() {
    assert_eq!(proposer_fee_share(1001), 500);
    assert_eq!(burned_fee_remainder(1001), 501);
}

/// Small fees: proposer=0, burned=1.
#[test]
fn test_small_fees() {
    assert_eq!(proposer_fee_share(1), 0);
    assert_eq!(burned_fee_remainder(1), 1);
}

#[test]
fn test_large_fees() {
    assert_eq!(proposer_fee_share(1_000_000_000), 500_000_000);
    assert_eq!(burned_fee_remainder(1_000_000_000), 500_000_000);
}

/// proposer + burned == total for various inputs.
#[test]
fn test_sum_invariant() {
    for fees in [0u64, 1, 2, 99, 100, 1001, 999_999, 1_000_000_000] {
        assert_eq!(proposer_fee_share(fees) + burned_fee_remainder(fees), fees);
    }
}

#[test]
fn test_fee_proposer_share_pct() {
    assert_eq!(FEE_PROPOSER_SHARE_PCT, 50);
}
