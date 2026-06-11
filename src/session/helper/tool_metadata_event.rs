//! Emit structured tool metadata without changing completion text events.

use crate::session::SessionEvent;
use serde_json::Value;
use std::collections::HashMap;
use tokio::sync::mpsc;

pub(in crate::session::helper) async fn send(
    event_tx: &mpsc::Sender<SessionEvent>,
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
