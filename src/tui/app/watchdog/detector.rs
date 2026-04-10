//! Watchdog stall detection logic.

use std::time::Duration;

use crate::tui::app::state::AppState;
use crate::tui::app::watchdog::state::WatchdogNotification;

/// Check if the current request is stalled and return a notification if so.
pub fn check_watchdog_stall(state: &AppState, timeout: Duration) -> Option<WatchdogNotification> {
    if !state.processing {
        return None;
    }
    let timed_out = state
        .main_last_event_at
        .map(|t| t.elapsed() >= timeout)
        .unwrap_or(true);
    if !timed_out {
        return None;
    }
    let count = state.main_watchdog_restart_count;
    Some(WatchdogNotification::new(
        format!(
            "⚠ No events for {}s. Stalled request detected.",
            timeout.as_secs()
        ),
        count,
    ))
}
