//! Native entry points for shared manual watchdog controls.
use crate::tui::app::state::AppState;
/// Dismiss the watchdog notification and stop processing.
pub fn handle_watchdog_cancel(state: &mut AppState) {
    codetether_watchdog::recovery::cancel(state);
}
/// Dismiss the notification after the event loop handles restart.
pub fn handle_watchdog_dismiss(state: &mut AppState) {
    codetether_watchdog::recovery::dismiss(state);
}
