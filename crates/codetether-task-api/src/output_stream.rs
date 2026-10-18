//! Task-output SSE transport for a caller-owned queue and bus.
use axum::{
    http::StatusCode,
    response::sse::{Event, KeepAlive, Sse},
};
use futures::Stream;
use std::convert::Infallible;

/// Return 404 for missing tasks, otherwise stream exact-task bus updates.
///
/// # Errors
/// Returns HTTP 404 when the requested task is not in the queue.
/// # Arguments
/// * `tasks` - Queue used to verify the task exists.
/// * `bus` - Bus receiving exact-task updates.
/// * `task_id` - Task identity to stream.
/// # Returns
/// An SSE response using the existing task-event converter and keepalive.
/// # Examples
/// ```no_run
/// # async fn example(tasks: &codetether_task_queue::KnativeTaskQueue, bus: &std::sync::Arc<codetether_bus_core::AgentBus>) {
/// let _ = codetether_task_api::stream_task_output(tasks, bus, "task-1".into()).await;
/// # }
/// ```
pub async fn stream_task_output(
    tasks: &codetether_task_queue::KnativeTaskQueue,
    bus: &std::sync::Arc<codetether_bus_core::AgentBus>,
    task_id: String,
) -> Result<Sse<impl Stream<Item = Result<Event, Infallible>>>, (StatusCode, String)> {
    tasks
        .get(&task_id)
        .await
        .ok_or_else(|| (StatusCode::NOT_FOUND, format!("Task {task_id} not found")))?;
    let rx = bus.handle("task-stream").into_receiver();
    Ok(Sse::new(codetether_bus_sse::task_events(rx, task_id)).keep_alive(KeepAlive::default()))
}
