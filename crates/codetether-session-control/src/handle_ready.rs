//! Pause and durable-goal checks before child provider and tool steps.

use crate::ControlHandle;

impl ControlHandle {
    /// Wait out transient pauses, then check cancellation and the durable gate.
    /// # Returns
    /// Unit when this control may execute its next provider or tool step.
    /// # Errors
    /// Returns the cancellation error or the attached application's gate error.
    /// # Examples
    /// ```rust,no_run
    /// # async fn example() -> anyhow::Result<()> {
    /// let handle = codetether_session_control::ControlHandle::new();
    /// handle.ready().await?;
    /// # Ok(()) }
    /// ```
    pub async fn ready(&self) -> anyhow::Result<()> {
        while self.is_paused() && !self.is_cancelled() {
            tokio::select! {
                _ = self.cancelled() => {},
                _ = tokio::time::sleep(std::time::Duration::from_millis(100)) => {},
            }
        }
        anyhow::ensure!(!self.is_cancelled(), "Session execution cancelled");
        if let Some(id) = self.session_id() {
            (self.0.gate)(id).await?;
        }
        Ok(())
    }
}
