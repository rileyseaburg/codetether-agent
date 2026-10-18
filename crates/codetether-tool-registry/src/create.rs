//! Tool registry create operations.
use super::*;
impl ToolRegistry {
    /// Create an empty tool map with the environment-configured plugin registry.
    ///
    /// # Returns
    /// A registry ready for application tool construction.
    /// # Examples
    /// ```no_run
    /// let registry = codetether_tool_registry::ToolRegistry::new();
    /// assert!(registry.is_empty());
    /// ```
    pub fn new() -> Self {
        Self {
            tools: HashMap::new(),
            plugin_registry: PluginRegistry::from_env(),
        }
    }

    /// Get a reference to the plugin registry for managing signed plugins.
    pub fn plugins(&self) -> &PluginRegistry {
        &self.plugin_registry
    }
}

impl Default for ToolRegistry {
    fn default() -> Self {
        Self::new()
    }
}
