//! Claim-adjusted task budget and lifecycle sequencing.
use super::*;
use std::time::Duration;

/// Claim, execute, and release a single task with budget enforcement.
/// # Arguments
/// * `host` - Application runtime operations.
/// * `task` - Task envelope containing an ID, optional title and timeout metadata.
/// # Returns
/// Unit after rejection or after executing and releasing an accepted claim.
/// # Errors
/// Returns missing-ID, claim, or release errors; execution errors become outcomes.
/// # Examples
/// ```rust,no_run
/// # async fn example(host: &impl codetether_a2a_worker_core::task_driver::TaskHandlerHost, task: &serde_json::Value) -> anyhow::Result<()> {
/// codetether_a2a_worker_core::task_driver::handle_task(host, task).await?;
/// # Ok(()) }
/// ```
pub async fn handle_task<H: TaskHandlerHost>(host: &H, task: &serde_json::Value) -> Result<()> {
    let task_id = task_str(task, "id").ok_or_else(|| anyhow::anyhow!("No task ID"))?;
    let title = task_str(task, "title").unwrap_or("Untitled");
    let timeout_secs = timeout_secs(task);
    let mut timeline = task_timeline::TaskTimeline::new(task_id, timeout_secs);
    timeline.checkpoint(task_timeline::TaskCheckpoint::TaskReceived);
    tracing::info!(target: "codetether_agent::a2a::worker::task_handler", task_id, title, "Handling task");
    host.sync_timeline(&timeline).await;

    let Some(claimed) = host.claim(task_id, &mut timeline).await? else {
        return Ok(());
    };
    // The claim response may lower or raise the task budget. Enforce the
    // remaining claim-adjusted budget rather than the pre-claim payload value.
    let execution_timeout = Duration::from_secs_f64(timeline.remaining_secs().max(0.001));
    let inner = execute_with_timeout(
        task_id,
        execution_timeout,
        host.execute(
            task,
            task_id,
            title,
            &claimed.claim_provenance,
            claimed.provider_keys,
            &mut timeline,
        ),
    )
    .await;
    let outcome = crate::outcome::from_inner(task_id, inner, &mut timeline);
    host.release(task_id, outcome, &mut timeline).await
}

fn timeout_secs(task: &serde_json::Value) -> u64 {
    let metadata = task_metadata(task);
    task_timeout_secs(task, &metadata)
}
