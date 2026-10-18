//! Native session operations for the shared resume loop.
use crate::provider::ProviderRegistry;
use crate::session::{Session, SessionEvent, SessionResult};
use std::sync::Arc;
use tokio::sync::mpsc;
#[async_trait::async_trait(?Send)]
impl codetether_run_resume::RunSession for Session {
    type Registry = ProviderRegistry;
    fn messages(&self) -> &[crate::provider::Message] {
        &self.messages
    }
    fn set_max_steps(&mut self, max_steps: Option<usize>) {
        self.max_steps = max_steps;
    }
    fn default_max_steps() -> usize {
        crate::session::DEFAULT_MAX_STEPS
    }
    async fn load_registry() -> anyhow::Result<Arc<ProviderRegistry>> {
        ProviderRegistry::shared_from_vault().await
    }
    async fn prompt(&mut self, message: &str) -> anyhow::Result<SessionResult> {
        Session::prompt(self, message).await
    }
    async fn prompt_with_events(
        &mut self,
        message: &str,
        events: mpsc::Sender<SessionEvent>,
        registry: Arc<ProviderRegistry>,
    ) -> anyhow::Result<SessionResult> {
        Session::prompt_with_events(self, message, events, registry).await
    }
}
