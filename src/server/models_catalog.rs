//! Native provider registry bindings for model discovery HTTP requests.
use crate::provider::ProviderRegistry;
use std::sync::Arc;

pub(super) struct AppHost;

impl codetether_model_catalog::http::Host for AppHost {
    type Registry = ProviderRegistry;
    async fn load_registry() -> anyhow::Result<Arc<ProviderRegistry>> {
        ProviderRegistry::shared_from_vault().await
    }
}
