//! Routed search registration using the application's provider registry.
use super::*;
impl<H: Application> ToolRegistry<H> {
    /// Register routed search backed by the supplied providers.
    /// # Arguments
    /// `providers` is shared with the new routed search tool.
    /// # Returns
    /// Completes after the tool is registered.
    /// # Examples
    /// ```no_run
    /// # fn example<H: codetether_application_registry::Application>(registry: &mut codetether_application_registry::ToolRegistry<H>, providers: std::sync::Arc<H::Providers>) {
    /// registry.register_search(providers);
    /// # }
    /// ```
    pub fn register_search(&mut self, providers: Arc<H::Providers>) {
        H::register_search(self, providers);
    }
}
