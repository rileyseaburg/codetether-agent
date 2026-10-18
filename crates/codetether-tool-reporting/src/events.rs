//! Compact session tool-event emitters.

use codetether_session_events::SessionEvent;
use tokio::sync::mpsc;

/// Emit a tool-start event, ignoring a closed receiver.
/// # Arguments
/// Receives the typed event sender, tool identity and event payload.
/// # Returns
/// Completes after attempting to send the event.
/// # Examples
/// ```no_run
/// # async fn example(tx: &tokio::sync::mpsc::Sender<codetether_session_events::SessionEvent<()>>) {
/// codetether_tool_reporting::start(tx, "id", "read", "{}".into()).await;
/// # }
/// ```
pub async fn start<S>(tx: &mpsc::Sender<SessionEvent<S>>, id: &str, name: &str, arguments: String) {
    let _ = tx
        .send(SessionEvent::ToolCallStart {
            tool_call_id: id.to_string(),
            name: name.to_string(),
            arguments,
        })
        .await;
}

/// Emit a tool-completion event, ignoring a closed receiver.
/// # Arguments
/// Receives the typed event sender, tool identity and event payload.
/// # Returns
/// Completes after attempting to send the event.
/// # Examples
/// ```no_run
/// # async fn example(tx: &tokio::sync::mpsc::Sender<codetether_session_events::SessionEvent<()>>) {
/// codetether_tool_reporting::complete(tx, "id", "read", "ok".into(), true, 1).await;
/// # }
/// ```
pub async fn complete<S>(
    tx: &mpsc::Sender<SessionEvent<S>>,
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
