//! Native compatibility wrapper for shared task credential ownership.
use super::registry::ProviderRegistry;
use anyhow::Result;
pub use codetether_secrets::TenantProviderKeyPayload;
mod platform;
/// Task-owned keys cleared by the shared payload owner on drop.
pub struct PerTaskProviderKeys(codetether_provider_registry::PerTaskProviderKeys);
impl std::fmt::Debug for PerTaskProviderKeys {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl PerTaskProviderKeys {
    /// Parse a populated provider-key claim.
    pub fn from_value(value: &serde_json::Value) -> Result<Option<Self>> {
        codetether_provider_registry::PerTaskProviderKeys::from_value(value)
            .map(|keys| keys.map(Self))
    }
    /// Return the claim source or its default label.
    pub fn source(&self) -> &str {
        self.0.source()
    }
    /// Return deterministic provider identifiers.
    pub fn provider_names(&self) -> Vec<String> {
        self.0.provider_names()
    }
    /// Construct native providers from the owned credentials.
    pub fn build_registry(&self) -> ProviderRegistry {
        ProviderRegistry {
            inner: self
                .0
                .build_registry::<super::init_dispatch::host::Native>(),
        }
    }
}
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
