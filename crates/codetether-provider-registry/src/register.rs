//! Provider registry register operations.
use super::*;
impl ProviderRegistry {
    /// Register a provider (automatically wrapped with metrics instrumentation).
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use codetether_provider_registry::ProviderRegistry;
    /// use std::sync::Arc;
    /// # fn demo(registry: &mut ProviderRegistry, p: Arc<dyn codetether_provider_core::Provider>) {
    /// registry.register(p);
    /// # }
    /// ```
    pub fn register(&mut self, provider: Arc<dyn Provider>) {
        let name = provider.name().to_string();
        let wrapped = codetether_provider_metrics::MetricsProvider::wrap(provider);
        self.providers.insert(name, wrapped);
    }

    /// Build a registry directly from provider secret records.
    ///
    /// # Arguments
    /// * `secrets` — Provider names mapped to their credential records.
    ///
    /// # Returns
    /// A registry containing successfully constructed, instrumented providers.
    ///
    /// # Examples
    /// ```
    /// # fn empty<H: codetether_provider_factory::Host>() {
    /// let registry = codetether_provider_registry::ProviderRegistry::from_provider_secrets_map::<H>(
    ///     &std::collections::HashMap::new(),
    /// );
    /// assert!(registry.is_empty());
    /// # }
    /// ```
    pub fn from_provider_secrets_map<H: codetether_provider_factory::Host>(
        secrets: &HashMap<String, ProviderSecrets>,
    ) -> Self {
        let mut registry = Self::new();
        for (provider_id, provider_secrets) in secrets {
            if let Some(provider) =
                codetether_provider_factory::dispatch::<H>(provider_id, provider_secrets)
            {
                registry.register(provider);
            }
        }
        registry
    }
}
