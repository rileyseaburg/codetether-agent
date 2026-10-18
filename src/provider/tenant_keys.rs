//! Native compatibility wrapper for shared task credential ownership.
use super::registry::ProviderRegistry;
use anyhow::Result;
pub use codetether_secrets::TenantProviderKeyPayload;
mod conversion;
mod platform;
/// Task-owned keys using the native provider factory and registry.
pub type PerTaskProviderKeys = codetether_provider_registry::BoundTaskKeys<
    super::init_dispatch::host::Native,
    ProviderRegistry,
>;
impl ProviderRegistry {
    /// Prefer usable task credentials, otherwise load the cached platform registry.
    pub(crate) async fn for_task_keys(keys: Option<&serde_json::Value>) -> Result<Self> {
        codetether_provider_registry::task::load_task_provider_registry::<
            super::init_dispatch::host::Native,
        >(keys)
        .await
        .map(|inner| Self { inner })
    }
}
