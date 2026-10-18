//! Tool registry alias operations.
use super::*;
impl ToolRegistry {
    /// Register a compatibility identifier delegating to the supplied tool.
    ///
    /// # Arguments
    /// * `id` — Alias exposed in tool lookup and definitions.
    /// * `inner` — Tool receiving delegated execution.
    /// # Returns
    /// Replaces any existing entry registered under the alias.
    /// # Examples
    /// ```no_run
    /// let mut registry = codetether_tool_registry::ToolRegistry::new();
    /// registry.register_compat_alias("legacy", std::sync::Arc::new(codetether_tool_core::InvalidTool::new()));
    /// assert!(registry.contains("legacy"));
    /// ```
    pub fn register_compat_alias(&mut self, id: &str, inner: Arc<dyn Tool>) {
        self.register(Arc::new(codetether_tool_core::AliasTool::new(id, inner)));
    }
}

impl ToolRegistry {
    /// Register a tool and a compatibility alias over the same Arc instance.
    ///
    /// # Arguments
    /// * `id` — Compatibility alias.
    /// * `tool` — Shared tool instance to register under both identities.
    /// # Returns
    /// Inserts the primary entry first and the alias second.
    /// # Examples
    /// ```no_run
    /// let mut registry = codetether_tool_registry::ToolRegistry::new();
    /// registry.register_with_alias("legacy", std::sync::Arc::new(codetether_tool_core::InvalidTool::new()));
    /// assert!(registry.contains("legacy"));
    /// ```
    pub fn register_with_alias(&mut self, id: &str, tool: Arc<dyn Tool>) {
        self.register(Arc::clone(&tool));
        self.register_compat_alias(id, tool);
    }
}
