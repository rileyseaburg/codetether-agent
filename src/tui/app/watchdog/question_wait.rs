//! Native field projection for shared user-question wait policy.
use crate::tui::app::state::{App, AppState};
pub(super) fn skip_stall_check(state: &AppState) -> bool {
    codetether_watchdog::interaction::skip_stall_check(
        state.processing,
        state.watchdog_notification.is_some(),
        state.approval_waiting,
        state.session_id.as_deref(),
    )
}
pub(crate) fn tick(app: &mut App) {
    codetether_watchdog::interaction::tick(
        app.state.session_id.as_deref(),
        &mut app.state.needs_redraw,
        &mut app.state.main_last_event_at,
    );
}
