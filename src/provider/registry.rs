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

use super::traits::Provider;
use crate::secrets::ProviderSecrets;
use anyhow::Result;
use std::collections::HashMap;
use std::sync::Arc;

/// Registry of available providers.
#[derive(Clone)]
pub struct ProviderRegistry {
    pub(crate) inner: codetether_provider_registry::ProviderRegistry,
}

impl std::fmt::Debug for ProviderRegistry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.inner, f)
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
            inner: codetether_provider_registry::ProviderRegistry::new(),
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
        self.inner.register(provider);
    }

    /// Build a registry directly from provider secret records.
    pub fn from_provider_secrets_map(secrets: &HashMap<String, ProviderSecrets>) -> Self {
        Self {
            inner: codetether_provider_registry::ProviderRegistry::from_provider_secrets_map::<
                super::init_dispatch::host::Native,
            >(secrets),
        }
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
        self.inner.get(name)
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
        self.inner.list()
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
        self.inner.resolve_model(model_str)
    }
}

impl Default for ProviderRegistry {
    fn default() -> Self {
        Self::new()
    }
}
