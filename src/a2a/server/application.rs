//! Application persistence and prompt methods for the native A2A session contract.
use crate::provider::ProviderRegistry;
use crate::session::{Session, SessionEvent, SessionResult};
use std::sync::Arc;
use tokio::sync::mpsc;
impl codetether_native_a2a::Session for Session {
    async fn resolve(context_id: Option<&str>) -> anyhow::Result<Self> {
        crate::a2a::session_resolve::resolve_session(context_id).await
    }
    fn id(&self) -> &str {
        &self.id
    }
    fn model_mut(&mut self) -> &mut Option<String> {
        &mut self.metadata.model
    }
    async fn save(&self) -> anyhow::Result<()> {
        Session::save(self).await
    }
    async fn prompt_with_events(
        &mut self,
        prompt: &str,
        sender: mpsc::Sender<SessionEvent>,
        registry: Arc<ProviderRegistry>,
    ) -> anyhow::Result<SessionResult> {
        Session::prompt_with_events(self, prompt, sender, registry).await
    }
}
