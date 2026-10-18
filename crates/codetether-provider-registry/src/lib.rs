//! Provider collection, metrics wrapping and model resolution.
//!
//! Create [ProviderRegistry], register providers, then resolve a model string.
//! Configuration and Vault construction use the shared provider factory with
//! application-supplied native constructors. Each registration adds metrics once.
use anyhow::Result;
use codetether_provider_core::Provider;
use codetether_secrets::ProviderSecrets;
use std::{collections::HashMap, sync::Arc};
mod construction;
mod create;
mod factory;
mod lookup;
mod register;
/// Provider collection keyed by the original provider name.
///
/// # Examples
/// ```
/// let registry = codetether_provider_registry::ProviderRegistry::new();
/// assert!(registry.list().is_empty());
/// assert!(registry.get("openai").is_none());
/// ```
#[derive(Clone)]
pub struct ProviderRegistry {
    providers: HashMap<String, Arc<dyn Provider>>,
}

impl std::fmt::Debug for ProviderRegistry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ProviderRegistry")
            .field("provider_count", &self.providers.len())
            .field("providers", &self.providers.keys().collect::<Vec<_>>())
            .finish()
    }
}

impl Default for ProviderRegistry {
    fn default() -> Self {
        Self::new()
    }
}

mod tenant_keys;
pub use tenant_keys::PerTaskProviderKeys;
pub mod task;
