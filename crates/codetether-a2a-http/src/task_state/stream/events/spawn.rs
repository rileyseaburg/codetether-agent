//! Spawn event forwarding while preserving shared task ownership.
use super::*;

/// Start forwarding with owned copies of the current bus and task state.
///
/// # Arguments
///
/// Supply the optional bus, task table, task id, event receiver, and a callback
/// mapping accepted events to JSON. Returning `None` skips an event.
///
/// # Returns
///
/// Starts a detached task that drains the receiver until all senders close.
///
/// # Panics
///
/// Panics if called outside a Tokio runtime.
///
/// # Examples
///
/// ```rust,no_run
/// # async fn demo() {
/// use std::sync::Arc;
/// let (_, rx) = tokio::sync::mpsc::channel::<serde_json::Value>(8);
/// let tasks = Arc::new(dashmap::DashMap::new());
/// codetether_a2a_http::task_state::stream::events::spawn(
///     &None, &tasks, &"task-1".to_string(), rx, |event| Some(event.clone()),
/// );
/// # }
/// ```
pub fn spawn<Event: Send + 'static>(
    bus: &Option<Arc<AgentBus>>,
    tasks: &Arc<DashMap<String, Task>>,
    task_id: &String,
    event_rx: mpsc::Receiver<Event>,
    event_data: fn(&Event) -> Option<serde_json::Value>,
) {
    let (bus_clone, activity_tasks) = (bus.clone(), tasks.clone());
    let task_id_clone = task_id.clone();
    tokio::spawn(forward(
        event_rx,
        bus_clone,
        activity_tasks,
        task_id_clone,
        event_data,
    ));
}
