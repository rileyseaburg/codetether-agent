use super::*;
impl CoordinatorState {
    /// Register a sub-agent for a subtask and increment the spawned count.
    ///
    /// # Arguments
    /// * `subtask` - Task whose ID and specialty identify the agent.
    /// * `model` - Model label copied into the agent.
    /// * `provider` - Provider label copied into the agent.
    ///
    /// # Returns
    /// A clone of the agent stored in the registry.
    ///
    /// # Examples
    /// ```
    /// let mut state = codetether_swarm_core::coordinator::CoordinatorState::default();
    /// let task = codetether_swarm_core::SubTask::new("inspect", "read source");
    /// let agent = state.create_subagent(&task, "model", "provider");
    /// assert_eq!(state.stats().subagents_spawned, 1);
    /// ```
    pub fn create_subagent(&mut self, subtask: &SubTask, model: &str, provider: &str) -> SubAgent {
        let specialty = subtask
            .specialty
            .clone()
            .unwrap_or_else(|| "General".to_string());
        let name = format!("{} Agent", specialty);

        let subagent = SubAgent::new(name, specialty, &subtask.id, model, provider);

        self.subagents.insert(subagent.id.clone(), subagent.clone());
        self.stats.subagents_spawned += 1;

        subagent
    }

    /// Record completion for a known task, updating its status and counters.
    ///
    /// Unknown IDs are ignored. Each call for a known task updates counters;
    /// successful IDs are appended to completion history without deduplication.
    ///
    /// # Arguments
    /// * `subtask_id` - ID of the task being completed.
    /// * `result` - Success and tool-call counts to record.
    ///
    /// # Returns
    /// Nothing; the task status, completion history and statistics are updated.
    ///
    /// # Examples
    /// See [CoordinatorState] for construction. A host passes its execution
    /// result to `state.complete_subtask(&task.id, result)` after a task finishes.
    pub fn complete_subtask(&mut self, subtask_id: &str, result: SubTaskResult) {
        if let Some(subtask) = self.subtasks.get_mut(subtask_id) {
            subtask.complete(result.success);

            if result.success {
                self.completed.push(subtask_id.to_string());
                self.stats.subagents_completed += 1;
            } else {
                self.stats.subagents_failed += 1;
            }

            self.stats.total_tool_calls += result.tool_calls;
        }
    }
}
