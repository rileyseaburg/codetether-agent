//! Registry register operation.
use crate::RegisteredTool;
use crate::ToolRegistry;
impl ToolRegistry {
    /// Register a new tool
    /// # Arguments
    /// tool is the registration to insert, replacing the same ID.
    /// # Returns
    /// Updates stored registration metadata.
    /// # Examples
    /// ```no_run
    /// # async fn example(registry: &codetether_tool_discovery::ToolRegistry, tool: codetether_tool_discovery::RegisteredTool) {
    /// registry.register(tool).await;
    /// # }
    /// ```
    pub async fn register(&self, tool: RegisteredTool) {
        let mut tools = self.tools.write().await;
        tools.insert(tool.id.clone(), tool);
    }
}
