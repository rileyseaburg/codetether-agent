//! [`ProviderRegistry`] — name → provider map with resolution.
//!
//! Holds all initialised providers and resolves `"provider/model"` strings
//! to the correct [`Provider`] instance.
//!
//! # Examples
//!
//! ```rust
//! use codetether_agent::provider::ProviderRegistry;
//!
//! let registry = ProviderRegistry::new();
//! assert!(registry.list().is_empty());
//! ```

use super::parse::parse_model_string;
use super::traits::Provider;
use crate::secrets::ProviderSecrets;
use anyhow::Result;
use std::collections::HashMap;
use std::sync::Arc;

/// Registry of available providers.
#[derive(Clone)]
pub struct ProviderRegistry {
    pub(crate) providers: HashMap<String, Arc<dyn Provider>>,
}

impl std::fmt::Debug for ProviderRegistry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ProviderRegistry")
            .field("provider_count", &self.providers.len())
            .field("providers", &self.providers.keys().collect::<Vec<_>>())
            .finish()
    }
}

impl ProviderRegistry {
    /// Create an empty registry.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use codetether_agent::provider::ProviderRegistry;
    /// let registry = ProviderRegistry::new();
    /// assert!(registry.list().is_empty());
    /// ```
    pub fn new() -> Self {
        Self {
            providers: HashMap::new(),
        }
    }

    /// Register a provider (automatically wrapped with metrics instrumentation).
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use codetether_agent::provider::ProviderRegistry;
    /// use std::sync::Arc;
    /// # fn demo(registry: &mut ProviderRegistry, p: Arc<dyn codetether_agent::provider::Provider>) {
    /// registry.register(p);
    /// # }
    /// ```
    pub fn register(&mut self, provider: Arc<dyn Provider>) {
        let name = provider.name().to_string();
        let wrapped = super::metrics::MetricsProvider::wrap(provider);
        self.providers.insert(name, wrapped);
    }

    /// Build a registry directly from provider secret records.
    pub fn from_provider_secrets_map(secrets: &HashMap<String, ProviderSecrets>) -> Self {
        let mut registry = Self::new();
        for (provider_id, provider_secrets) in secrets {
            if let Some(provider) = super::init_dispatch::dispatch(provider_id, provider_secrets) {
                registry.register(provider);
            }
        }
        registry
    }

    /// Get a provider by name.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use codetether_agent::provider::ProviderRegistry;
    /// let registry = ProviderRegistry::new();
    /// assert!(registry.get("openai").is_none());
    /// ```
    pub fn get(&self, name: &str) -> Option<Arc<dyn Provider>> {
        self.providers.get(name).cloned()
    }

    /// List all registered provider names.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use codetether_agent::provider::ProviderRegistry;
    /// let registry = ProviderRegistry::new();
    /// assert!(registry.list().is_empty());
    /// ```
    pub fn list(&self) -> Vec<&str> {
        self.providers.keys().map(|s| s.as_str()).collect()
    }

    /// Resolve a model string to a provider and model name.
    ///
    /// Accepts:
    /// - `"provider/model"` (e.g. `"openai/gpt-4o"`)
    /// - `"model"` alone (uses first available provider)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use codetether_agent::provider::ProviderRegistry;
    ///
    /// let registry = ProviderRegistry::new();
    /// // No providers ⇒ error
    /// assert!(registry.resolve_model("gpt-4o").is_err());
    /// ```
    pub fn resolve_model(&self, model_str: &str) -> Result<(Arc<dyn Provider>, String)> {
        let (provider_name, model) = parse_model_string(model_str);

        if let Some(provider_name) = provider_name {
            let normalized = match provider_name {
                "local-cuda" | "localcuda" => "local_cuda",
                "zhipuai" => "zai",
                other => other,
            };

            let provider = self.providers.get(normalized).cloned().ok_or_else(|| {
                anyhow::anyhow!(
                    "Provider '{}' not found. Available: {:?}",
                    normalized,
                    self.list()
                )
            })?;
            Ok((provider, model.to_string()))
        } else {
            let first_provider = self
                .providers
                .values()
                .next()
                .ok_or_else(|| anyhow::anyhow!("No providers available in registry"))?;
            Ok((first_provider.clone(), model_str.to_string()))
        }
    }
}

impl Default for ProviderRegistry {
    fn default() -> Self {
        Self::new()
    }
}
