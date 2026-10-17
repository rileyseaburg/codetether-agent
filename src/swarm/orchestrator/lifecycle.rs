use super::*;
impl Orchestrator {
    /// Create a sub-agent for a subtask
    pub fn create_subagent(&mut self, subtask: &SubTask) -> SubAgent {
        self.state
            .create_subagent(subtask, &self.model, &self.provider)
    }

    /// Mark a subtask as completed
    pub fn complete_subtask(&mut self, subtask_id: &str, result: SubTaskResult) {
        self.state.complete_subtask(subtask_id, result)
    }
}
