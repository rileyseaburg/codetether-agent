//! Kubernetes API spawn operation.
use crate::SpawnSubagentRequest;
use axum::{Json, http::StatusCode};
use codetether_k8s::K8sManager;

/// Execute the spawn API operation using caller-owned services.
/// # Arguments
/// * `manager` - Kubernetes manager to query or mutate.
/// * `req` - Request parameters.
/// * `map_error` - Caller-owned error response policy.
/// # Returns
/// JSON response from the manager operation.
/// # Errors
/// Returns the caller-mapped manager error or a request validation error.
/// # Examples
/// ```no_run
/// # async fn example(manager: &codetether_k8s::K8sManager, req: codetether_k8s_api::SpawnSubagentRequest, map_error: codetether_k8s_api::ErrorMapper) {
/// let _ = codetether_k8s_api::k8s_spawn_subagent(manager, req, map_error).await;
/// # }
/// ```
pub async fn k8s_spawn_subagent(
    manager: &K8sManager,
    req: SpawnSubagentRequest,
    map_error: crate::ErrorMapper,
) -> Result<Json<codetether_k8s::DeployAction>, (StatusCode, String)> {
    manager
        .spawn_subagent_pod_with_spec(
            &req.subagent_id,
            codetether_k8s::SubagentPodSpec {
                image: req.image,
                env_vars: req.env_vars,
                labels: req.labels,
                command: req.command,
                args: req.args,
            },
        )
        .await
        .map(Json)
        .map_err(map_error)
}
