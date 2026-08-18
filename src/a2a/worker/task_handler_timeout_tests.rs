//! Tests for the claimed-task timeout wrapper.

use std::{future::pending, time::Duration};

use super::task_handler_timeout::execute_with_timeout;

#[tokio::test]
async fn returns_completed_execution_result() {
    let result = execute_with_timeout("task-ok", Duration::from_secs(1), async {
        Ok::<_, anyhow::Error>("done")
    })
    .await;

    assert_eq!(result.expect("execution should complete"), "done");
}

#[tokio::test]
async fn cancels_execution_after_task_budget() {
    let result = execute_with_timeout(
        "task-stuck",
        Duration::from_millis(10),
        pending::<anyhow::Result<()>>(),
    )
    .await;

    let error = result.expect_err("execution should time out").to_string();
    assert!(error.contains("task-stuck"));
    assert!(error.contains("execution exceeded"));
}
