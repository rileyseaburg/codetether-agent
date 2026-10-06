//! HTTP polling adapter for the process-local task completion receipt.

mod response;
mod service;
use service::snapshot;
#[cfg(test)]
mod tests;

use super::AppState;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

pub(super) async fn get_agent_task_output(
    State(state): State<AppState>,
    Path(task_id): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    snapshot(&state.knative_tasks, &task_id).await.map(Json)
}
