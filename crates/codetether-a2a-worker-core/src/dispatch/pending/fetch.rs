use super::Host;
use anyhow::Result;
/// Fetch tasks at startup or reconnection and dispatch each in order.
/// # Arguments
/// * `host` — Native pending-task retrieval and dispatch services.
/// # Returns
/// Success after all returned tasks have been dispatched.
/// # Errors
/// Propagates retrieval failure before any tasks are dispatched.
/// # Examples
/// ```ignore
/// fetch_pending_tasks(&native_worker).await?;
/// ```
pub async fn fetch_pending_tasks<H: Host>(host: &H) -> Result<()> {
    let tasks = host.pending_tasks().await?;
    tracing::info!(count = tasks.len(), "Found pending tasks");
    for task in tasks {
        host.dispatch(&task).await;
    }
    Ok(())
}
