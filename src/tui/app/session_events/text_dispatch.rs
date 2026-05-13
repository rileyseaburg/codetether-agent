//! Dispatcher for text stream events.

use crate::session::SessionEvent;
use crate::tui::app::state::App;

pub(super) fn handle_event(app: &mut App, evt: SessionEvent) -> Option<SessionEvent> {
    match evt {
        SessionEvent::TextChunk(chunk) => super::text::chunk(app, chunk),
        SessionEvent::TextComplete(done) => super::text::complete(app, done),
        SessionEvent::ThinkingComplete(done) => super::text::thinking_complete(app, done),
        other => return Some(other),
    }
    None
}
