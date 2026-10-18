//! Registry list operation.
use crate::RegisteredTool;
use crate::ToolRegistry;
impl ToolRegistry {
    /// List all tools (excluding expired)
    /// # Arguments
    /// No arguments; reads the current time to filter expired records.
    /// # Returns
    /// Cloned unexpired registrations in map iteration order.
    /// # Examples
    /// ```no_run
    /// # async fn example(registry: &codetether_tool_discovery::ToolRegistry) {
    /// let _ = registry.list().await;
    /// # }
    /// ```
    pub async fn list(&self) -> Vec<RegisteredTool> {
        let tools = self.tools.read().await;
        let now = chrono::Utc::now();
        tools
            .values()
            .filter(|t| t.expires_at > now)
            .cloned()
            .collect()
    }
}
