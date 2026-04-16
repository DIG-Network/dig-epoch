# DFSP Processing - Normative Requirements

> **Domain:** dfsp_processing
> **Prefix:** DFS
> **Spec reference:** [SPEC.md - Sections 9.1-9.7](../../../resources/SPEC.md)

## Requirements

### Type Aliases

The following type aliases are used throughout the DFSP processing domain:

- `NodeId = Bytes32` — A node identifier is a 32-byte hash (from chia-protocol). Used in storage proof evaluation and issuance distribution.
- `Cid` — Content identifier. References come from external crates (e.g., `cid` crate). Used in CID lifecycle management and fragment reassignment.

### DFS-001: Burn Policy

DfspEpochBurnPolicyV1 MUST contain the following fields:
- `min_bond_threshold: u128` - minimum bond required (mojos)
- `wall_clock_epoch_seconds: u64` - wall-clock duration of one epoch in seconds

DfspEpochBurnContextV1 MUST contain the following fields:
- `elapsed_wall_clock_epochs: u64` - number of wall-clock epochs elapsed since last activity
- `min_bond_threshold: u128` - the resolved minimum bond threshold

protocol_dfsp_epoch_burn_policy() MUST return the default DfspEpochBurnPolicyV1 for the protocol.

resolve_dfsp_epoch_burn_policy_from_schedule(height) MUST resolve the active burn policy from a height-based schedule. The schedule allows protocol parameters to evolve over time via governance-driven height activations.

**Spec reference:** SPEC Section 9.2

### DFS-002: Storage Proof and Issuance

DfspEpochStorageProofEvaluationContextV1 MUST contain the following fields:
- `evaluated_epoch: u64` - the epoch being evaluated
- `proved_node_ids: Vec<Bytes32>` - node IDs that submitted valid storage proofs

DfspEpochIssuancePreviewV1 MUST contain the following fields:
- `proof_shares_by_node: Vec<(Bytes32, u64)>` - per-node proof share counts
- `total_proof_shares: u64` - sum of all proof shares
- `epoch_total_issuance_pool_mojos: u128` - total issuance pool for the epoch (mojos)
- `node_issuance_mojos: Vec<(Bytes32, u128)>` - per-node issuance allocation (mojos)

The issuance preview distributes the epoch's total issuance pool proportionally based on each node's proof shares. Nodes that do not submit valid storage proofs receive no issuance.

**Spec reference:** SPEC Section 9.3

### DFS-003: Boundary Finalize and Staged Outputs

DfspEpochBoundaryFinalizePreviewV1 MUST contain the following fields:
- `evaluated_epoch: u64` - the epoch being finalized
- `reassignment_batch_plan: DfspEpochReassignmentBatchPlanPreviewV1` - the plan for CID reassignment
- `issuance_preview: DfspEpochIssuancePreviewV1` - the issuance distribution preview
- `operations_digest: Bytes32` - digest of all boundary operations

DfspEpochBoundaryStagedOutputsV1 MUST contain the following fields:
- `evaluated_epoch: u64` - the epoch being processed
- `fragment_cids_considered: Vec<Cid>` - fragment CIDs considered during reassignment
- `reassigned_fragments: Vec<Cid>` - fragment CIDs actually reassigned
- `reassignment_batch_plan: DfspEpochReassignmentBatchPlanPreviewV1` - reassignment batch plan
- `issuance_preview: DfspEpochIssuancePreviewV1` - issuance preview
- `finalize_preview: DfspEpochBoundaryFinalizePreviewV1` - finalize preview

DfspExecutionStageV1 MUST define the following enum variants representing pipeline stages:
- `EpochBurn` - burn expired bonds and inactive nodes
- `CollateralAndCid` - process collateral adjustments and CID lifecycle
- `NodeRegistry` - update node registry state
- `Proofs` - evaluate storage proofs
- `Namespace` - process namespace updates
- `FinalizeRoots` - compute final DFSP roots and commitment digest

The 7-stage pipeline MUST execute deterministically: given the same inputs, it MUST produce identical staged outputs.

**Spec reference:** SPEC Section 9.1

### DFS-004: Finalize Roots Commitment Digest

DfspFinalizeRootsCommitmentPreviewV1 MUST contain the following fields:
- 4 DFSP roots (collateral_registry_root, cid_state_root, node_registry_root, namespace_epoch_root)
- `operations_digest: Bytes32` - digest of all boundary operations
- `commitment_digest: Bytes32` - the final commitment digest binding all roots

compute_epoch_boundary_operations_digest_v1(epoch, plan, issuance) -> Bytes32 MUST compute a deterministic digest over the epoch number, reassignment batch plan, and issuance preview using chia-sha2::Sha256.

compute_dfsp_finalize_roots_commitment_digest_v1(roots, operations_digest) -> Bytes32 MUST compute a deterministic digest binding the 4 DFSP roots and the operations digest using chia-sha2::Sha256.

**Spec reference:** SPEC Section 9.4

### DFS-005: DFSP Activation Control

dfsp_activation_height_for_network() -> u64 MUST return DFSP_ACTIVATION_HEIGHT. The default value MUST be u64::MAX (effectively disabled). The value MUST be overridable via the environment variable DIG_DFSP_ACTIVATION_HEIGHT.

is_dfsp_active_at_height(height: u64) -> bool MUST return height >= dfsp_activation_height_for_network().

set_dfsp_activation_height_override(height: Option<u64>) MUST set a runtime override for the DFSP activation height. When set to Some(height), dfsp_activation_height_for_network() returns that height instead of reading the environment variable or constant. When set to None, the override is cleared and normal resolution resumes. This function is intended for testing only.

DFSP operations MUST reject with DfspNotActive error if called before the activation height. This provides a clean governance mechanism for enabling DFSP functionality on a network.

**Spec reference:** SPEC Section 9.7

### DFS-006: Namespace Rollup and Tail Roots

compute_dfsp_namespace_epoch_root_rollup_v1(blocks: &[L2BlockHeader]) -> Result<Bytes32> MUST aggregate the namespace_update_root from each block header into an epoch-level rollup. The rollup produces a single root committing to all namespace updates across the epoch.

dfsp_checkpoint_signing_tail_roots_v1(blocks: &[L2BlockHeader]) -> Result<(Bytes32, Bytes32, Bytes32, Bytes32)> MUST return the 4 DFSP tail roots extracted from the last block header in the epoch. The 4 roots are: collateral_registry_root, cid_state_root, node_registry_root, namespace_epoch_root. MUST return an error if the block list is empty.

**Spec reference:** SPEC Sections 9.5, 9.6

### DFS-007: Parse Burn Policy Schedule

parse_dfsp_epoch_burn_policy_schedule_v1(schedule_str: &str) -> Result<Vec<DfspEpochBurnPolicyScheduleEntryV1>, EpochError> MUST parse a burn policy schedule from a string configuration. Each entry defines an activation height and the burn policy parameters that take effect from that height onward. MUST return an error if the schedule string is malformed. An empty schedule string MUST return an empty vector. The parsed schedule is consumed by resolve_dfsp_epoch_burn_policy_from_schedule() (DFS-001).

**Spec reference:** SPEC Section 9.2

### DFS-008: Storage Proof Evaluation Step

apply_epoch_storage_proof_evaluation_step_v1(context: &DfspEpochStorageProofEvaluationContextV1, ...) -> DfspEpochIssuancePreviewV1 MUST evaluate storage proofs submitted during the epoch, determine which nodes provided valid retrieval proofs, and compute their proof shares for issuance allocation. This is Stage 5 (Proofs) of the 7-stage DFSP epoch-boundary pipeline. MUST distribute the epoch's issuance pool proportionally based on proof shares. Nodes without valid proofs MUST receive zero issuance. MUST be deterministic.

**Spec reference:** SPEC Section 9.3
