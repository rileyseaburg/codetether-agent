//! Refresh externally edited session state while preserving runtime services.
use crate::session::Session;

impl Session {
    /// Reload the current working window after a persisted context edit.
    /// Pending transcript data must be saved before invoking the editing tool.
    pub(crate) async fn refresh_context_from_store(&mut self) -> anyhow::Result<()> {
        let mut latest = Self::load_tail(&self.id, self.messages.len())
            .await?
            .session;
        latest.max_steps = self.max_steps;
        latest.bus = self.bus.clone();
        latest.metadata.subcall_provider = self.metadata.subcall_provider.clone();
        latest.metadata.subcall_model_name = self.metadata.subcall_model_name.clone();
        *self = latest;
        Ok(())
    }
}
