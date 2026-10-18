//! Worker progress HTTP extraction; output ownership is shared.
use super::AppState;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use codetether_task_api::output_ingest::TaskOutputPayload;
pub(super) async fn agent_task_output(
    State(state): State<AppState>,
    Path(task_id): Path<String>,
    Json(payload): Json<TaskOutputPayload>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    codetether_task_api::output_ingest::respond(
        &state.knative_tasks,
        &state.bus,
        &task_id,
        &payload,
    )
    .await
}
