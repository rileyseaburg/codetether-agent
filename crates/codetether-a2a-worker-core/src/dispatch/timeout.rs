//! Task-budget timeout wrapper for claimed task execution.

use std::{future::Future, time::Duration};

use anyhow::{Result, anyhow};

/// Run `future`, failing with a task-scoped error once `timeout` elapses.
///
/// # Arguments
/// * `task_id` - Task identifier included in the timeout error.
/// * `timeout` - Maximum execution duration.
/// * `future` - Fallible task operation to await.
/// # Returns
/// The task result if it finishes within the budget.
/// # Errors
/// Propagates task errors or reports the task ID and elapsed budget on timeout.
/// # Examples
/// ```rust,no_run
/// # async fn example() -> anyhow::Result<()> {
/// use std::time::Duration;
/// use codetether_a2a_worker_core::dispatch::execute_with_timeout;
/// let value = execute_with_timeout("task", Duration::from_secs(1), async { Ok(42) }).await?;
/// assert_eq!(value, 42);
/// # Ok(()) }
/// ```
pub async fn execute_with_timeout<F, T>(task_id: &str, timeout: Duration, future: F) -> Result<T>
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
