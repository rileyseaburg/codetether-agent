//! Claimed task handling and release orchestration.

use anyhow::Result;

use super::{WorkerTaskRuntime, task_metadata, task_str, task_timeout_secs};
use super::{execute_claimed_task, sync_timeline_to_runtime};
use super::{task_claim::claim_task, task_handler_release, task_outcome, task_timeline};

pub(super) async fn handle_task(
    runtime: &WorkerTaskRuntime,
    task: &serde_json::Value,
) -> Result<()> {
    let task_id = task_str(task, "id").ok_or_else(|| anyhow::anyhow!("No task ID"))?;
    let title = task_str(task, "title").unwrap_or("Untitled");
    let mut timeline = task_timeline::TaskTimeline::new(task_id, timeout_secs(task));
    timeline.checkpoint(task_timeline::TaskCheckpoint::TaskReceived);
    tracing::info!(task_id, title, "Handling task");
    sync_timeline_to_runtime(&timeline, runtime).await;

    let Some(claimed) = claim_task(runtime, task_id, &mut timeline).await? else {
        return Ok(());
    };
    let inner = execute_claimed_task(
        runtime,
        task,
        task_id,
        title,
        &claimed.claim_provenance,
        claimed.provider_keys,
        &mut timeline,
    )
    .await;
    let outcome = task_outcome::from_inner(task_id, inner, &mut timeline);
    task_handler_release::release_handled_task(runtime, task_id, outcome, &mut timeline).await
}

fn timeout_secs(task: &serde_json::Value) -> u64 {
    let metadata = task_metadata(task);
    task_timeout_secs(task, &metadata)
}
