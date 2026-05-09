//! Provider registry access for agent tool actions.
//!
//! Owns the cached provider registry used by agent actions.

use crate::provider::ProviderRegistry;
use anyhow::Result;
use std::sync::Arc;
use tokio::sync::OnceCell;

static PROVIDER_REGISTRY: OnceCell<Arc<ProviderRegistry>> = OnceCell::const_new();

/// Returns a cached provider registry for agent-tool operations.
pub(super) async fn get_registry() -> Result<Arc<ProviderRegistry>> {
    PROVIDER_REGISTRY
        .get_or_try_init(|| async { Ok(Arc::new(ProviderRegistry::from_vault().await?)) })
        .await
        .cloned()
}

#[cfg(test)]
pub(super) async fn set_registry_for_test(registry: Arc<ProviderRegistry>) {
    let _ = PROVIDER_REGISTRY.set(registry);
}
