//! Bind shared watchdog cancellation feedback to application UI fields.
#[path = "watchdog_ui.rs"]
mod ui;
use crate::tui::{
    app::{state::App, watchdog::WatchdogNotification},
    constants::MAIN_PROCESSING_WATCHDOG_TIMEOUT_SECS,
};
pub(in crate::tui::app::event_loop) fn apply_watchdog_state(
    app: &mut App,
    notification: WatchdogNotification,
) {
    codetether_watchdog::recovery::apply(app, notification, MAIN_PROCESSING_WATCHDOG_TIMEOUT_SECS);
}
