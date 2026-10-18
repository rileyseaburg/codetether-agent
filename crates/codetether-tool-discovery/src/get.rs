//! Registry get operation.
use crate::RegisteredTool;
use crate::ToolRegistry;
impl ToolRegistry {
    /// Get a tool by ID
    #[allow(dead_code)]
    /// # Arguments
    /// id is the registration identifier to retrieve.
    /// # Returns
    /// A cloned record if present, including expired records.
    /// # Examples
    /// ```no_run
    /// # async fn example(registry: &codetether_tool_discovery::ToolRegistry) {
    /// let _ = registry.get("id").await;
    /// # }
    /// ```
    pub async fn get(&self, id: &str) -> Option<RegisteredTool> {
        let tools = self.tools.read().await;
        tools.get(id).cloned()
    }
}
