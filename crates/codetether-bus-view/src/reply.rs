//! Emit a bus reply to `from_agent` after a remote task completes.

use codetether_worker_events::IncomingTask;

/// Send a completion reply for the active remote task.
/// # Arguments
/// * `active_task` — Task slot consumed before bus lookup and sender validation.
/// * `status` — Completion status included in the reply.
/// * `error` — Optional error replacing the original task text.
/// # Returns
/// Sends when a task, global bus and originating agent are available.
/// # Examples
/// ```
/// let mut task = None;
/// codetether_bus_view::reply::emit(&mut task, "done", None);
/// assert!(task.is_none());
/// ```
pub fn emit(active_task: &mut Option<IncomingTask>, status: &str, error: Option<String>) {
    let Some(task) = active_task.take() else {
        return;
    };
    let Some(bus) = codetether_bus_core::global::global() else {
        return;
    };
    let from_agent = task.from_agent.unwrap_or_default();
    if from_agent.is_empty() {
        return;
    }
    let handle = bus.handle("tui");
    let message = error.as_deref().unwrap_or(&task.message);
    handle.send_to_agent(
        &from_agent,
        vec![codetether_a2a_core::Part::Text {
            text: format!("Task {} {}: {}", task.task_id, status, message),
        }],
    );
    tracing::info!(target: "codetether_agent::tui::app::bus::reply",
        task_id = %task.task_id,
        to = %from_agent,
        status,
        "Emitted bus reply for remote task"
    );
}
