//! Empty, default and provider-backed registry construction.
use super::*;
impl<H: Application> ToolRegistry<H> {
    /// Construct an empty tool and plugin registry.
    /// # Arguments
    /// Application bindings are selected by `H`.
    /// # Returns
    /// An empty registry without batch or search tools.
    /// # Examples
    /// ```no_run
    /// # fn example<H: codetether_application_registry::Application>() {
    /// assert!(codetether_application_registry::ToolRegistry::<H>::new().is_empty());
    /// # }
    /// ```
    pub fn new() -> Self {
        Self {
            _application: PhantomData,
            inner: codetether_tool_registry::ToolRegistry::new(),
        }
    }
    /// Populate the application's default catalog without batch initialization.
    /// # Arguments
    /// Application factories are selected by `H`.
    /// # Returns
    /// The populated default registry.
    /// # Examples
    /// ```no_run
    /// # fn example<H: codetether_application_registry::Application>() {
    /// let _registry = codetether_application_registry::ToolRegistry::<H>::with_defaults();
    /// # }
    /// ```
    pub fn with_defaults() -> Self {
        let mut registry = Self::new();
        H::populate(&mut registry, None);
        registry
    }
    /// Populate the application's provider-backed catalog.
    /// # Arguments
    /// `provider` and `model` configure tools needing model access.
    /// # Returns
    /// The populated registry before shared batch initialization.
    /// # Examples
    /// ```no_run
    /// # fn example<H: codetether_application_registry::Application>(provider: std::sync::Arc<dyn codetether_provider_core::Provider>) {
    /// let _registry = codetether_application_registry::ToolRegistry::<H>::with_provider(provider, "model".into());
    /// # }
    /// ```
    pub fn with_provider(provider: Arc<dyn Provider>, model: String) -> Self {
        let mut registry = Self::new();
        H::populate(&mut registry, Some((provider, model)));
        registry
    }
    /// Populate default tools, register batch, and bind batch through a weak reference.
    /// # Arguments
    /// Application factories are selected by `H`.
    /// # Returns
    /// The shared registry without a strong ownership cycle.
    /// # Examples
    /// ```no_run
    /// # fn example<H: codetether_application_registry::Application>() {
    /// let _registry = codetether_application_registry::ToolRegistry::<H>::with_defaults_arc();
    /// # }
    /// ```
    pub fn with_defaults_arc() -> Arc<Self> {
        codetether_tool_registry::lifecycle::with_defaults::<lifecycle::Policy<H>>()
    }
    /// Populate provider tools, batch and routed search, then bind batch weakly.
    /// # Arguments
    /// `provider` and `model` configure the catalog and routed search.
    /// # Returns
    /// The shared registry with its batch tool bound to this instance.
    /// # Examples
    /// ```no_run
    /// # fn example<H: codetether_application_registry::Application>(provider: std::sync::Arc<dyn codetether_provider_core::Provider>) {
    /// let _registry = codetether_application_registry::ToolRegistry::<H>::with_provider_arc(provider, "model".into());
    /// # }
    /// ```
    pub fn with_provider_arc(provider: Arc<dyn Provider>, model: String) -> Arc<Self> {
        codetether_tool_registry::lifecycle::with_provider::<lifecycle::Policy<H>>(provider, model)
    }
}
impl<H: Application> Default for ToolRegistry<H> {
    fn default() -> Self {
        Self::with_defaults()
    }
}
