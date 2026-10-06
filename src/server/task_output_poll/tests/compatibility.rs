//! Absent results and rejected replacements remain truthful in polling.

use super::{fixtures, snapshot};
use crate::server::{KnativeTaskQueue, task_queue::ReleaseError};

#[tokio::test]
async fn legacy_active_task_has_null_output_without_invented_receipt_fields() {
    let tasks = KnativeTaskQueue::new();
    tasks.push(fixtures::task()).await;
    let output = snapshot(&tasks, "task-1").await.unwrap();
    assert_eq!(output["status"], "working");
    assert!(output["output"].is_null());
    for field in ["result", "error", "session_id", "diagnostics"] {
        assert!(output.get(field).is_none(), "unexpected field: {field}");
    }
}

#[tokio::test]
async fn failed_without_result_preserves_original_session_and_diagnostics() {
    let tasks = KnativeTaskQueue::new();
    tasks.push(fixtures::task()).await;
    let mut receipt = fixtures::receipt();
    receipt.result = None;
    tasks
        .release_with_completion("task-1", "failed", receipt)
        .await
        .unwrap();
    let accepted = snapshot(&tasks, "task-1").await.unwrap();
    assert_eq!(accepted["status"], "failed");
    assert!(accepted["output"].is_null());
    assert!(accepted.get("result").is_none());
    assert_eq!(accepted["error"], "diagnostic");
    assert_eq!(accepted["session_id"], "session-1");
    assert_eq!(accepted["diagnostics"]["attempt"], 1);
    assert_eq!(
        tasks
            .release_with_completion("task-1", "completed", fixtures::receipt())
            .await
            .unwrap_err(),
        ReleaseError::NotActive
    );
    assert_eq!(snapshot(&tasks, "task-1").await.unwrap(), accepted);
}
