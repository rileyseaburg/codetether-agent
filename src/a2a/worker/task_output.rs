//! Output streaming callback for active tasks.
#[path = "task_output_request.rs"]
mod request;

use std::sync::Arc;

use crate::bus::AgentBus;

/// Streams one line of task output, optionally with a structured session event.
///
/// # Examples
///
/// ```ignore
/// emit(&sink, "[tool:bash:ok] done".into(), Some(event));
/// ```
pub(super) type EventSink =
    Arc<dyn Fn(String, Option<serde_json::Value>) + Send + Sync + 'static>;

pub(super) fn build_output_callback(
    client: reqwest::Client,
    server: String,
    token: Option<String>,
    worker_id: String,
    task_id: String,
    bus: Arc<AgentBus>,
) -> Arc<dyn Fn(String) + Send + Sync + 'static> {
    let sink = build_event_sink(client, server, token, worker_id, task_id, bus);
    Arc::new(move |output: String| sink(output, None))
}

/// Builds a sink that can carry typed session events beside the output text.
///
/// Tool activity previously reached the server only as pre-joined text, so the
/// transcript could never render a tool card. This sink preserves the structure
/// without changing any existing `Fn(String)` callback signature.
pub(super) fn build_event_sink(
    client: reqwest::Client,
    server: String,
    token: Option<String>,
    worker_id: String,
    task_id: String,
    bus: Arc<AgentBus>,
) -> EventSink {
    Arc::new(move |output: String, event: Option<serde_json::Value>| {
        let client = client.clone();
        let server = server.clone();
        let token = token.clone();
        let worker_id = worker_id.clone();
        let task_id = task_id.clone();
        bus.handle("task-output").send(
            format!("task.{task_id}"),
            crate::bus::BusMessage::TaskUpdate {
                task_id: task_id.clone(),
                state: crate::a2a::types::TaskState::Working,
                message: Some(output.clone()),
            },
        );
        tokio::spawn(request::send_with_event(
            client, server, token, worker_id, task_id, output, event,
        ));
    })
}