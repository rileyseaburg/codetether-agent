//! Kubernetes API status operation.
use axum::{Json, http::StatusCode};
use codetether_k8s::K8sManager;

// ── K8s self-deployment endpoints ──

/// Execute the status API operation using caller-owned services.
/// # Arguments
/// * `manager` - Kubernetes manager to query or mutate.
/// # Returns
/// JSON response from the manager operation.
/// # Examples
/// ```no_run
/// # async fn example(manager: &codetether_k8s::K8sManager) {
/// let _ = codetether_k8s_api::get_k8s_status(manager).await;
/// # }
/// ```
pub async fn get_k8s_status(
    manager: &K8sManager,
) -> Result<Json<codetether_k8s::K8sStatus>, (StatusCode, String)> {
    Ok(Json(manager.status().await))
}
