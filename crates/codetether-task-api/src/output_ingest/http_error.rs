//! Queue output rejection to HTTP status mapping.

use axum::http::StatusCode;
use codetether_task_queue::OutputError;

pub(super) fn response(error: OutputError, task_id: &str) -> (StatusCode, String) {
    match error {
        OutputError::NotFound => (StatusCode::NOT_FOUND, format!("Task {task_id} not found")),
        OutputError::NotActive => (
            StatusCode::CONFLICT,
            format!("Task {task_id} does not accept output"),
        ),
    }
}
