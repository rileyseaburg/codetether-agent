//! Play-break session event adapter.

use crate::session::SessionEvent;
use crate::tui::app::state::{App, interlude::InterludeState};

pub(super) fn handle_event(app: &mut App, evt: SessionEvent) -> Option<SessionEvent> {
    match evt {
        SessionEvent::PlayBreak(true) => app.state.interlude = Some(InterludeState::new()),
        SessionEvent::PlayBreak(false) => app.state.interlude = None,
        other => return Some(other),
    }
    app.state.needs_redraw = true;
    None
}
