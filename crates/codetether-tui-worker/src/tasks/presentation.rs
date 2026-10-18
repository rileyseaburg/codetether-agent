use crate::WorkerState;

/// Text to display after removing an incoming task from the worker queue.
///
/// # Examples
/// ```
/// use codetether_tui_worker::tasks::Presentation;
/// let view = Presentation { message: "Incoming task".into(), status: "Received".into() };
/// assert_eq!(view.status, "Received");
/// ```
pub struct Presentation {
    /// System chat message describing the task and its sender.
    pub message: String,
    /// Status line identifying the received task.
    pub status: String,
}

/// Take the next task for display only when the UI is idle.
///
/// # Arguments
/// * `state` - Worker state owning the pending task queue.
/// * `processing` - Whether a prompt is currently running.
/// # Returns
/// Display content for one dequeued task, or `None` if busy or empty.
/// # Examples
/// ```
/// use codetether_tui_worker::{tasks, WorkerState};
/// assert!(tasks::take_next(&mut WorkerState::default(), false).is_none());
/// ```
pub fn take_next(state: &mut WorkerState, processing: bool) -> Option<Presentation> {
    if !processing && let Some(task) = state.dequeue_worker_task() {
        let from_agent = task.from_agent.unwrap_or_else(|| "unknown".to_string());
        Some(Presentation {
            message: format!(
                "Incoming A2A task {} from {}\n{}",
                task.task_id, from_agent, task.message
            ),
            status: format!("Received remote task {}", task.task_id),
        })
    } else {
        None
    }
}
