//! Native provider loading and lookup bindings for OpenAI HTTP requests.
use crate::provider::{Provider, ProviderRegistry};
use std::sync::Arc;

pub(super) struct AppHost;

impl codetether_openai_serving::http::Host for AppHost {
    type Registry = ProviderRegistry;
    type Error = anyhow::Error;
    async fn load_registry() -> Result<Arc<ProviderRegistry>, anyhow::Error> {
        ProviderRegistry::shared_from_vault().await
    }
}

impl codetether_openai_serving::http::Registry for ProviderRegistry {
    fn list(&self) -> Vec<&str> {
        ProviderRegistry::list(self)
    }
    fn get(&self, name: &str) -> Option<Arc<dyn Provider>> {
        ProviderRegistry::get(self, name)
    }
}
