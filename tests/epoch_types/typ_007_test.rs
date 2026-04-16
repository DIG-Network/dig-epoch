/// TYP-007 — EpochStats Struct
///
/// Normative: docs/requirements/domains/epoch_types/NORMATIVE.md §TYP-007
/// Spec ref:  docs/resources/SPEC.md §3.8
use dig_epoch::types::events::EpochStats;

/// Default zeros all 5 fields.
#[test]
fn test_epoch_stats_default() {
    let s = EpochStats::default();
    assert_eq!(s.total_epochs, 0);
    assert_eq!(s.finalized_epochs, 0);
    assert_eq!(s.total_blocks, 0);
    assert_eq!(s.total_transactions, 0);
    assert_eq!(s.total_fees, 0);
}

/// Fields hold correct values after construction.
#[test]
fn test_epoch_stats_fields() {
    let s = EpochStats {
        total_epochs: 10,
        finalized_epochs: 8,
        total_blocks: 320,
        total_transactions: 5_000,
        total_fees: 100_000,
    };
    assert_eq!(s.total_epochs, 10);
    assert_eq!(s.finalized_epochs, 8);
    assert_eq!(s.total_blocks, 320);
    assert_eq!(s.total_transactions, 5_000);
    assert_eq!(s.total_fees, 100_000);
}

/// Clone produces identical fields.
#[test]
fn test_epoch_stats_clone() {
    let s = EpochStats {
        total_epochs: 3,
        finalized_epochs: 2,
        total_blocks: 96,
        total_transactions: 200,
        total_fees: 50_000,
    };
    let cloned = s.clone();
    assert_eq!(s.total_epochs, cloned.total_epochs);
    assert_eq!(s.total_fees, cloned.total_fees);
}

/// Debug produces non-empty output.
#[test]
fn test_epoch_stats_debug() {
    let s = EpochStats::default();
    assert!(!format!("{s:?}").is_empty());
}
