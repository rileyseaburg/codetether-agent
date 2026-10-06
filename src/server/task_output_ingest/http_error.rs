//! Queue output rejection to HTTP status mapping.

use crate::server::task_queue::OutputError;
use axum::http::StatusCode;

pub(super) fn response(error: OutputError, task_id: &str) -> (StatusCode, String) {
    match error {
        OutputError::NotFound => (StatusCode::NOT_FOUND, format!("Task {task_id} not found")),
        OutputError::NotActive => (
            StatusCode::CONFLICT,
            format!("Task {task_id} does not accept output"),
        ),
    }
}
