//! HTTP extraction and delegation for task-ingress events.
use super::HttpState;
use crate::CloudEventResponse;
use axum::{
    Json,
    extract::State,
    http::{HeaderMap, StatusCode},
};

pub(super) async fn receive_task_event<S: HttpState>(
    State(state): State<S>,
    headers: HeaderMap,
    Json(body): Json<serde_json::Value>,
) -> Result<Json<CloudEventResponse>, (StatusCode, String)> {
    crate::receive_task_event(state.queue(), state.audit_log(), headers, body).await
}
