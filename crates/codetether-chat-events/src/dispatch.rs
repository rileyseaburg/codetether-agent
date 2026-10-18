//! Dispatcher for text stream events.

use crate::StreamView;
use codetether_session_events::SessionEvent;

/// Apply chat text presentation and timing effects.
/// # Arguments
/// Receives frontend state and the owned event or payload fields.
/// # Returns
/// Unrelated events are returned unchanged; matching events are consumed.
/// # Examples
/// ```no_run
/// # fn example<S>(view: &mut impl codetether_chat_events::StreamView, event: codetether_session_events::SessionEvent<S>) {
/// let _ = codetether_chat_events::handle_text_event(view, event);
/// # }
/// ```
pub fn handle_event<S>(
    view: &mut impl StreamView,
    evt: SessionEvent<S>,
) -> Option<SessionEvent<S>> {
    match evt {
        SessionEvent::TextChunk(chunk) => super::text::chunk(view, chunk),
        SessionEvent::TextComplete(done) => super::text::complete(view, done),
        SessionEvent::ThinkingComplete(done) => super::text::thinking_complete(view, done),
        other => return Some(other),
    }
    None
}
