//! Provider registry lookup operations.
use super::*;
impl ProviderRegistry {
    /// Get a provider by name.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use codetether_provider_registry::ProviderRegistry;
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
    /// use codetether_provider_registry::ProviderRegistry;
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
    /// use codetether_provider_registry::ProviderRegistry;
    ///
    /// let registry = ProviderRegistry::new();
    /// // No providers ⇒ error
    /// assert!(registry.resolve_model("gpt-4o").is_err());
    /// ```
    pub fn resolve_model(&self, model_str: &str) -> Result<(Arc<dyn Provider>, String)> {
        codetether_provider_core::routing::resolve_model(&self.providers, model_str)
    }
}
