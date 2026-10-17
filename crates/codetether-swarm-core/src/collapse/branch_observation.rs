use super::*;

/// Branch observation produced by any execution backend.
#[derive(Debug, Clone)]
///
/// # Examples
/// ```
/// let observation = codetether_swarm_core::collapse::BranchObservation { subtask_id: "task".into(), branch: "work".into(), compile_ok: true, changed_files: Default::default(), changed_lines: 0, resource_health_score: 1.0, infra_unhealthy_signals: 0 };
/// assert!(observation.compile_ok);
/// ```
pub struct BranchObservation {
    pub subtask_id: String,
    pub branch: String,
    pub compile_ok: bool,
    pub changed_files: HashSet<String>,
    pub changed_lines: u32,
    pub resource_health_score: f32,
    pub infra_unhealthy_signals: u32,
}
