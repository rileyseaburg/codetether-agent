use super::Host;
use anyhow::Result;
/// Poll pending tasks, logging the count only when tasks exist.
/// # Arguments
/// * `host` — Native pending-task retrieval and dispatch services.
/// # Returns
/// Success after all returned tasks have been dispatched.
/// # Errors
/// Propagates retrieval failure before any tasks are dispatched.
/// # Examples
/// ```ignore
/// poll_pending_tasks(&native_worker).await?;
/// ```
pub async fn poll_pending_tasks<H: Host>(host: &H) -> Result<()> {
    let tasks = host.pending_tasks().await?;
    if !tasks.is_empty() {
        tracing::debug!(count = tasks.len(), "Poll found pending tasks");
    }
    for task in &tasks {
        host.dispatch(task).await;
    }
    Ok(())
}
