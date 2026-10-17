use super::*;
impl CoordinatorState {
    /// Return the highest registered stage, or zero for an empty registry.
    pub fn max_stage(&self) -> usize {
        self.subtasks.values().map(|s| s.stage).max().unwrap_or(0)
    }

    /// Get subtasks ready to execute (dependencies satisfied)
    pub fn ready_subtasks(&self) -> Vec<&SubTask> {
        self.subtasks
            .values()
            .filter(|s| s.status == SubTaskStatus::Pending && s.can_run(&self.completed))
            .collect()
    }

    /// Get subtasks for a specific stage
    pub fn subtasks_for_stage(&self, stage: usize) -> Vec<&SubTask> {
        self.subtasks
            .values()
            .filter(|s| s.stage == stage)
            .collect()
    }

    /// Get all subtasks
    pub fn all_subtasks(&self) -> Vec<&SubTask> {
        self.subtasks.values().collect()
    }

    /// Check if all subtasks are complete
    pub fn is_complete(&self) -> bool {
        self.subtasks.values().all(|s| {
            matches!(
                s.status,
                SubTaskStatus::Completed | SubTaskStatus::Failed | SubTaskStatus::Cancelled
            )
        })
    }
}
