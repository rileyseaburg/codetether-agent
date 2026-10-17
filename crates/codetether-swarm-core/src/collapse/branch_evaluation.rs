use super::*;

/// Full evaluation output for one branch.
#[derive(Debug, Clone)]
///
/// # Examples
/// ```
/// use codetether_swarm_core::collapse::{BranchEvaluation, CoherenceScore};
/// let score = CoherenceScore { compile_health: 1.0, test_health: 1.0, contract_alignment: 1.0, diff_conflict_risk: 0.0, velocity: 1.0, resource_health_score: 1.0, invariant_breaks: 0 };
/// let evaluation = BranchEvaluation { subtask_id: "task".into(), branch: "work".into(), aggregate_score: score.collapse_score(), score };
/// assert_eq!(evaluation.subtask_id, "task");
/// ```
pub struct BranchEvaluation {
    pub subtask_id: String,
    pub branch: String,
    pub score: CoherenceScore,
    pub aggregate_score: f32,
}
