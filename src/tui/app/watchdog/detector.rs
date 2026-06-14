//! Watchdog stall detection logic.
//!
//! Two distinct timeouts protect long-running sessions from silent stalls:
//!
//! * **Inactivity timeout** — the agent was streaming but has gone silent for
//!   `WATCHDOG_TIMEOUT`. This catches dead provider connections and hung tool
//!   calls where no further `SessionEvent` arrives.
//! * **No-first-token timeout** — the request started but nothing has arrived
//!   at all within `WATCHDOG_TIMEOUT`. This catches providers that never
//!   respond (bad auth, unreachable endpoint, etc.).
//!
//! Both use the same wall-clock budget to keep configuration simple.

use std::time::Duration;

use crate::tui::app::state::AppState;
use crate::tui::app::watchdog::state::WatchdogNotification;
use crate::tui::constants::WATCHDOG_MAX_RESTARTS;

/// Check if the current request is stalled and return a notification if so.
///
/// Returns `None` when the request is healthy, already being restarted, or has
/// exhausted its retry budget.
pub fn check_watchdog_stall(state: &AppState, timeout: Duration) -> Option<WatchdogNotification> {
    if !state.processing || state.watchdog_notification.is_some() {
        return None;
    }
    // After too many restarts, stop — the provider or tool is permanently broken.
    if state.main_watchdog_restart_count >= WATCHDOG_MAX_RESTARTS {
        return None;
    }
    let now = std::time::Instant::now();
    // Inactivity: last event was too long ago.
    let inactive = state
        .main_last_event_at
        .map(|t| now.duration_since(t) >= timeout)
        .unwrap_or(false);
    // No-first-token: request started but nothing arrived yet.
    let no_first_token = state
        .processing_started_at
        .map(|t| now.duration_since(t) >= timeout)
        .unwrap_or(false);
    if !inactive && !no_first_token {
        return None;
    }
    let secs = timeout.as_secs();
    let label = if inactive {
        format!("⚠ No events for {secs}s. Stalled request detected.")
    } else {
        format!("⚠ No response after {secs}s. Provider may be unreachable.")
    };
    Some(WatchdogNotification::new(label, state.main_watchdog_restart_count))
}
