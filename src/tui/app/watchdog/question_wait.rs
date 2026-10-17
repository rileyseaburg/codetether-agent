//! Waiting for a human is not a provider stall.
use crate::tui::app::state::{App, AppState};

fn waiting(state: &AppState) -> bool {
    state
        .session_id
        .as_deref()
        .is_some_and(|id| codetether_question::runtime::pending(id).is_some())
}

pub(super) fn skip_stall_check(state: &AppState) -> bool {
    !state.processing
        || state.watchdog_notification.is_some()
        || state.approval_waiting
        || waiting(state)
}

pub(crate) fn tick(app: &mut App) {
    if waiting(&app.state) {
        app.state.needs_redraw = true;
        // Give the provider a fresh inactivity budget after the answer arrives.
        app.state.main_last_event_at = Some(std::time::Instant::now());
    }
}
