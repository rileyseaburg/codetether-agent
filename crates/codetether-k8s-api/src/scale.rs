//! Kubernetes API scale operation.
use crate::ScaleRequest;
use axum::{Json, http::StatusCode};
use codetether_audit::{AuditCategory, AuditLog, AuditOutcome};
use codetether_k8s::K8sManager;

/// Execute the scale API operation using caller-owned services.
/// # Arguments
/// * `manager` - Kubernetes manager to query or mutate.
/// * `audit` - Audit sink used before the manager operation.
/// * `req` - Request parameters.
/// * `map_error` - Caller-owned error response policy.
/// # Returns
/// JSON response from the manager operation.
/// # Errors
/// Returns the caller-mapped manager error or a request validation error.
/// # Examples
/// ```no_run
/// # async fn example(manager: &codetether_k8s::K8sManager, audit: &codetether_audit::AuditLog, req: codetether_k8s_api::ScaleRequest, map_error: codetether_k8s_api::ErrorMapper) {
/// let _ = codetether_k8s_api::k8s_scale(manager, audit, req, map_error).await;
/// # }
/// ```
pub async fn k8s_scale(
    manager: &K8sManager,
    audit: &AuditLog,
    req: ScaleRequest,
    map_error: crate::ErrorMapper,
) -> Result<Json<codetether_k8s::DeployAction>, (StatusCode, String)> {
    if req.replicas < 0 || req.replicas > 100 {
        return Err((
            StatusCode::BAD_REQUEST,
            "Replicas must be between 0 and 100".to_string(),
        ));
    }

    audit
        .log(
            AuditCategory::K8s,
            format!("scale:{}", req.replicas),
            AuditOutcome::Success,
            None,
            None,
        )
        .await;

    manager
        .scale(req.replicas)
        .await
        .map(Json)
        .map_err(map_error)
}
