//! Dynamic tool discovery list tools operation.
use crate::{RegisteredToolView, ToolRegistry};
use axum::Json;

/// List all registered tools (active, non-expired)
/// # Arguments
/// * `registry` - Shared dynamic tool registrations.
/// # Returns
/// JSON views of unexpired registrations.
/// # Examples
/// ```no_run
/// # async fn example(registry: &codetether_tool_discovery::ToolRegistry) {
/// let _ = codetether_tool_discovery::list_tools(registry).await;
/// # }
/// ```
pub async fn list_tools(registry: &ToolRegistry) -> Json<Vec<RegisteredToolView>> {
    Json(
        registry
            .list()
            .await
            .into_iter()
            .map(RegisteredToolView::from)
            .collect(),
    )
}
