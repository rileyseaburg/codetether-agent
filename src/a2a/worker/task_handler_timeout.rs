//! Task-budget timeout wrapper for claimed task execution.

use std::{future::Future, time::Duration};

use anyhow::{Result, anyhow};

/// Run `future`, failing with a task-scoped error once `timeout` elapses.
pub(super) async fn execute_with_timeout<F, T>(
    task_id: &str,
    timeout: Duration,
    future: F,
) -> Result<T>
where
    F: Future<Output = Result<T>>,
{
    tokio::time::timeout(timeout, future).await.map_err(|_| {
        anyhow!(
            "Task {task_id} execution exceeded {} seconds",
            timeout.as_secs()
        )
    })?
}
