//! Compatibility alias registration over the owned storage registry.
use super::*;
impl<H: Application> ToolRegistry<H> {
    /// Register an additional name for an existing tool implementation.
    /// # Arguments
    /// `id` is the compatibility name; `tool` is the shared implementation.
    /// # Returns
    /// Completes after the compatibility entry is registered.
    /// # Examples
    /// ```no_run
    /// # fn example<H: codetether_application_registry::Application>(registry: &mut codetether_application_registry::ToolRegistry<H>, tool: std::sync::Arc<dyn codetether_tool_core::Tool>) {
    /// registry.register_compat_alias("legacy", tool);
    /// # }
    /// ```
    pub fn register_compat_alias(&mut self, id: &str, tool: Arc<dyn Tool>) {
        self.inner.register_compat_alias(id, tool);
    }
    /// Register the tool's canonical name and a compatibility alias together.
    /// # Arguments
    /// `id` is the compatibility name; `tool` supplies the canonical name.
    /// # Returns
    /// Completes after both entries are registered.
    /// # Examples
    /// ```no_run
    /// # fn example<H: codetether_application_registry::Application>(registry: &mut codetether_application_registry::ToolRegistry<H>, tool: std::sync::Arc<dyn codetether_tool_core::Tool>) {
    /// registry.register_with_alias("legacy", tool);
    /// # }
    /// ```
    pub fn register_with_alias(&mut self, id: &str, tool: Arc<dyn Tool>) {
        self.inner.register_with_alias(id, tool);
    }
}
