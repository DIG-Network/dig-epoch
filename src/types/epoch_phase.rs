//! # `types::epoch_phase` — `EpochPhase` enum and `PhaseTransition` struct
//!
//! **Implemented by:** `TYP-001` — SPEC §3.2-3.3.

use serde::{Deserialize, Serialize};

/// Sentinel marker proving the module exists and is reachable at
/// `dig_epoch::types::epoch_phase::STR_002_MODULE_PRESENT`.
#[doc(hidden)]
pub const STR_002_MODULE_PRESENT: () = ();

/// The lifecycle phase of a DIG L2 epoch, driven by L1 block-height progress.
///
/// Phase progress is `(l1_now - epoch_start_l1) * 100 / EPOCH_L1_BLOCKS`:
///
/// | Range | Phase |
/// |-------|-------|
/// | 0 – 49 % | `BlockProduction` |
/// | 50 – 74 % | `Checkpoint` |
/// | 75 – 99 % | `Finalization` |
/// | ≥ 100 % | `Complete` |
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EpochPhase {
    /// Block production (0–50 % of L1 window).
    BlockProduction,
    /// Checkpoint submission (50–75 % of L1 window).
    Checkpoint,
    /// Finalization (75–100 % of L1 window).
    Finalization,
    /// Epoch complete (≥ 100 % of L1 window).
    Complete,
}

impl EpochPhase {
    /// Ordinal index: `BlockProduction=0`, `Checkpoint=1`, `Finalization=2`, `Complete=3`.
    pub fn index(&self) -> usize {
        match self {
            Self::BlockProduction => 0,
            Self::Checkpoint => 1,
            Self::Finalization => 2,
            Self::Complete => 3,
        }
    }

    /// Successor phase, or `None` if already `Complete`.
    pub fn next(&self) -> Option<EpochPhase> {
        match self {
            Self::BlockProduction => Some(Self::Checkpoint),
            Self::Checkpoint => Some(Self::Finalization),
            Self::Finalization => Some(Self::Complete),
            Self::Complete => None,
        }
    }

    /// Predecessor phase, or `None` if already `BlockProduction`.
    pub fn previous(&self) -> Option<EpochPhase> {
        match self {
            Self::Complete => Some(Self::Finalization),
            Self::Finalization => Some(Self::Checkpoint),
            Self::Checkpoint => Some(Self::BlockProduction),
            Self::BlockProduction => None,
        }
    }

    /// Canonical string name used in Display and logging.
    pub fn name(&self) -> &'static str {
        match self {
            Self::BlockProduction => "BlockProduction",
            Self::Checkpoint => "Checkpoint",
            Self::Finalization => "Finalization",
            Self::Complete => "Complete",
        }
    }

    /// Returns `true` only during `BlockProduction`.
    pub fn allows_block_production(&self) -> bool {
        matches!(self, Self::BlockProduction)
    }

    /// Returns `true` only during `Checkpoint`.
    pub fn allows_checkpoint_submission(&self) -> bool {
        matches!(self, Self::Checkpoint)
    }

    /// Returns `true` only during `Finalization`.
    pub fn allows_finalization(&self) -> bool {
        matches!(self, Self::Finalization)
    }
}

impl std::fmt::Display for EpochPhase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

/// Records the details of a phase transition event.
#[derive(Debug, Clone)]
pub struct PhaseTransition {
    /// Epoch number in which the transition occurred.
    pub epoch: u64,
    /// Phase before the transition.
    pub from: EpochPhase,
    /// Phase after the transition.
    pub to: EpochPhase,
    /// L1 block height at which the transition was observed.
    pub l1_height: u32,
}
