//! Task execution context struct definition.

/// Resolved execution context produced by [`super::build_task_context`].
///
/// # Examples
/// ```rust
/// use codetether_a2a_worker_core::task_config::build_task_context;
/// let context = build_task_context(&serde_json::json!({"prompt": "review"}), "fallback");
/// assert_eq!(context.prompt, "review");
/// ```
pub struct TaskContext {
    pub metadata: serde_json::Map<String, serde_json::Value>,
    pub resume_session_id: Option<String>,
    pub context_id: Option<String>,
    pub preserve_session_workspace: bool,
    pub complexity_hint: Option<String>,
    pub model_tier: Option<String>,
    pub worker_personality: Option<String>,
    pub target_agent_name: Option<String>,
    pub selected_model: Option<String>,
    pub raw_agent: String,
    pub prompt: String,
    pub workspace_id: Option<String>,
    pub is_virtual_task: bool,
}
