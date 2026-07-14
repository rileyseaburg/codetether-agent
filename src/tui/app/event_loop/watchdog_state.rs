//! Reset app state after the watchdog cancels a stalled request.
//!
//! Extracted into a standalone module so the per-tick fast-path
//! (`tick_watchdog`) can reuse the same state transitions without
//! duplicating logic.

use crate::tui::app::state::App;
use crate::tui::app::watchdog::WatchdogNotification;
use crate::tui::chat::message::{ChatMessage, MessageType};
use crate::tui::constants::MAIN_PROCESSING_WATCHDOG_TIMEOUT_SECS;

/// Apply watchdog cancel state: increment counter, clear stream, push error.
pub(in crate::tui::app::event_loop) fn apply_watchdog_state(
    app: &mut App,
    notif: WatchdogNotification,
) {
    app.state.main_watchdog_restart_count += 1;
    let count = app.state.main_watchdog_restart_count;
    app.state.watchdog_notification = Some(notif);
    app.state.processing = false;
    app.state.clear_streaming_text();
    app.state.clear_request_timing();
    app.state.status = format!("Watchdog timeout — restarting (attempt {count})");
    app.state.messages.push(ChatMessage::new(
        MessageType::Error,
        format!(
            "⚠ Watchdog: no events for {MAIN_PROCESSING_WATCHDOG_TIMEOUT_SECS}s. \
             Auto-restarting (attempt {count})."
        ),
    ));
    app.state.scroll_to_bottom();
}
