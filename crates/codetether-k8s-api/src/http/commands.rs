//! commands over application-owned Kubernetes services.
use super::HttpState;
use crate::{ScaleRequest, SpawnSubagentRequest};
use axum::{
    Json,
    extract::State,
    http::StatusCode,
};

pub(super) async fn k8s_scale<S: HttpState>(
    State(state): State<S>,
    Json(req): Json<ScaleRequest>,
) -> Result<Json<codetether_k8s::DeployAction>, (StatusCode, String)> {
    crate::k8s_scale(state.manager(), state.audit_log(), req, S::map_error).await
}

pub(super) async fn k8s_restart<S: HttpState>(
    State(state): State<S>,
) -> Result<Json<codetether_k8s::DeployAction>, (StatusCode, String)> {
    crate::k8s_restart(state.manager(), state.audit_log(), S::map_error).await
}

pub(super) async fn k8s_spawn_subagent<S: HttpState>(
    State(state): State<S>,
    Json(req): Json<SpawnSubagentRequest>,
) -> Result<Json<codetether_k8s::DeployAction>, (StatusCode, String)> {
    crate::k8s_spawn_subagent(state.manager(), req, S::map_error).await
}

pub(super) async fn k8s_delete_subagent<S: HttpState>(
    State(state): State<S>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<codetether_k8s::DeployAction>, (StatusCode, String)> {
    crate::k8s_delete_subagent(state.manager(), id, S::map_error).await
}
