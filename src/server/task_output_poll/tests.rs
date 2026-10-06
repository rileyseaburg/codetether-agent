//! In-process polling contract; no durable or tenant-isolation proof.

use super::snapshot;
use crate::server::KnativeTaskQueue;

mod compatibility;
mod fixtures;
use axum::http::StatusCode;

#[tokio::test]
async fn missing_task_is_not_found() {
    let error = snapshot(&KnativeTaskQueue::new(), "missing")
        .await
        .unwrap_err();
    assert_eq!(error.0, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn polling_exposes_the_accepted_receipt_for_both_terminal_states() {
    for status in ["completed", "failed"] {
        let tasks = KnativeTaskQueue::new();
        tasks.push(fixtures::task()).await;
        let initial = snapshot(&tasks, "task-1").await.unwrap();
        assert!(initial["output"].is_null());
        tasks
            .release_with_completion("task-1", status, fixtures::receipt())
            .await
            .unwrap();
        let output = snapshot(&tasks, "task-1").await.unwrap();
        assert_eq!(output["task_id"], "task-1");
        assert_eq!(output["status"], status);
        assert_eq!(output["title"], "receipt");
        assert_eq!(output["output"], "result");
        assert_eq!(output["result"], "result");
        assert_eq!(output["error"], "diagnostic");
        assert_eq!(output["session_id"], "session-1");
        assert_eq!(output["diagnostics"]["attempt"], 1);
    }
}
