//! Release a completed task and update its shared progress state.
use super::*;
/// Submit a task outcome with diagnostics and clear progress after acceptance.
///
/// # Arguments
/// * `target` - Worker identity and control-plane destination.
/// * `task_id` - Task to release.
/// * `outcome` - Execution status, result and session metadata.
/// * `timeline` - Timeline supplying diagnostics and release checkpoints.
/// * `task_progress` - Shared progress cleared after a successful response.
/// # Returns
/// Unit after the release succeeds and progress is cleared.
/// # Errors
/// Returns serialization, signing, transport or server rejection errors.
/// # Examples
/// ```rust,no_run
/// # async fn example(target: codetether_a2a_http::worker::completion::ReleaseTarget<'_>, outcome: codetether_a2a_worker_core::outcome::TaskOutcome) -> anyhow::Result<()> {
/// use std::sync::Arc;
/// use tokio::sync::Mutex;
/// use codetether_a2a_worker_core::task_timeline::{TaskTimeline, TaskProgressState};
/// use codetether_a2a_http::worker::completion::release_handled_task;
/// let mut timeline = TaskTimeline::new("task", 60);
/// let progress = Arc::new(Mutex::new(TaskProgressState::default()));
/// release_handled_task(target, "task", outcome, &mut timeline, &progress).await?;
/// # Ok(()) }
/// ```

pub async fn release_handled_task(
    target: ReleaseTarget<'_>,
    task_id: &str,
    outcome: TaskOutcome,
    timeline: &mut task_timeline::TaskTimeline,
    task_progress: &Arc<Mutex<task_timeline::TaskProgressState>>,
) -> Result<()> {
    timeline.emit_diagnostics();
    let diagnostics = timeline.diagnostics_json();
    timeline.checkpoint(task_timeline::TaskCheckpoint::Releasing);
    let request = super::super::build_release_request(
        target.client,
        target.server,
        target.token,
        target.worker_id,
        super::super::Payload {
            task_id,
            status: outcome.status,
            result: outcome.result,
            error: outcome.error,
            session_id: outcome.session_id,
            diagnostics: Some(diagnostics),
        },
    )?;
    super::super::send_release(request).await?;
    timeline.checkpoint(task_timeline::TaskCheckpoint::Released);
    task_progress.lock().await.clear();
    tracing::info!(target: "codetether_agent::a2a::worker::task_handler_release", task_id, status = outcome.status, "Task released");
    Ok(())
}
