//! Periodic expiry of dynamic tool registrations.
use crate::ToolRegistry;
use std::time::Duration;
/// Spawn registry cleanup at the existing 15-second interval.
/// # Arguments
/// * `tool_registry` - Shared registry whose expired entries are removed.
/// # Returns
/// Handle to the background cleanup task. Dropping it detaches the task.
/// # Panics
/// Panics if called outside a Tokio runtime.
/// # Examples
/// ```no_run
/// # async fn example() {
/// let task = codetether_tool_discovery::spawn_reaper(Default::default());
/// # }
/// ```
pub fn spawn_reaper(tool_registry: ToolRegistry) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(15));
        loop {
            interval.tick().await;
            tool_registry.cleanup().await;
            tracing::debug!("Tool reaper ran cleanup");
        }
    })
}
