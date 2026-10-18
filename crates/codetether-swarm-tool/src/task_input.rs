//! Task input normalized from string or object invocation entries.

/// Identity and instructions for one direct swarm worker.
/// # Examples
/// ```
/// let task = codetether_swarm_tool::task_input::TaskInput {
///     id: None, name: "Review".into(), instruction: "Inspect API".into(),
///     specialty: None, needs_worktree: Some(false),
/// };
/// assert_eq!(task.intent_name(), "Review");
/// ```
#[derive(Clone)]
pub struct TaskInput {
    /// Optional stable task identifier.
    pub id: Option<String>,
    /// Display name for the worker.
    pub name: String,
    /// Instructions to execute.
    pub instruction: String,
    /// Optional role used when building the worker prompt.
    pub specialty: Option<String>,
    /// Explicit worktree intent, when supplied.
    pub needs_worktree: Option<bool>,
}

impl TaskInput {
    /// Prefer the stable identifier when classifying task intent.
    pub fn intent_name(&self) -> &str {
        self.id.as_deref().unwrap_or(&self.name)
    }
}
