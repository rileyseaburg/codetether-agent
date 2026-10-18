//! Tool registry collection operations.
use super::*;
impl ToolRegistry {
    /// Register a tool
    pub fn register(&mut self, tool: Arc<dyn Tool>) {
        self.tools.insert(tool.id().to_string(), tool);
    }

    /// Get a tool by ID
    pub fn get(&self, id: &str) -> Option<Arc<dyn Tool>> {
        self.tools.get(id).cloned()
    }

    /// List all tool IDs
    pub fn list(&self) -> Vec<&str> {
        self.tools.keys().map(|s| s.as_str()).collect()
    }

    /// Register multiple tools at once
    pub fn register_all(&mut self, tools: Vec<Arc<dyn Tool>>) {
        for tool in tools {
            self.register(tool);
        }
    }

    /// Remove a tool by ID
    pub fn unregister(&mut self, id: &str) -> Option<Arc<dyn Tool>> {
        self.tools.remove(id)
    }

    /// Check if a tool exists
    pub fn contains(&self, id: &str) -> bool {
        self.tools.contains_key(id)
    }

    /// Get the number of registered tools
    pub fn len(&self) -> usize {
        self.tools.len()
    }

    /// Check if the registry is empty
    pub fn is_empty(&self) -> bool {
        self.tools.is_empty()
    }
}
