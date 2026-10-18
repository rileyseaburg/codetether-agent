//! Structured session-event sink lifecycle for worker task execution.

use crate::WorkerTaskRuntime;
use codetether_a2a_http::worker::output_callback as task_output;
use codetether_a2a_worker_core::events::sink as session_event_sink;

/// Install the structured event sink so tool execution emits typed
/// `tool.call` / `tool.result` events instead of flattened text.
///
/// # Arguments
/// * `runtime` - Worker services retained by the event delivery callback.
/// * `task_id` - Task that receives structured execution events.
///
/// # Examples
/// ```no_run
/// # fn example(runtime: &codetether_native_worker::WorkerTaskRuntime) {
/// codetether_native_worker::event_sink::install_task_event_sink(runtime, "task-id");
/// # }
/// ```
pub fn install_task_event_sink(runtime: &WorkerTaskRuntime, task_id: &str) {
    session_event_sink::install_sink(Some(task_output::build_event_sink(
        runtime.client.clone(),
        runtime.server.clone(),
        runtime.token.clone(),
        runtime.worker_id.clone(),
        task_id.to_string(),
        runtime.bus.clone(),
    )));
}

/// Clear the sink so a later task cannot stream into this one's transcript.
///
/// # Examples
/// ```no_run
/// codetether_native_worker::event_sink::clear_task_event_sink();
/// ```
pub fn clear_task_event_sink() {
    session_event_sink::install_sink(None);
}
