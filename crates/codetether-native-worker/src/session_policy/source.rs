//! Delegate session bookkeeping through its existing policy-source contract.
use super::{Native, ProviderRegistry, policy};
use codetether_native_providers::Message;
use std::path::PathBuf;
impl<S: policy::Source<Registry = ProviderRegistry>, T, W> policy::Source for Native<'_, S, T, W> {
    type Registry = ProviderRegistry;
    async fn load(&mut self) -> anyhow::Result<ProviderRegistry> {
        policy::Source::load(self.session).await
    }
    fn model(&self) -> Option<&str> {
        policy::Source::model(self.session)
    }
    fn append(&mut self, message: Message) {
        policy::Source::append(self.session, message);
    }
    fn needs_title(&self) -> bool {
        policy::Source::needs_title(self.session)
    }
    async fn title(&mut self, registry: &ProviderRegistry) -> anyhow::Result<()> {
        policy::Source::title(self.session, registry).await
    }
    fn workspace(&self) -> Option<PathBuf> {
        policy::Source::workspace(self.session)
    }
    fn session_id(&self) -> &str {
        policy::Source::session_id(self.session)
    }
}
