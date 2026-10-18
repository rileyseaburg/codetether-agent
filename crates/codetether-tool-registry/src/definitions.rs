//! Tool registry definitions operations.
use super::*;
impl ToolRegistry {
    /// Project registered tools into definitions and apply the active schema profile.
    ///
    /// # Returns
    /// Definitions in the map's existing iteration order after profile filtering.
    /// # Examples
    /// ```no_run
    /// let registry = codetether_tool_registry::ToolRegistry::new();
    /// assert!(registry.definitions().is_empty());
    /// ```
    pub fn definitions(&self) -> Vec<codetether_provider_core::ToolDefinition> {
        let defs: Vec<_> = self
            .tools
            .values()
            .map(|t| codetether_provider_core::ToolDefinition {
                name: t.id().to_string(),
                description: t.description().to_string(),
                parameters: t.parameters(),
            })
            .collect();
        codetether_tool_core::profile::apply(defs)
    }
}
