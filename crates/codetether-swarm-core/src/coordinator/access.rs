use super::*;
impl CoordinatorState {
    /// Insert one task without discarding prior tasks or completion history.
    ///
    /// # Arguments
    /// * `task` - Task to index by its ID.
    ///
    /// # Returns
    /// Nothing; an existing task with the same ID is replaced.
    ///
    /// # Examples
    /// ```
    /// let mut state = codetether_swarm_core::coordinator::CoordinatorState::default();
    /// state.insert_task(codetether_swarm_core::SubTask::new("inspect", "read"));
    /// assert_eq!(state.all_subtasks().len(), 1);
    /// ```
    pub fn insert_task(&mut self, task: SubTask) {
        self.subtasks.insert(task.id.clone(), task);
    }
    /// Replace the task registry, retaining agents, completion history and statistics.
    ///
    /// # Arguments
    /// * `tasks` - Tasks already indexed by ID.
    ///
    /// # Returns
    /// Nothing; only the task registry is replaced.
    ///
    /// # Examples
    /// ```
    /// let mut state = codetether_swarm_core::coordinator::CoordinatorState::default();
    /// state.replace_tasks(Default::default());
    /// assert!(state.all_subtasks().is_empty());
    /// ```
    pub fn replace_tasks(&mut self, tasks: HashMap<String, SubTask>) {
        self.subtasks = tasks;
    }

    /// Get statistics
    pub fn stats(&self) -> &SwarmStats {
        &self.stats
    }

    /// Get mutable statistics
    pub fn stats_mut(&mut self) -> &mut SwarmStats {
        &mut self.stats
    }
}
