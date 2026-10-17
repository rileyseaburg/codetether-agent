//! Per-task registry adapter for shared tenant-key payload handling.
use super::registry::ProviderRegistry;
use anyhow::Result;
pub use codetether_secrets::TenantProviderKeyPayload;
/// Owned task keys; selected strings are cleared before registry owners drop.
#[derive(Debug)]
pub struct PerTaskProviderKeys {
    payload: TenantProviderKeyPayload,
}
impl PerTaskProviderKeys {
    /// Parse a populated tenant-key claim, or return None for absent keys.
    pub fn from_value(value: &serde_json::Value) -> Result<Option<Self>> {
        Ok(TenantProviderKeyPayload::from_value(value)?.map(|payload| Self { payload }))
    }
    /// Return the claim source, falling back to the existing BYOK label.
    pub fn source(&self) -> &str {
        self.payload.source()
    }
    /// Return configured provider identifiers in deterministic order.
    pub fn provider_names(&self) -> Vec<String> {
        self.payload.provider_names()
    }
    /// Construct application providers from the owned task credentials.
    pub fn build_registry(&self) -> ProviderRegistry {
        ProviderRegistry::from_provider_secrets_map(&self.payload.providers)
    }
}
impl Drop for PerTaskProviderKeys {
    fn drop(&mut self) {
        self.payload.clear_key_strings();
    }
}
