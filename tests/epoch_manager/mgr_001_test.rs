/// MGR-001 — EpochManager struct: wraps RwLock<EpochManagerInner> with 5 required fields.
///
/// Normative: docs/requirements/domains/epoch_manager/NORMATIVE.md §MGR-001
/// Spec ref:  docs/resources/SPEC.md §6.1, §6.2
use chia_protocol::Bytes32;
use dig_epoch::constants::GENESIS_HEIGHT;
use dig_epoch::manager::EpochManager;
use dig_epoch::types::checkpoint_competition::CompetitionStatus;
use dig_epoch::types::epoch_phase::EpochPhase;

fn nid() -> Bytes32 {
    Bytes32::new([1u8; 32])
}

fn root() -> Bytes32 {
    Bytes32::new([7u8; 32])
}

/// Construct EpochManager with network_id, genesis_l1_height, initial_state_root.
/// All 5 inner fields visible via public accessors.
#[test]
fn test_construction_initializes_all_fields() {
    let mgr = EpochManager::new(nid(), 100, root());

    // network_id
    assert_eq!(mgr.network_id(), nid());
    // genesis_l1_height
    assert_eq!(mgr.genesis_l1_height(), 100);
    // current_epoch (EpochInfo) — epoch 0, start_l1=100, start_l2=GENESIS_HEIGHT
    let info = mgr.current_epoch_info();
    assert_eq!(info.epoch, 0);
    assert_eq!(info.start_l1_height, 100);
    assert_eq!(info.start_l2_height, GENESIS_HEIGHT);
    assert_eq!(info.start_state_root, root());
    assert_eq!(info.phase, EpochPhase::BlockProduction);
    // competition — fresh Pending for epoch 0
    let comp = mgr.competition();
    assert_eq!(comp.epoch, 0);
    assert_eq!(comp.status, CompetitionStatus::Pending);
    assert!(comp.submissions.is_empty());
    assert!(comp.current_winner.is_none());
    // summaries — empty
    assert!(mgr.recent_summaries(100).is_empty());
    // rewards — empty
    assert!(mgr.get_rewards(0).is_none());
}

/// current_epoch() returns 0 at construction.
#[test]
fn test_current_epoch_zero_at_start() {
    let mgr = EpochManager::new(nid(), 100, root());
    assert_eq!(mgr.current_epoch(), 0);
}

/// current_phase() is BlockProduction at start.
#[test]
fn test_current_phase_block_production_at_start() {
    let mgr = EpochManager::new(nid(), 100, root());
    assert_eq!(mgr.current_phase(), EpochPhase::BlockProduction);
}

/// network_id and genesis_l1_height are immutable after construction.
/// Cloning the manager's state (via current_epoch_info) leaves originals intact.
#[test]
fn test_immutable_identity_fields() {
    let mgr = EpochManager::new(nid(), 42, root());
    mgr.record_block(100, 5).unwrap();
    mgr.record_block(200, 3).unwrap();
    // network_id and genesis_l1_height unchanged by writes to current_epoch
    assert_eq!(mgr.network_id(), nid());
    assert_eq!(mgr.genesis_l1_height(), 42);
}

/// Concurrent reads are allowed: multiple threads can call read-only methods.
#[test]
fn test_concurrent_reads() {
    use std::sync::Arc;
    use std::thread;

    let mgr = Arc::new(EpochManager::new(nid(), 100, root()));
    let mut handles = Vec::new();
    for _ in 0..8 {
        let m = Arc::clone(&mgr);
        handles.push(thread::spawn(move || {
            for _ in 0..1000 {
                let _ = m.current_epoch();
                let _ = m.current_phase();
                let _ = m.network_id();
                let _ = m.genesis_l1_height();
            }
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
}

/// Writes serialize correctly under contention.
#[test]
fn test_exclusive_writes() {
    use std::sync::Arc;
    use std::thread;

    let mgr = Arc::new(EpochManager::new(nid(), 100, root()));
    let mut handles = Vec::new();
    for _ in 0..4 {
        let m = Arc::clone(&mgr);
        handles.push(thread::spawn(move || {
            for _ in 0..250 {
                m.record_block(1, 1).unwrap();
            }
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    let info = mgr.current_epoch_info();
    assert_eq!(info.blocks_produced, 1000);
    assert_eq!(info.total_fees, 1000);
    assert_eq!(info.total_transactions, 1000);
}
