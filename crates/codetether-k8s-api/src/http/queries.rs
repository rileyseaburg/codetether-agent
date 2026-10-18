//! queries over application-owned Kubernetes services.
use super::HttpState;
use axum::{
    Json,
    extract::State,
    http::StatusCode,
};

// ── K8s self-deployment endpoints ──

pub(super) async fn get_k8s_status<S: HttpState>(
    State(state): State<S>,
) -> Result<Json<codetether_k8s::K8sStatus>, (StatusCode, String)> {
    crate::get_k8s_status(state.manager()).await
}

pub(super) async fn k8s_list_pods<S: HttpState>(
    State(state): State<S>,
) -> Result<Json<Vec<codetether_k8s::PodInfo>>, (StatusCode, String)> {
    crate::k8s_list_pods(state.manager(), S::map_error).await
}

pub(super) async fn k8s_actions<S: HttpState>(
    State(state): State<S>,
) -> Result<Json<Vec<codetether_k8s::DeployAction>>, (StatusCode, String)> {
    crate::k8s_actions(state.manager()).await
}

pub(super) async fn list_connected_workers<S: HttpState>(
    State(state): State<S>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    crate::list_connected_workers(state.manager()).await
}
