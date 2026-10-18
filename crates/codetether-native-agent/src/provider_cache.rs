//! Provider registry access for agent tool actions.
//!
//! Owns the cached provider registry used by agent actions.

use anyhow::Result;
use codetether_application_providers::cache::RegistryCache;
use codetether_native_providers::ProviderRegistry;
use std::sync::Arc;

static PROVIDER_REGISTRY: RegistryCache<
    codetether_native_providers::registry::application::Native,
> = RegistryCache::new();

/// Returns a cached provider registry for agent-tool operations.
/// # Errors
/// Propagates native provider initialization failures.
/// # Examples
/// ```no_run
/// # async fn example() -> anyhow::Result<()> {
/// let providers = codetether_native_agent::provider_cache::get_registry().await?;
/// # Ok(()) }
/// ```
pub async fn get_registry() -> Result<Arc<ProviderRegistry>> {
    PROVIDER_REGISTRY.get().await
}

#[cfg(any(test, feature = "test-support"))]
/// Initialize the same cache used by agent-tool integration tests.
pub async fn set_registry_for_test(registry: Arc<ProviderRegistry>) {
    let _ = PROVIDER_REGISTRY.initialize(registry);
}
