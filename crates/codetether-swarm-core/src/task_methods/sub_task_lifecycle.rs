//! SubTask lifecycle operations.
use super::*;
impl SubTask {
    /// Check if this subtask can run (all dependencies complete)
    ///
    /// # Arguments
    /// * `completed` - Value used for this operation.
    /// # Returns
    /// Whether all declared dependencies are complete.
    pub fn can_run(&self, completed: &[String]) -> bool {
        self.dependencies.iter().all(|dep| completed.contains(dep))
    }

    /// Mark as running
    ///
    /// # Arguments
    /// * `agent_id` - Value used for this operation.
    /// # Returns
    /// Unit after assigning the agent and running status.
    pub fn start(&mut self, agent_id: &str) {
        self.status = SubTaskStatus::Running;
        self.assigned_agent = Some(agent_id.to_string());
    }

    /// Mark as completed
    ///
    /// # Arguments
    /// * `success` - Value used for this operation.
    /// # Returns
    /// Unit after recording success/failure and completion time.
    pub fn complete(&mut self, success: bool) {
        self.status = if success {
            SubTaskStatus::Completed
        } else {
            SubTaskStatus::Failed
        };
        self.completed_at = Some(Utc::now());
    }
}
