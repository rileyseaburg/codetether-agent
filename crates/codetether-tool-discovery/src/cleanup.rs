//! Registry cleanup operation.
use crate::ToolRegistry;
impl ToolRegistry {
    /// Clean up expired tools
    /// # Arguments
    /// No arguments; removes records expired at the current time.
    /// # Returns
    /// Removes expired records from shared state.
    /// # Examples
    /// ```no_run
    /// # async fn example(registry: &codetether_tool_discovery::ToolRegistry) {
    /// let _ = registry.cleanup().await;
    /// # }
    /// ```
    pub async fn cleanup(&self) {
        let mut tools = self.tools.write().await;
        let now = chrono::Utc::now();
        tools.retain(|_, t| t.expires_at > now);
    }
}
