//! Queue errors have distinct worker-facing HTTP meanings.

use super::super::http_error;
use axum::http::StatusCode;
use codetether_task_queue::ReleaseError;

#[test]
fn release_errors_map_to_http_statuses() {
    for (reason, expected) in [
        (ReleaseError::NotFound, StatusCode::NOT_FOUND),
        (ReleaseError::NotActive, StatusCode::CONFLICT),
        (ReleaseError::InvalidStatus, StatusCode::BAD_REQUEST),
    ] {
        let (status, message) = http_error::from_release("task-1", reason);
        assert_eq!(status, expected);
        assert!(!message.is_empty());
    }
    assert_eq!(
        http_error::from_release("task-1", ReleaseError::NotFound).1,
        "Task not found: task-1"
    );
}
