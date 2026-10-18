//! Instrumented provider registration, lookup and model resolution.
use super::*;
impl<H: Application> ProviderRegistry<H> {
    /// Register a provider with shared metrics instrumentation.
    /// # Arguments
    /// `provider` is registered under its name and wrapped with metrics.
    /// # Returns
    /// Completes after the provider is registered.
    /// # Examples
    /// ```no_run
    /// # fn example<H: codetether_application_providers::Application>(registry: &mut codetether_application_providers::ProviderRegistry<H>, provider: std::sync::Arc<dyn codetether_provider_core::Provider>) {
    /// registry.register(provider);
    /// # }
    /// ```
    pub fn register(&mut self, provider: Arc<dyn Provider>) {
        self.inner.register(provider);
    }
    /// Look up a provider by its registered name.
    /// # Arguments
    /// `name` is the registered provider identifier.
    /// # Returns
    /// The shared provider, or `None` when no provider matches.
    /// # Examples
    /// ```no_run
    /// # fn example<H: codetether_application_providers::Application>() {
    /// let registry = codetether_application_providers::ProviderRegistry::<H>::new();
    /// assert!(registry.get("missing").is_none());
    /// # }
    /// ```
    pub fn get(&self, name: &str) -> Option<Arc<dyn Provider>> {
        self.inner.get(name)
    }
    /// List registered provider names.
    /// # Arguments
    /// Borrows this registry.
    /// # Returns
    /// The registered provider names.
    /// # Examples
    /// ```no_run
    /// # fn example<H: codetether_application_providers::Application>() {
    /// let registry = codetether_application_providers::ProviderRegistry::<H>::new();
    /// assert!(registry.list().is_empty());
    /// # }
    /// ```
    pub fn list(&self) -> Vec<&str> {
        self.inner.list()
    }
    /// Resolve a provider/model selector with the shared resolution policy.
    /// # Arguments
    /// `model_str` is a provider/model selector or a model identifier.
    /// # Returns
    /// The selected provider and its local model identifier.
    /// # Errors
    /// Returns the shared resolver error when no provider can serve the selector.
    /// # Examples
    /// ```no_run
    /// # fn example<H: codetether_application_providers::Application>() {
    /// let registry = codetether_application_providers::ProviderRegistry::<H>::new();
    /// assert!(registry.resolve_model("missing/model").is_err());
    /// # }
    /// ```
    pub fn resolve_model(&self, model_str: &str) -> Result<(Arc<dyn Provider>, String)> {
        self.inner.resolve_model(model_str)
    }
}
