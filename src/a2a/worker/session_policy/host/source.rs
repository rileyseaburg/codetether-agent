use super::Native;
use crate::{
    provider::{Message, ProviderRegistry},
    session::Session,
};
use std::path::PathBuf;
impl codetether_a2a_worker_core::session_protocol::policy::Source for Native<'_> {
    type Registry = ProviderRegistry;
    async fn load(&mut self) -> anyhow::Result<ProviderRegistry> {
        super::super::super::load_task_provider_registry(self.session).await
    }
    fn model(&self) -> Option<&str> {
        self.session.metadata.model.as_deref()
    }
    fn append(&mut self, message: Message) {
        self.session.add_delegated_message(message);
    }
    fn needs_title(&self) -> bool {
        self.session.title.is_none()
    }
    async fn title(&mut self, registry: &ProviderRegistry) -> anyhow::Result<()> {
        self.session.generate_ai_title(registry).await
    }
    fn workspace(&self) -> Option<PathBuf> {
        self.session.metadata.directory.clone()
    }
    fn session_id(&self) -> &str {
        &self.session.id
    }
}
