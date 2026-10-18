//! Dynamic tool discovery register tool operation.
use crate::{RegisterToolRequest, RegisterToolResponse, RegisteredTool, contract as tool_contract};
use crate::{RegisteredToolView, ToolRegistry};
use axum::Json;
use axum::http::StatusCode;
use std::time::Duration;

/// Register a new tool
/// # Arguments
/// * `registry` - Shared dynamic tool registrations.
/// * `req` - Advertised tool metadata.
/// # Returns
/// The stored discovery-only tool and heartbeat guidance.
/// # Examples
/// ```no_run
/// # async fn example(registry: &codetether_tool_discovery::ToolRegistry, req: codetether_tool_discovery::RegisterToolRequest) {
/// let _ = codetether_tool_discovery::register_tool(registry, req).await;
/// # }
/// ```
pub async fn register_tool(
    registry: &ToolRegistry,
    req: RegisterToolRequest,
) -> Result<Json<RegisterToolResponse>, (StatusCode, String)> {
    let now = chrono::Utc::now();
    let tool = RegisteredTool {
        id: req.id.clone(),
        name: req.name,
        description: req.description,
        version: req.version,
        endpoint: req.endpoint,
        capabilities: req.capabilities,
        parameters: req.parameters,
        execution_mode: tool_contract::registration_execution_mode(),
        registered_at: now,
        last_heartbeat: now,
        expires_at: now + Duration::from_secs(90),
    };

    registry.register(tool.clone()).await;

    tracing::info!(tool_id = %tool.id, "Tool registered");

    Ok(Json(RegisterToolResponse {
        tool: RegisteredToolView::from(tool),
        message: tool_contract::discovery_registration_message(),
    }))
}
