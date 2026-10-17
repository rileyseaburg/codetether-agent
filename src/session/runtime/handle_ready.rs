//! Pause and durable-goal checks before child provider and tool steps.

use super::ControlHandle;

impl ControlHandle {
    pub(crate) async fn ready(&self) -> anyhow::Result<()> {
        while self.is_paused() && !self.is_cancelled() {
            tokio::select! {
                _ = self.cancelled() => {},
                _ = tokio::time::sleep(std::time::Duration::from_millis(100)) => {},
            }
        }
        anyhow::ensure!(!self.is_cancelled(), "Session execution cancelled");
        if let Some(id) = self.session_id() {
            crate::session::runtime::boundary::ensure_running(id).await?;
        }
        Ok(())
    }
}
