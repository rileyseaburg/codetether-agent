//! Tool collection lookup, mutation and metadata access.
use super::*;
impl<H: Application> ToolRegistry<H> {
    /// Get a reference to the plugin registry for managing signed plugins.
    pub fn plugins(&self) -> &PluginRegistry {
        self.inner.plugins()
    }

    /// Register a tool
    pub fn register(&mut self, tool: Arc<dyn Tool>) {
        self.inner.register(tool);
    }

    /// Get a tool by ID
    pub fn get(&self, id: &str) -> Option<Arc<dyn Tool>> {
        self.inner.get(id)
    }

    /// List all tool IDs
    pub fn list(&self) -> Vec<&str> {
        self.inner.list()
    }

    /// Get tool definitions for LLM
    pub fn definitions(&self) -> Vec<codetether_provider_core::ToolDefinition> {
        self.inner.definitions()
    }

    /// Register multiple tools at once
    pub fn register_all(&mut self, tools: Vec<Arc<dyn Tool>>) {
        self.inner.register_all(tools);
    }

    /// Remove a tool by ID
    pub fn unregister(&mut self, id: &str) -> Option<Arc<dyn Tool>> {
        self.inner.unregister(id)
    }

    /// Check if a tool exists
    pub fn contains(&self, id: &str) -> bool {
        self.inner.contains(id)
    }

    /// Get the number of registered tools
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Check if the registry is empty
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}
