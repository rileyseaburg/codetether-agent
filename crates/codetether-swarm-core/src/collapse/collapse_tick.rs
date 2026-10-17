use super::*;

/// One collapse sampling tick output.
#[derive(Debug, Clone, Default)]
///
/// # Examples
/// ```
/// let tick = codetether_swarm_core::collapse::CollapseTick::default();
/// assert!(tick.kills.is_empty());
/// ```
pub struct CollapseTick {
    pub evaluations: Vec<BranchEvaluation>,
    pub kills: Vec<KillDecision>,
    pub promoted_subtask_id: Option<String>,
}
