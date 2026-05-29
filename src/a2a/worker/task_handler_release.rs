//! Final release step for claimed task handling.

use anyhow::Result;

use super::{WorkerTaskRuntime, release_task_result, task_timeline};

pub(super) async fn release_handled_task(
    runtime: &WorkerTaskRuntime,
    task_id: &str,
    outcome: super::task_outcome::TaskOutcome,
    timeline: &mut task_timeline::TaskTimeline,
) -> Result<()> {
    timeline.emit_diagnostics();
    let diagnostics = timeline.diagnostics_json();
    timeline.checkpoint(task_timeline::TaskCheckpoint::Releasing);
    release_task_result(
        &runtime.client,
        &runtime.server,
        &runtime.token,
        &runtime.worker_id,
        task_id,
        outcome.status,
        outcome.result,
        outcome.error,
        outcome.session_id,
        Some(diagnostics),
    )
    .await?;
    timeline.checkpoint(task_timeline::TaskCheckpoint::Released);
    runtime.task_progress.lock().await.clear();
    tracing::info!(task_id, status = outcome.status, "Task released");
    Ok(())
}
