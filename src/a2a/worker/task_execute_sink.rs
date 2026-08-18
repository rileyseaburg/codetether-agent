//! Structured session-event sink lifecycle for worker task execution.

use super::WorkerTaskRuntime;

/// Install the structured event sink so tool execution emits typed
/// `tool.call` / `tool.result` events instead of flattened text.
pub(super) fn install_task_event_sink(runtime: &WorkerTaskRuntime, task_id: &str) {
    super::session_event_sink::install_sink(Some(super::task_output::build_event_sink(
        runtime.client.clone(),
        runtime.server.clone(),
        runtime.token.clone(),
        runtime.worker_id.clone(),
        task_id.to_string(),
        runtime.bus.clone(),
    )));
}

/// Clear the sink so a later task cannot stream into this one's transcript.
pub(super) fn clear_task_event_sink() {
    super::session_event_sink::install_sink(None);
}
