//! Kubernetes API delete operation.
use axum::{Json, http::StatusCode};
use codetether_k8s::K8sManager;

/// Execute the delete API operation using caller-owned services.
/// # Arguments
/// * `manager` - Kubernetes manager to query or mutate.
/// * `id` - Subagent identifier.
/// * `map_error` - Caller-owned error response policy.
/// # Returns
/// JSON response from the manager operation.
/// # Errors
/// Returns the caller-mapped manager error or a request validation error.
/// # Examples
/// ```no_run
/// # async fn example(manager: &codetether_k8s::K8sManager, id: String, map_error: codetether_k8s_api::ErrorMapper) {
/// let _ = codetether_k8s_api::k8s_delete_subagent(manager, id, map_error).await;
/// # }
/// ```
pub async fn k8s_delete_subagent(
    manager: &K8sManager,
    id: String,
    map_error: crate::ErrorMapper,
) -> Result<Json<codetether_k8s::DeployAction>, (StatusCode, String)> {
    manager
        .delete_subagent_pod(&id)
        .await
        .map(Json)
        .map_err(map_error)
}
