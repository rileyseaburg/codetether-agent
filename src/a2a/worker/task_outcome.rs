//! Claimed-task execution outcome mapping.

use anyhow::Result;

use super::task_timeline;

pub(super) type TaskInner = Result<(&'static str, Option<String>, Option<String>, Option<String>)>;

pub(super) struct TaskOutcome {
    pub(super) status: &'static str,
    pub(super) result: Option<String>,
    pub(super) error: Option<String>,
    pub(super) session_id: Option<String>,
}

pub(super) fn from_inner(
    task_id: &str,
    inner: TaskInner,
    timeline: &mut task_timeline::TaskTimeline,
) -> TaskOutcome {
    match inner {
        Ok((status, result, error, session_id)) => TaskOutcome {
            status,
            result,
            error,
            session_id,
        },
        Err(error) => failed(task_id, error, timeline),
    }
}

fn failed(
    task_id: &str,
    error: anyhow::Error,
    timeline: &mut task_timeline::TaskTimeline,
) -> TaskOutcome {
    tracing::error!(task_id, error = %error, "Task failed after claim");
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
