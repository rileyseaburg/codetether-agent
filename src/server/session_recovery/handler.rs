//! HTTP adapter for caller-authorized durable session recovery.
use super::{ResumeRequest, authority, http_error, resume, scope::ScopeError};
use crate::server::AppState;
use axum::{
    Json,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
};

/// Resume a session only after the control plane binds caller and codebase.
/// # Errors
/// Returns redacted authority, scope and durable-storage failures.
pub(in crate::server) async fn handle(
    State(state): State<AppState>,
    Path((codebase_id, session_id)): Path<(String, String)>,
    headers: HeaderMap,
    Json(request): Json<ResumeRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let server = state
        .config
        .a2a
        .server_url
        .as_deref()
        .filter(|server| !server.trim().is_empty())
        .ok_or_else(|| http_error(ScopeError::Unavailable.into()))?;
    let workspace = authority::workspace(server, &codebase_id, &headers)
        .await
        .map_err(http_error)?;
    resume(&session_id, request, &workspace)
        .await
        .map(Json)
        .map_err(http_error)
}
