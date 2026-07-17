#[derive(Clone)]
pub(super) struct TaskInput {
    pub id: Option<String>,
    pub name: String,
    pub instruction: String,
    pub specialty: Option<String>,
    pub needs_worktree: Option<bool>,
}

impl TaskInput {
    pub(super) fn intent_name(&self) -> &str {
        self.id.as_deref().unwrap_or(&self.name)
    }
}
