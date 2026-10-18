//! Native play-break state binding for session events.
use crate::{session::SessionEvent, tui::app::state::App};
pub(super) fn handle_event(app: &mut App, evt: SessionEvent) -> Option<SessionEvent> {
    codetether_interlude::interactive::handle_event(
        &mut app.state.interlude,
        &mut app.state.needs_redraw,
        evt,
    )
}
