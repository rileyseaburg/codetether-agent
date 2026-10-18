//! Workspace authorization and HTTP adaptation for durable session recovery.
use super::HttpState;
use crate::{RecoverySession, ResumeRequest};
use axum::{
    Json,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
};
use codetether_session_access::{authority, http_error, scope::ScopeError};

pub(super) async fn handle<R: RecoverySession, S: HttpState>(
    State(state): State<S>,
    Path((codebase_id, session_id)): Path<(String, String)>,
    headers: HeaderMap,
    Json(request): Json<ResumeRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let server = state
        .server_url()
        .filter(|server| !server.trim().is_empty())
        .ok_or_else(|| http_error(ScopeError::Unavailable.into()))?;
    let workspace = authority::workspace(server, &codebase_id, &headers)
        .await
        .map_err(http_error)?;
    crate::resume::<R>(&session_id, request, &workspace)
        .await
        .map(Json)
        .map_err(http_error)
}
