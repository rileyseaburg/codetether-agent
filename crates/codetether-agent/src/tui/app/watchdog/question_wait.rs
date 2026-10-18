//! Native field projection for shared user-question wait policy.
use crate::tui::app::state::App;
pub(crate) fn tick(app: &mut App) {
    codetether_watchdog::interaction::tick(
        app.state.session_id.as_deref(),
        &mut app.state.needs_redraw,
        &mut app.state.watchdog.last_event_at,
    );
}
