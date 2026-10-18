//! Adapt shared live tool output to the session's bounded event channel.
use codetether_session_events::SessionEvent;
use codetether_tool_progress::Guard;
use tokio::sync::mpsc;

/// Register an invocation with nonblocking session-event delivery.
///
/// # Arguments
/// * `id` - Invocation identifier.
/// * `name` - Tool name included in output events.
/// * `tx` - Session channel; full or closed channels drop the chunk.
/// # Returns
/// A guard that removes the invocation's sink when dropped.
/// # Panics
/// Panics if the shared progress registry mutex was poisoned.
/// # Examples
/// ```rust
/// let (tx, _rx) = tokio::sync::mpsc::channel::<codetether_session_events::SessionEvent<()>>(1);
/// let _guard = codetether_tool_reporting::register_progress("call", "bash", &tx);
/// ```
pub fn register_progress<S: Send + 'static>(
    id: &str,
    name: &str,
    tx: &mpsc::Sender<SessionEvent<S>>,
) -> Guard {
    let tx = tx.clone();
    codetether_tool_progress::register(id, name, move |output| {
        let _ = tx.try_send(SessionEvent::ToolOutputChunk {
            tool_call_id: output.tool_call_id,
            name: output.name,
            stream: output.stream,
            chunk: output.chunk,
        });
    })
}
