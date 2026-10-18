//! Session-scoped task cancellation and server shutdown cleanup.
use super::registry::AgentTaskRegistry;
use anyhow::Result;

impl AgentTaskRegistry {
    /// Interrupt a running task after checking its owning session.
    ///
    /// # Arguments
    /// * `session` - Owning mux session.
    /// * `id` - Task identifier.
    /// # Returns
    /// Returns unit after signaling, or immediately if the task has exited.
    /// # Errors
    /// Returns an error for absent/foreign tasks or a failed cancellation signal.
    /// # Panics
    /// Panics if an internal mutex is poisoned.
    /// # Examples
    /// ```
    /// let registry = codetether_mux_agent_task::AgentTaskRegistry::new();
    /// assert!(registry.cancel("work", "task-1234").is_err());
    /// ```
    pub fn cancel(&self, session: &str, id: &str) -> Result<()> {
        let task = self.get(session, id)?;
        if task.running() {
            super::cancel::send(task.pid)?;
        }
        Ok(())
    }

    /// Interrupt every running turn owned by `session`, ignoring signal failures.
    ///
    /// # Panics
    /// Panics if the store mutex is poisoned.
    pub fn cancel_session(&self, session: &str) {
        for task in self.store.lock().unwrap().values() {
            if task.running() && task.session == session {
                let _ = super::cancel::send(task.pid);
            }
        }
    }

    /// Interrupt all running turns, ignoring individual signal failures.
    ///
    /// # Panics
    /// Panics if the store mutex is poisoned.
    pub fn cancel_all(&self) {
        for task in self.store.lock().unwrap().values() {
            if task.running() {
                let _ = super::cancel::send(task.pid);
            }
        }
    }
}
