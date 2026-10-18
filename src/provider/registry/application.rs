//! Native factory choice and process-wide Vault cache.
use super::ProviderRegistry;
use std::sync::Arc;
use tokio::sync::OnceCell;
/// Application provider bindings used by the public registry alias.
pub struct Native;
impl codetether_application_providers::Application for Native {
    type Factory = crate::provider::init_dispatch::host::Native;
    fn cache() -> &'static OnceCell<Arc<ProviderRegistry>> {
        static CACHED: OnceCell<Arc<ProviderRegistry>> = OnceCell::const_new();
        &CACHED
    }
}

impl codetether_application_providers::thinker::Registry for Native {
    async fn think(
        config: &codetether_cognition_core::ThinkerConfig,
        system: &str,
        user: &str,
    ) -> anyhow::Result<codetether_cognition_core::ThinkerOutput> {
        codetether_application_providers::thinker::think::<Self>(config, system, user).await
    }
}
