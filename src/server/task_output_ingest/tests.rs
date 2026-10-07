//! HTTP error and request compatibility contracts for worker progress.

mod completion;
mod event_assertions;
mod fixtures;
mod legacy;
mod ordered;
mod rejected;
mod released;
mod service;
mod service_race;

use super::{http_error, request::TaskOutputPayload};
use crate::server::task_queue::OutputError;
use axum::http::StatusCode;

#[test]
fn rejected_progress_maps_to_http_errors() {
    for (error, status) in [
        (OutputError::NotFound, StatusCode::NOT_FOUND),
        (OutputError::NotActive, StatusCode::CONFLICT),
    ] {
        let response = http_error::response(error, "task-1");
        assert_eq!(response.0, status);
        assert!(response.1.contains("task-1"));
    }
}

#[test]
fn progress_payload_accepts_legacy_optional_fields() {
    let empty: TaskOutputPayload = serde_json::from_str("{}").unwrap();
    assert!(empty.worker_id.is_none());
    assert!(empty.output.is_none());
    let payload: TaskOutputPayload = serde_json::from_value(serde_json::json!({
        "worker_id": "worker-1",
        "output": "progress",
    }))
    .unwrap();
    assert_eq!(payload.worker_id.as_deref(), Some("worker-1"));
    assert_eq!(payload.output.as_deref(), Some("progress"));
}
