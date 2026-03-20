#[derive(Debug, Clone, Default)]
pub struct ClaimProvenance {
    pub worker_id: String,
    pub task_id: String,
    pub run_id: Option<String>,
    pub attempt_id: Option<String>,
    pub tenant_id: Option<String>,
    pub agent_identity_id: Option<String>,
}
