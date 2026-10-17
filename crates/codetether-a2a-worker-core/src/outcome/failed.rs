//! Failure diagnostics for a task that was already claimed.
use super::*;

pub(super) fn failed(
    task_id: &str,
    error: anyhow::Error,
    timeline: &mut task_timeline::TaskTimeline,
) -> TaskOutcome {
    tracing::error!(target: "codetether_agent::a2a::worker::task_outcome", task_id, error = %error, "Task failed after claim");
    timeline.checkpoint_with_detail(
        task_timeline::TaskCheckpoint::Failed,
        Some(format!("{error}")),
    );
    TaskOutcome {
        status: "failed",
        result: None,
        error: Some(format!("Worker error after claim: {error}")),
        session_id: None,
    }
}
