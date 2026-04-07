//! Cancel and retry key handlers for watchdog notifications.

use crate::tui::app::state::AppState;

/// Dismiss the watchdog notification and stop processing.
pub fn handle_watchdog_cancel(state: &mut AppState) {
    state.processing = false;
    state.streaming_text.clear();
    state.clear_request_timing();
    state.watchdog_notification = None;
    state.status = "Request cancelled.".to_string();
}

/// Dismiss the notification without stopping (restart already handled by event loop).
pub fn handle_watchdog_dismiss(state: &mut AppState) {
    state.watchdog_notification = None;
}
