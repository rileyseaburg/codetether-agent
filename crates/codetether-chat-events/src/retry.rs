//! Reconnect status and failed-preview cleanup.

use crate::StreamView;
use codetether_session_events::SessionEvent;

/// Clear failed stream previews and report retry progress.
/// # Arguments
/// Receives frontend state and the owned event or payload fields.
/// # Returns
/// Unrelated events are returned unchanged; matching events are consumed.
/// # Examples
/// ```no_run
/// # fn example<S>(view: &mut impl codetether_chat_events::StreamView, event: codetether_session_events::SessionEvent<S>) {
/// let _ = codetether_chat_events::retry::handle_event(view, event);
/// # }
/// ```
pub fn handle_event<S>(
    view: &mut impl StreamView,
    event: SessionEvent<S>,
) -> Option<SessionEvent<S>> {
    match event {
        SessionEvent::StreamRetry(codetether_session_events::StreamRetryEvent {
            attempt,
            max_restarts,
            ..
        }) => {
            view.clear_streaming_text();
            view.set_status(format!("Reconnecting… {attempt}/{max_restarts}"));
            None
        }
        other => Some(other),
    }
}
