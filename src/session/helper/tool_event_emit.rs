//! Compact session tool-event emitters.

use crate::session::SessionEvent;
use tokio::sync::mpsc;

pub(in crate::session::helper) async fn start(
    tx: &mpsc::Sender<SessionEvent>,
    id: &str,
    name: &str,
    arguments: String,
) {
    let _ = tx
        .send(SessionEvent::ToolCallStart {
            tool_call_id: id.to_string(),
            name: name.to_string(),
            arguments,
        })
        .await;
}

pub(in crate::session::helper) async fn complete(
    tx: &mpsc::Sender<SessionEvent>,
    id: &str,
    name: &str,
    output: String,
    success: bool,
    duration_ms: u64,
) {
    let _ = tx
        .send(SessionEvent::ToolCallComplete {
            tool_call_id: id.to_string(),
            name: name.to_string(),
            output,
            success,
            duration_ms,
        })
        .await;
}
