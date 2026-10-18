//! queue request adapters over shared task operations.
use super::HttpState;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use codetether_task_queue::KnativeTask;

/// List all Knative tasks in queue
pub(super) async fn list_knative_tasks<S: HttpState>(
    State(state): State<S>,
) -> Json<Vec<KnativeTask>> {
    crate::list_knative_tasks(state.queue()).await
}

/// Get a specific Knative task by ID
pub(super) async fn get_knative_task<S: HttpState>(
    State(state): State<S>,
    Path(task_id): Path<String>,
) -> Result<Json<KnativeTask>, (StatusCode, String)> {
    crate::get_knative_task(state.queue(), task_id).await
}

/// Claim/start processing a Knative task
pub(super) async fn claim_knative_task<S: HttpState>(
    State(state): State<S>,
    Path(task_id): Path<String>,
) -> Result<Json<KnativeTask>, (StatusCode, String)> {
    crate::claim_knative_task(state.queue(), task_id).await
}

/// Complete a Knative task
pub(super) async fn complete_knative_task<S: HttpState>(
    State(state): State<S>,
    Path(task_id): Path<String>,
) -> Result<Json<KnativeTask>, (StatusCode, String)> {
    crate::complete_knative_task(state.queue(), task_id).await
}
