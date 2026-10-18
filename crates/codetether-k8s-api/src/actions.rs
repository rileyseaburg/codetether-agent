//! Kubernetes API actions operation.
use axum::{Json, http::StatusCode};
use codetether_k8s::K8sManager;

/// Execute the actions API operation using caller-owned services.
/// # Arguments
/// * `manager` - Kubernetes manager to query or mutate.
/// # Returns
/// JSON response from the manager operation.
/// # Examples
/// ```no_run
/// # async fn example(manager: &codetether_k8s::K8sManager) {
/// let _ = codetether_k8s_api::k8s_actions(manager).await;
/// # }
/// ```
pub async fn k8s_actions(
    manager: &K8sManager,
) -> Result<Json<Vec<codetether_k8s::DeployAction>>, (StatusCode, String)> {
    Ok(Json(manager.recent_actions(100).await))
}
