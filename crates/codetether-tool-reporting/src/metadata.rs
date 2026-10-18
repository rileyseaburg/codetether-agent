//! Emit structured tool metadata without changing completion text events.

use codetether_session_events::SessionEvent;
use serde_json::Value;
use std::collections::HashMap;
use tokio::sync::mpsc;

/// Emit a metadata event only for a nonempty metadata map.
/// # Arguments
/// Receives the typed event sender, tool identity and event payload.
/// # Returns
/// Completes after attempting to send the event.
/// # Examples
/// ```no_run
/// # async fn example(tx: &tokio::sync::mpsc::Sender<codetether_session_events::SessionEvent<()>>) {
/// codetether_tool_reporting::send_metadata(tx, "id", "read", None).await;
/// # }
/// ```
pub async fn send<S>(
    event_tx: &mpsc::Sender<SessionEvent<S>>,
    tool_call_id: &str,
    name: &str,
    metadata: Option<&HashMap<String, Value>>,
) {
    let Some(metadata) = metadata else {
        return;
    };
    if metadata.is_empty() {
        return;
    }
    let metadata = serde_json::to_value(metadata).unwrap_or(Value::Null);
    let _ = event_tx
        .send(SessionEvent::ToolCallMetadata {
            tool_call_id: tool_call_id.to_string(),
            name: name.to_string(),
            metadata,
        })
        .await;
}
