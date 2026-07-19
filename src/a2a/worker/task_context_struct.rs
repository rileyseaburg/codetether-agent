//! Task execution context struct definition.

pub(super) struct TaskContext {
    pub(super) metadata: serde_json::Map<String, serde_json::Value>,
    pub(super) resume_session_id: Option<String>,
    pub(super) context_id: Option<String>,
    pub(super) preserve_session_workspace: bool,
    pub(super) complexity_hint: Option<String>,
    pub(super) model_tier: Option<String>,
    pub(super) worker_personality: Option<String>,
    pub(super) target_agent_name: Option<String>,
    pub(super) selected_model: Option<String>,
    pub(super) raw_agent: String,
    pub(super) prompt: String,
    pub(super) workspace_id: Option<String>,
    pub(super) is_virtual_task: bool,
}
