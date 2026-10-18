//! Registry heartbeat operation.
use crate::RegisteredTool;
use crate::ToolRegistry;
use std::time::Duration;
impl ToolRegistry {
    /// Update heartbeat for a tool (extends TTL by 90s)
    /// # Arguments
    /// id identifies the registration whose expiry should be extended.
    /// # Returns
    /// The refreshed record, or None when the ID is unknown.
    /// # Examples
    /// ```no_run
    /// # async fn example(registry: &codetether_tool_discovery::ToolRegistry) {
    /// let _ = registry.heartbeat("id").await;
    /// # }
    /// ```
    pub async fn heartbeat(&self, id: &str) -> Option<RegisteredTool> {
        let mut tools = self.tools.write().await;
        if let Some(tool) = tools.get_mut(id) {
            let now = chrono::Utc::now();
            tool.last_heartbeat = now;
            tool.expires_at = now + Duration::from_secs(90);
            return Some(tool.clone());
        }
        None
    }
}
