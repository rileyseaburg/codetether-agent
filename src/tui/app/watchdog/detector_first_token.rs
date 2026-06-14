//! First-activity predicates for the watchdog detector.
//!
//! Splitting these tiny predicates out of [`super`] keeps both files within the
//! project's 50-line limit while giving the no-first-token clause a single,
//! testable source of truth for "has anything happened yet?".

use crate::tui::app::state::AppState;
use crate::tui::constants::WATCHDOG_MAX_RESTARTS;

/// Returns `true` once the restart budget for this request is spent.
///
/// After [`WATCHDOG_MAX_RESTARTS`] restarts the provider or tool is treated as
/// permanently broken and the watchdog stops intervening.
pub(super) fn restart_budget_exhausted(state: &AppState) -> bool {
    state.main_watchdog_restart_count >= WATCHDOG_MAX_RESTARTS
}

/// Returns `true` if any activity event has arrived since the request started.
///
/// "Activity" is any of: a streamed token (tracked via
/// `current_request_first_token_ms`), or any other `SessionEvent` that refreshed
/// `main_last_event_at` *after* `processing_started_at` — which includes tool
/// calls and `ToolHeartbeat` ticks. When this is `true`, the no-first-token
/// clause is suppressed and the inactivity clause governs instead.
pub(super) fn had_activity_since_start(state: &AppState) -> bool {
    if state.current_request_first_token_ms.is_some() {
        return true;
    }
    match (state.processing_started_at, state.main_last_event_at) {
        (Some(started), Some(last_event)) => last_event > started,
        _ => false,
    }
}
