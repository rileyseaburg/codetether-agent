//! SubTask construct operations.
use super::*;
impl SubTask {
    /// Create a new subtask
    ///
    /// # Arguments
    /// * `name` - Value used for this operation.
    /// * `instruction` - Value used for this operation.
    /// # Returns
    /// A newly initialized task or agent.
    pub fn new(name: impl Into<String>, instruction: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: name.into(),
            instruction: instruction.into(),
            specialty: None,
            dependencies: Vec::new(),
            priority: 0,
            max_steps: 100,
            context: SubTaskContext::default(),
            status: SubTaskStatus::Pending,
            assigned_agent: None,
            created_at: Utc::now(),
            completed_at: None,
            stage: 0,
            needs_worktree: None,
        }
    }
}
