//! Dynamic tool discovery tool heartbeat operation.
use crate::{RegisteredToolView, ToolRegistry};
use axum::Json;
use axum::http::StatusCode;

/// Heartbeat endpoint to extend tool TTL
/// # Arguments
/// * `registry` - Shared dynamic tool registrations.
/// * `id` - Registration identifier to refresh.
/// # Returns
/// The refreshed discovery view.
/// # Errors
/// Returns HTTP not found when the tool is not registered.
/// # Examples
/// ```no_run
/// # async fn example(registry: &codetether_tool_discovery::ToolRegistry) {
/// let _ = codetether_tool_discovery::tool_heartbeat(registry, "id".into()).await;
/// # }
/// ```
pub async fn tool_heartbeat(
    registry: &ToolRegistry,
    id: String,
) -> Result<Json<RegisteredToolView>, (StatusCode, String)> {
    registry
        .heartbeat(&id)
        .await
        .map(|tool| {
            tracing::info!(tool_id = %id, "Tool heartbeat received");
            Json(RegisteredToolView::from(tool))
        })
        .ok_or_else(|| (StatusCode::NOT_FOUND, format!("Tool {} not found", id)))
}
