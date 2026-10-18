//! Native Vault and config registry loading for worker inventories.
use crate::provider::ProviderRegistry;
use anyhow::Result;
use std::sync::Arc;

pub(super) struct Native;
impl codetether_model_discovery::worker::Host for Native {
    type Registry = ProviderRegistry;
    async fn shared() -> Result<Arc<ProviderRegistry>> {
        ProviderRegistry::shared_from_vault().await
    }
    async fn fallback() -> Result<ProviderRegistry> {
        let config = crate::config::Config::load().await.unwrap_or_default();
        ProviderRegistry::from_config(&config).await
    }
}
