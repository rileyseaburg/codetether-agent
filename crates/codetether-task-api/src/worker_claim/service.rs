//! Atomic worker claim and response projection.
use super::ClaimRequest;
use axum::{Json, http::StatusCode};
use codetether_a2a_worker_core::claim::TaskClaimResponse;
use codetether_task_queue::KnativeTaskQueue;
/// Claim pending work and return the existing worker assignment metadata.
/// # Arguments
/// * `queue` - Process-local task queue.
/// * `req` - Requested task identity.
/// # Returns
/// Claim response after the task transitions to processing.
/// # Errors
/// Returns 404 for missing tasks and 409 for tasks no longer pending.
/// # Examples
/// ```no_run
/// # async fn example(q: &codetether_task_queue::KnativeTaskQueue) {
/// let request = codetether_task_api::worker_claim::ClaimRequest { task_id: "task-1".into() };
/// let _ = codetether_task_api::worker_claim::claim(q, &request).await;
/// # }
/// ```
pub async fn claim(
    queue: &KnativeTaskQueue,
    req: &ClaimRequest,
) -> Result<Json<TaskClaimResponse>, (StatusCode, String)> {
    let task = queue
        .claim(&req.task_id)
        .await
        .map_err(|reason| match reason {
            codetether_task_queue::ClaimError::NotFound => (
                StatusCode::NOT_FOUND,
                format!("Task {} not found", req.task_id),
            ),
            codetether_task_queue::ClaimError::NotPending => (
                StatusCode::CONFLICT,
                format!("Task {} is not pending", req.task_id),
            ),
        })?;

    tracing::info!(task_id = %req.task_id, "Task claimed by worker");

    Ok(Json(TaskClaimResponse {
        task_id: task.task_id,
        worker_id: "assigned".to_string(),
        run_id: None,
        attempt_id: None,
        tenant_id: None,
        user_id: None,
        agent_identity_id: None,
        task_timeout_seconds: None,
        provider_keys: None,
        provider_key_source: None,
    }))
}
