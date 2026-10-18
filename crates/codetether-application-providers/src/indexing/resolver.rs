//! Resolve embedding credentials and models through the application registry.

use super::{Resolver, provider::ApplicationProvider};
use crate::{Application, ProviderRegistry};
use anyhow::{Context, Result};
use codetether_indexer::{EmbeddingProvider, EmbeddingResolver};
use std::sync::Arc;

#[async_trait::async_trait]
impl<H: Application> EmbeddingResolver for Resolver<ProviderRegistry<H>> {
    async fn resolve(&self, selector: &str) -> Result<(Arc<dyn EmbeddingProvider>, String)> {
        let registry = ProviderRegistry::<H>::from_vault().await?;
        let (provider, model) = registry
            .resolve_model(selector)
            .with_context(|| format!("failed resolving embedding model '{selector}'"))?;
        Ok((Arc::new(ApplicationProvider(provider)), model))
    }
}
