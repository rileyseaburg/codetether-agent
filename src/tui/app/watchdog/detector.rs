//! Watchdog stall detection logic.
//!
//! Two distinct timeouts protect long-running sessions from silent stalls:
//!
//! * **Inactivity timeout** — the agent produced activity (tokens, tool calls,
//!   or tool heartbeats) but has since gone silent for `WATCHDOG_TIMEOUT`. This
//!   catches dead provider connections and hung tool calls where no further
//!   `SessionEvent` arrives.
//! * **No-first-token timeout** — the request started but *nothing* has arrived
//!   at all within `WATCHDOG_TIMEOUT`. This catches providers that never
//!   respond (bad auth, unreachable endpoint, etc.).
//!
//! Both use the same wall-clock budget to keep configuration simple. Crucially,
//! the no-first-token clause is only evaluated *before* the first activity
//! event; once any event arrives (including a [`SessionEvent::ToolHeartbeat`])
//! the inactivity clause takes over so long-but-active turns are not killed.
//!
//! [`SessionEvent::ToolHeartbeat`]: crate::session::SessionEvent::ToolHeartbeat

use std::time::Duration;

use crate::tui::app::state::AppState;
use crate::tui::app::watchdog::state::WatchdogNotification;

#[path = "detector_first_token.rs"]
mod detector_first_token;

use detector_first_token::{had_activity_since_start, restart_budget_exhausted};

/// Check if the current request is stalled and return a notification if so.
///
/// Returns `None` when the request is healthy, already being restarted, or has
/// exhausted its retry budget.
pub fn check_watchdog_stall(state: &AppState, timeout: Duration) -> Option<WatchdogNotification> {
    if !state.processing || state.watchdog_notification.is_some() {
        return None;
    }
    if restart_budget_exhausted(state) {
        return None;
    }
    let now = std::time::Instant::now();
    // Inactivity: last event (token, tool call, or heartbeat) was too long ago.
    let inactive = state
        .main_last_event_at
        .map(|t| now.duration_since(t) >= timeout)
        .unwrap_or(false);

    // No-first-token: request started, nothing has arrived yet. Only meaningful
    // before the first activity event — once anything arrives, the inactivity
    // clause governs so active turns are never killed by the start clock.
    let no_first_token = !had_activity_since_start(state)
        && state
            .processing_started_at
            .map(|t| now.duration_since(t) >= timeout)
            .unwrap_or(false);
    if !inactive && !no_first_token {
        return None;
    }
    let secs = timeout.as_secs();
    let label = if no_first_token {
        format!("⚠ No response after {secs}s. Provider may be unreachable.")
    } else {
        format!("⚠ No events for {secs}s. Stalled request detected.")
    };
    Some(WatchdogNotification::new(
        label,
        state.main_watchdog_restart_count,
    ))
}