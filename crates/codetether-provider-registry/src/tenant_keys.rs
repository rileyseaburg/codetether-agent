//! Per-task registry adapter for shared tenant-key payload handling.
use crate::ProviderRegistry;
use anyhow::Result;
pub use codetether_secrets::TenantProviderKeyPayload;
/// Owned task keys; selected strings are cleared when this owner drops.
///
/// # Examples
/// ```
/// let keys = codetether_provider_registry::PerTaskProviderKeys::from_value(&serde_json::Value::Null)?;
/// assert!(keys.is_none());
/// # Ok::<(), anyhow::Error>(())
/// ```
#[derive(Debug)]
pub struct PerTaskProviderKeys {
    payload: TenantProviderKeyPayload,
}
impl PerTaskProviderKeys {
    /// Parse a populated tenant-key claim, or return None for absent keys.
    ///
    /// # Arguments
    /// * `value` — Optional provider-key claim encoded as JSON.
    /// # Returns
    /// An owned payload when provider credentials are present.
    /// # Errors
    /// Returns the shared payload parser error for an invalid claim shape.
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
    pub fn build_registry<H: codetether_provider_factory::Host>(&self) -> ProviderRegistry {
        ProviderRegistry::from_provider_secrets_map::<H>(&self.payload.providers)
    }
}
impl Drop for PerTaskProviderKeys {
    fn drop(&mut self) {
        self.payload.clear_key_strings();
    }
}
