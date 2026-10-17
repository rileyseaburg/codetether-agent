//! Post-agent task finalization helpers.

use anyhow::Result;

use super::FinalizationWorkspace;

use crate::task_timeline;

/// Finalize a task result and publish completed non-virtual workspace changes.
///
/// # Arguments
/// * `workspace` - Optional repository and execution attribution.
/// * `task_id` - Task identifier for publication and diagnostics.
/// * `status` - Execution status, changed to failed on publication failure.
/// * `result` - Existing result, extended with a publication summary on success.
/// * `error` - Error text populated if publication fails.
/// * `is_virtual_task` - Whether repository publication should be skipped.
/// * `timeline` - Progress and deadline accounting updated during finalization.
/// # Returns
/// Unit after synchronizing progress; publication failures are stored in the result fields.
/// # Errors
/// The current implementation records publication errors and returns success.
/// # Examples
/// ```rust,no_run
/// # async fn example() -> anyhow::Result<()> {
/// use codetether_a2a_worker_core::{finalization::{finalize_task_result, FinalizationWorkspace}, task_timeline::TaskTimeline};
/// let mut timeline = TaskTimeline::new("task", 60);
/// let mut status = "completed";
/// let (mut result, mut error) = (None, None);
/// finalize_task_result(FinalizationWorkspace { directory: None, provenance: None }, "task", &mut status, &mut result, &mut error, false, &mut timeline).await?;
/// assert_eq!(status, "completed");
/// # Ok(()) }
/// ```
pub async fn finalize_task_result(
    workspace: FinalizationWorkspace<'_>,
    task_id: &str,
    status: &mut &'static str,
    result: &mut Option<String>,
    error: &mut Option<String>,
    is_virtual_task: bool,
    timeline: &mut task_timeline::TaskTimeline,
) -> Result<()> {
    if timeline.is_near_deadline() {
        tracing::warn!(target: "codetether_agent::a2a::worker::task_finalize",
            task_id,
            elapsed_secs = format!("{:.1}", timeline.elapsed_secs()),
            budget_pct = format!("{:.1}%", timeline.budget_pct_used()),
            "Near deadline after agent completion — will attempt graceful shutdown"
        );
        timeline.checkpoint(task_timeline::TaskCheckpoint::GracefulShutdown);
    }
    if *status == "completed"
        && !is_virtual_task
        && let Some(directory) = workspace.directory
    {
        match super::publication::run(directory, task_id, workspace.provenance, timeline).await {
            Ok(Some(summary)) => {
                *result = Some(match result.take() {
                    Some(existing) if !existing.trim().is_empty() => {
                        format!("{existing}\n\n{summary}")
                    }
                    _ => summary,
                })
            }
            Ok(None) => {}
            Err(err) => {
                tracing::error!(target: "codetether_agent::a2a::worker::task_finalize", task_id, error = %err, "Failed to commit and push task changes");
                *status = "failed";
                *error = Some(format!("Post-agent git commit/push failed: {err}"));
            }
        }
    }
    timeline.sync_progress().await;
    Ok(())
}
