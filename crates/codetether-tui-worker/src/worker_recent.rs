//! Bounded recent task descriptions.
//! Hosts use WorkerState independently of the bridge transport lifecycle.

impl crate::WorkerState {
    /// Record a task label, retaining only the most recent twenty.
    ///
    /// # Arguments
    /// * `task` — Description to append.
    ///
    /// # Returns
    /// Updates the worker state in place.
    ///
    /// # Examples
    /// ```
    /// let mut state = codetether_tui_worker::WorkerState::default();
    /// state.push_recent_task("review".into());
    /// assert_eq!(state.recent_tasks.len(), 1);
    /// ```

    pub fn push_recent_task(&mut self, task: String) {
        self.recent_tasks.push(task);
        if self.recent_tasks.len() > 20 {
            let drain_len = self.recent_tasks.len() - 20;
            self.recent_tasks.drain(0..drain_len);
        }
    }
}
