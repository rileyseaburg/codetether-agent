//! HTTP adapters over registry operations.
use crate::{HttpState, RegisterToolRequest, RegisterToolResponse, RegisteredToolView};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

pub(super) async fn list_tools<S: HttpState>(
    State(state): State<S>,
) -> Json<Vec<RegisteredToolView>> {
    crate::list_tools(state.tool_registry()).await
}

pub(super) async fn register_tool<S: HttpState>(
    State(state): State<S>,
    Json(req): Json<RegisterToolRequest>,
) -> Result<Json<RegisterToolResponse>, (StatusCode, String)> {
    crate::register_tool(state.tool_registry(), req).await
}

pub(super) async fn tool_heartbeat<S: HttpState>(
    State(state): State<S>,
    Path(id): Path<String>,
) -> Result<Json<RegisteredToolView>, (StatusCode, String)> {
    crate::tool_heartbeat(state.tool_registry(), id).await
}
