//! HTTP polling adapter for the shared completion receipt service.
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
    codetether_task_api::output_poll::snapshot(&state.knative_tasks, &task_id)
        .await
        .map(Json)
}
