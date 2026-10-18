/// Deterministic kill decision.
#[derive(Debug, Clone)]
///
/// # Examples
/// ```
/// let decision = codetether_swarm_core::collapse::KillDecision { subtask_id: "task".into(), branch: "work".into(), reason: "threshold exceeded".into() };
/// assert_eq!(decision.subtask_id, "task");
/// ```
pub struct KillDecision {
    pub subtask_id: String,
    pub branch: String,
    pub reason: String,
}
