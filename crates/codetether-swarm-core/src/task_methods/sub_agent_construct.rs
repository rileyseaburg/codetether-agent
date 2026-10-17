//! SubAgent construct operations.
use super::*;
impl SubAgent {
    /// Create a new sub-agent for a subtask
    ///
    /// # Arguments
    /// * `name` - Value used for this operation.
    /// * `specialty` - Value used for this operation.
    /// * `subtask_id` - Value used for this operation.
    /// * `model` - Value used for this operation.
    /// * `provider` - Value used for this operation.
    /// # Returns
    /// A newly initialized task or agent.
    pub fn new(
        name: impl Into<String>,
        specialty: impl Into<String>,
        subtask_id: impl Into<String>,
        model: impl Into<String>,
        provider: impl Into<String>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            name: name.into(),
            specialty: specialty.into(),
            subtask_id: subtask_id.into(),
            status: SubAgentStatus::Initializing,
            steps: 0,
            tool_calls: Vec::new(),
            output: String::new(),
            model: model.into(),
            provider: provider.into(),
            created_at: now,
            last_active: now,
        }
    }
}
