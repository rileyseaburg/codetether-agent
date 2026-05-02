//! POST /v1/worker/tasks/claim — claim a pending task.

use crate::a2a::claim::TaskClaimResponse;
use crate::server::AppState;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct ClaimRequest {
    pub task_id: String,
}

/// Claim a task for processing by a worker.
///
/// Marks the task as `"processing"` and returns a
/// [`TaskClaimResponse`] with provenance metadata.
pub async fn worker_task_claim(
    State(state): State<AppState>,
    axum::extract::Json(req): axum::extract::Json<ClaimRequest>,
) -> Result<Json<TaskClaimResponse>, (StatusCode, String)> {
    let updated = state
        .knative_tasks
        .update_status(&req.task_id, "processing")
        .await;

    if !updated {
        return Err((
            StatusCode::NOT_FOUND,
            format!("Task {} not found", req.task_id),
        ));
    }

    let task = state.knative_tasks.get(&req.task_id).await.ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            format!("Task {} not found", req.task_id),
        )
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
    }))
}
