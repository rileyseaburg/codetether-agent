//! Propagate persisted approval decisions through the active session stream.

use crate::approval::ApprovalStore;
use crate::session::SessionEvent;
use serde_json::json;
use tokio::sync::mpsc::{Sender, error::TrySendError};

pub(super) fn emit(events: &Sender<SessionEvent>, call_id: &str, tool: &str, id: &str) {
    let Some(decision) = ApprovalStore::open_default()
        .and_then(|store| store.decision(id))
        .ok()
        .flatten()
    else {
        return;
    };
    let event = SessionEvent::ToolCallMetadata {
        tool_call_id: call_id.to_string(),
        name: tool.to_string(),
        metadata: json!({"approval_decision": decision}),
    };
    match events.try_send(event) {
        Ok(()) | Err(TrySendError::Closed(_)) => {}
        Err(TrySendError::Full(event)) => send_later(events.clone(), event),
    }
}

fn send_later(events: Sender<SessionEvent>, event: SessionEvent) {
    let Ok(runtime) = tokio::runtime::Handle::try_current() else {
        return;
    };
    runtime.spawn(async move {
        let _ = events.send(event).await;
    });
}
