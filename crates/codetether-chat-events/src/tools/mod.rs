//! Tool-call presentation, status hints and event routing.
//!
//! [handle_event] consumes tool start/completion/heartbeat/approval events.
//! [View] keeps native timing, worker and approval delivery operations outside
//! the shared message construction and status policy.
mod approval_hint;
mod block_hint;
mod complete;
mod start;
mod timing;
mod view;
use codetether_session_events::SessionEvent;
pub use complete::complete;
pub use start::start;
pub use timing::Timing;
pub use view::View;
/// Apply tool-event presentation through frontend state bindings.
/// # Arguments
/// Receives frontend state and owned tool-event data.
/// # Returns
/// Consumes matching events and returns unrelated events unchanged.
/// # Examples
/// ```no_run
/// # async fn example<S>(view: &mut impl codetether_chat_events::tools::View, event: codetether_session_events::SessionEvent<S>) {
/// let _ = codetether_chat_events::tools::handle_event(view, event).await;
/// # }
/// ```
pub async fn handle_event<S>(
    view: &mut impl View,
    evt: SessionEvent<S>,
) -> Option<SessionEvent<S>> {
    match evt {
        SessionEvent::ToolCallStart {
            name, arguments, ..
        } => {
            start(view, name, arguments).await;
        }
        SessionEvent::ToolCallComplete {
            name,
            output,
            success,
            duration_ms,
            ..
        } => {
            complete(view, name, output, success, duration_ms);
        }
        SessionEvent::ApprovalRequest(request) => view.approval_request(request),
        SessionEvent::ToolHeartbeat {
            name, elapsed_secs, ..
        } => {
            view.set_status(format!("Running {name}… {elapsed_secs}s"));
        }
        other => return Some(other),
    }
    None
}
