//! Native provider registry bindings for model discovery HTTP requests.
use crate::provider::{Provider, ProviderRegistry};
use std::sync::Arc;

pub(super) struct AppHost;

impl codetether_model_catalog::http::Host for AppHost {
    type Registry = ProviderRegistry;
    async fn load_registry() -> anyhow::Result<Arc<ProviderRegistry>> {
        ProviderRegistry::shared_from_vault().await
    }
}

impl codetether_model_catalog::discovery::CatalogRegistry for ProviderRegistry {
    fn list(&self) -> Vec<&str> {
        ProviderRegistry::list(self)
    }
    fn get(&self, name: &str) -> Option<Arc<dyn Provider>> {
        ProviderRegistry::get(self, name)
    }
}
