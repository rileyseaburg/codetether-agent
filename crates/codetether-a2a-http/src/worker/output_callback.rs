//! Output streaming callback for active tasks.
use super::output as request;

use std::sync::Arc;

use codetether_bus_core::AgentBus;

pub use codetether_a2a_worker_core::events::sink::EventSink;

/// Build a plain-text worker output callback.
///
/// # Arguments
/// * `client` - HTTP client for signed output delivery.
/// * `server` - Control-plane URL.
/// * `token` - Optional bearer token.
/// * `worker_id` - Worker sending the output.
/// * `task_id` - Task associated with each emitted message.
/// * `bus` - Shared local bus notified before HTTP submission is spawned.
/// # Returns
/// A thread-safe callback retaining the supplied request context.
/// # Examples
/// ```rust,no_run
/// # fn example(bus: std::sync::Arc<codetether_bus_core::AgentBus>) {
/// use codetether_a2a_http::worker::output_callback::build_output_callback;
/// let callback = build_output_callback(reqwest::Client::new(), "http://localhost:8001".into(), None, "worker".into(), "task".into(), bus);
/// # }
/// ```
pub fn build_output_callback(
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
/// # Arguments
/// * `client` - HTTP client for signed output delivery.
/// * `server` - Control-plane URL.
/// * `token` - Optional bearer token.
/// * `worker_id` - Worker sending the output.
/// * `task_id` - Task associated with each emitted message.
/// * `bus` - Shared local bus notified before HTTP submission is spawned.
/// # Returns
/// A thread-safe callback retaining the supplied request context.
/// # Examples
/// ```rust,no_run
/// # fn example(bus: std::sync::Arc<codetether_bus_core::AgentBus>) {
/// use codetether_a2a_http::worker::output_callback::build_event_sink;
/// let sink = build_event_sink(reqwest::Client::new(), "http://localhost:8001".into(), None, "worker".into(), "task".into(), bus);
/// # }
/// ```
pub fn build_event_sink(
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
            codetether_bus_core::BusMessage::TaskUpdate {
                task_id: task_id.clone(),
                state: codetether_a2a_core::TaskState::Working,
                message: Some(output.clone()),
            },
        );
        tokio::spawn(request::send_with_event(
            client, server, token, worker_id, task_id, output, event,
        ));
    })
}
