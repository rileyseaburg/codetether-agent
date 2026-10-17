//! Normalize successful and failed task execution results.
use super::*;
/// Preserve task result fields or record a claimed-task failure.
///
/// # Arguments
/// * `task_id` - Task identifier for failure diagnostics.
/// * `inner` - Execution result to normalize.
/// * `timeline` - Progress timeline receiving failure details.
/// # Returns
/// A completion-ready task outcome.
/// # Examples
/// ```rust
/// use codetether_a2a_worker_core::{outcome::from_inner, task_timeline::TaskTimeline};
/// let mut timeline = TaskTimeline::new("task", 60);
/// let outcome = from_inner("task", Ok(("completed", None, None, None)), &mut timeline);
/// assert_eq!(outcome.status, "completed");
/// ```

pub fn from_inner(
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
