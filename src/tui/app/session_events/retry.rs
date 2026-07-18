//! Reconnect status and failed-preview cleanup.

use crate::session::SessionEvent;
use crate::tui::app::state::App;

pub(super) fn handle_event(app: &mut App, event: SessionEvent) -> Option<SessionEvent> {
    match event {
        SessionEvent::StreamRetry(crate::session::StreamRetryEvent {
            attempt,
            max_restarts,
            ..
        }) => {
            app.state.clear_streaming_text();
            app.state.status = format!("Reconnecting… {attempt}/{max_restarts}");
            None
        }
        other => Some(other),
    }
}
