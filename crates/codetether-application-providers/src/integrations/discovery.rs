//! discovery provider registry integration.
use crate::{Application, Provider, ProviderRegistry};
use std::sync::Arc;
impl<H: Application> codetether_model_discovery::Registry for ProviderRegistry<H> {
    fn list(&self) -> Vec<&str> {
        ProviderRegistry::list(self)
    }
    fn get(&self, name: &str) -> Option<Arc<dyn Provider>> {
        ProviderRegistry::get(self, name)
    }
}

impl<H: Application> codetether_model_discovery::worker::Host for ProviderRegistry<H> {
    type Registry = Self;
    async fn shared() -> anyhow::Result<Arc<Self>> {
        Self::shared_from_vault().await
    }
    async fn fallback() -> anyhow::Result<Self> {
        let config = codetether_config::Config::load().await.unwrap_or_default();
        Self::from_config(&config).await
    }
}
