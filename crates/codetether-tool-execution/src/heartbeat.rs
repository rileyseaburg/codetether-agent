//! Heartbeat and progress lifetime around one tool execution.
use crate::Host;
use codetether_session_events::SessionEvent;
use codetether_tool_reporting::{ToolTuple, spawn_heartbeat};
use std::time::Instant;
use tokio::sync::mpsc;

/// Execute a tool while keeping its existing liveness and progress reporting.
///
/// # Arguments
/// * `host` — Tool registry, policy, and workspace coordination bindings.
/// * `tool_call_id` — Provider call identifier for heartbeat/progress events.
/// * `tool_name` — Registered tool name.
/// * `input` — Enriched execution arguments.
/// * `session_id` — Session identifier for audit records.
/// * `started` — Start instant for elapsed-time reporting.
/// * `events` — Optional channel for heartbeat and tool progress.
/// # Returns
/// The same tool result tuple returned by [crate::execute].
/// # Panics
/// Panics if called with events outside a Tokio runtime or if progress
/// registration encounters a poisoned registry mutex.
/// # Examples
/// ```no_run
/// # async fn example(host: &impl codetether_tool_execution::Host) {
/// let _result = codetether_tool_execution::execute_with_heartbeat::<()>(
///     host, "call", "read", &serde_json::json!({}), "session",
///     std::time::Instant::now(), None,
/// ).await;
/// # }
/// ```
pub async fn execute_with_heartbeat<S: Send + 'static>(
    host: &impl Host,
    tool_call_id: &str,
    tool_name: &str,
    input: &serde_json::Value,
    session_id: &str,
    started: Instant,
    events: Option<&mpsc::Sender<SessionEvent<S>>>,
) -> ToolTuple {
    let heartbeat = events.map(|events| spawn_heartbeat(events, tool_call_id, tool_name, started));
    let progress = events.map(|events| (events, tool_call_id));
    let result = crate::execute(host, tool_name, input, session_id, started, progress).await;
    if let Some(heartbeat) = heartbeat {
        heartbeat.abort();
    }
    result
}
