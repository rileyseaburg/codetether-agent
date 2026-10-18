use crate::{
    provider::{Message, ProviderRegistry},
    session::Session,
};
use std::path::PathBuf;
impl codetether_a2a_worker_core::session_protocol::policy::Source for Session {
    type Registry = ProviderRegistry;
    async fn load(&mut self) -> anyhow::Result<ProviderRegistry> {
        ProviderRegistry::for_session(&self.metadata).await
    }
    fn model(&self) -> Option<&str> {
        self.metadata.model.as_deref()
    }
    fn append(&mut self, message: Message) {
        self.add_delegated_message(message);
    }
    fn needs_title(&self) -> bool {
        self.title.is_none()
    }
    async fn title(&mut self, registry: &ProviderRegistry) -> anyhow::Result<()> {
        self.generate_ai_title(registry).await
    }
    fn workspace(&self) -> Option<PathBuf> {
        self.metadata.directory.clone()
    }
    fn session_id(&self) -> &str {
        &self.id
    }
}
