//! Map queue failures to the worker release HTTP contract.

use crate::server::task_queue::ReleaseError;
use axum::http::StatusCode;

pub(super) fn from_release(task_id: &str, reason: ReleaseError) -> (StatusCode, String) {
    let status = match reason {
        ReleaseError::NotFound => StatusCode::NOT_FOUND,
        ReleaseError::NotActive => StatusCode::CONFLICT,
        ReleaseError::InvalidStatus => StatusCode::BAD_REQUEST,
    };
    let message = match reason {
        ReleaseError::NotFound => format!("Task not found: {task_id}"),
        _ => reason.to_string(),
    };
    (status, message)
}
