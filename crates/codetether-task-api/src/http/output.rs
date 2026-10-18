//! output request adapters over shared task operations.
use super::HttpState;
use axum::response::sse::{Event, Sse};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use futures::stream;
use std::convert::Infallible;

/// Stream task output as SSE events
pub(super) async fn stream_agent_task_output<S: HttpState>(
    State(state): State<S>,
    Path(task_id): Path<String>,
) -> Result<Sse<impl stream::Stream<Item = Result<Event, Infallible>>>, (StatusCode, String)> {
    crate::stream_task_output(state.queue(), state.bus(), task_id).await
}

pub(super) async fn agent_task_output<S: HttpState>(
    State(state): State<S>,
    Path(task_id): Path<String>,
    Json(payload): Json<TaskOutputPayload>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    crate::output_ingest::respond(state.queue(), state.bus(), &task_id, &payload).await
}

pub(super) async fn get_agent_task_output<S: HttpState>(
    State(state): State<S>,
    Path(task_id): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    crate::output_poll::snapshot(state.queue(), &task_id)
        .await
        .map(Json)
}
use crate::output_ingest::TaskOutputPayload;
