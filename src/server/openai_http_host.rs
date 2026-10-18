//! Native provider loading and lookup bindings for OpenAI HTTP requests.
use crate::provider::ProviderRegistry;
use std::sync::Arc;

pub(super) struct AppHost;

impl codetether_openai_serving::http::Host for AppHost {
    type Registry = ProviderRegistry;
    type Error = anyhow::Error;
    async fn load_registry() -> Result<Arc<ProviderRegistry>, anyhow::Error> {
        ProviderRegistry::shared_from_vault().await
    }
}
