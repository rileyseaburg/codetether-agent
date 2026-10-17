//! Resolve embedding credentials and models through the application registry.

use super::provider_adapter::ApplicationProvider;
use super::*;
use std::sync::Arc;

pub(super) struct ApplicationResolver;

#[async_trait::async_trait]
impl EmbeddingResolver for ApplicationResolver {
    async fn resolve(&self, selector: &str) -> Result<(Arc<dyn EmbeddingProvider>, String)> {
        let registry = crate::provider::ProviderRegistry::from_vault().await?;
        let (provider, model) = registry
            .resolve_model(selector)
            .with_context(|| format!("failed resolving embedding model '{selector}'"))?;
        Ok((Arc::new(ApplicationProvider(provider)), model))
    }
}
