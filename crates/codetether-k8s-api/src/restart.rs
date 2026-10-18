//! Kubernetes API restart operation.
use axum::{Json, http::StatusCode};
use codetether_audit::{AuditCategory, AuditLog, AuditOutcome};
use codetether_k8s::K8sManager;

/// Execute the restart API operation using caller-owned services.
/// # Arguments
/// * `manager` - Kubernetes manager to query or mutate.
/// * `audit` - Audit sink used before the manager operation.
/// * `map_error` - Caller-owned error response policy.
/// # Returns
/// JSON response from the manager operation.
/// # Errors
/// Returns the caller-mapped manager error or a request validation error.
/// # Examples
/// ```no_run
/// # async fn example(manager: &codetether_k8s::K8sManager, audit: &codetether_audit::AuditLog, map_error: codetether_k8s_api::ErrorMapper) {
/// let _ = codetether_k8s_api::k8s_restart(manager, audit, map_error).await;
/// # }
/// ```
pub async fn k8s_restart(
    manager: &K8sManager,
    audit: &AuditLog,
    map_error: crate::ErrorMapper,
) -> Result<Json<codetether_k8s::DeployAction>, (StatusCode, String)> {
    audit
        .log(
            AuditCategory::K8s,
            "rolling_restart",
            AuditOutcome::Success,
            None,
            None,
        )
        .await;

    manager.rolling_restart().await.map(Json).map_err(map_error)
}
