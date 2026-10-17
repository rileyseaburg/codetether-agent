use super::*;
impl Orchestrator {
    /// Get maximum stage number
    fn max_stage(&self) -> usize {
        self.state.max_stage()
    }

    /// Get subtasks ready to execute (dependencies satisfied)
    pub fn ready_subtasks(&self) -> Vec<&SubTask> {
        self.state.ready_subtasks()
    }

    /// Get subtasks for a specific stage
    pub fn subtasks_for_stage(&self, stage: usize) -> Vec<&SubTask> {
        self.state.subtasks_for_stage(stage)
    }

    /// Get all subtasks
    pub fn all_subtasks(&self) -> Vec<&SubTask> {
        self.state.all_subtasks()
    }

    /// Check if all subtasks are complete
    pub fn is_complete(&self) -> bool {
        self.state.is_complete()
    }
}
