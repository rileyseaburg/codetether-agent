//! Session-scoped task output reads.
use super::registry::AgentTaskRegistry;
use anyhow::Result;

impl AgentTaskRegistry {
    /// Read retained task output, waiting briefly when a running task has no new bytes.
    ///
    /// # Arguments
    /// * `session` - Owning mux session.
    /// * `id` - Task identifier.
    /// * `offset` - First requested byte offset.
    /// # Returns
    /// Output bytes, next offset, running status and optional exit code.
    /// # Errors
    /// Returns an error when the task is absent or belongs to another session.
    /// # Panics
    /// Panics if an internal mutex is poisoned.
    /// # Examples
    /// ```no_run
    /// # async fn example() {
    /// let registry = codetether_mux_agent_task::AgentTaskRegistry::new();
    /// assert!(registry.read("work", "task-1234", 0).await.is_err());
    /// # }
    /// ```
    pub async fn read(
        &self,
        session: &str,
        id: &str,
        offset: u64,
    ) -> Result<(Vec<u8>, u64, bool, Option<i32>)> {
        Ok(self.get(session, id)?.read(offset).await)
    }
}
