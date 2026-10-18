//! Provider registry access for agent tool actions.
//!
//! Owns the cached provider registry used by agent actions.

use crate::provider::ProviderRegistry;
use anyhow::Result;
use codetether_application_providers::cache::RegistryCache;
use std::sync::Arc;

static PROVIDER_REGISTRY: RegistryCache<crate::provider::registry::application::Native> =
    RegistryCache::new();

/// Returns a cached provider registry for agent-tool operations.
pub(super) async fn get_registry() -> Result<Arc<ProviderRegistry>> {
    PROVIDER_REGISTRY.get().await
}

#[cfg(test)]
pub(super) async fn set_registry_for_test(registry: Arc<ProviderRegistry>) {
    let _ = PROVIDER_REGISTRY.initialize(registry);
}
