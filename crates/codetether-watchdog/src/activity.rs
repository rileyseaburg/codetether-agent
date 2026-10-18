//! First-activity predicates for the watchdog detector.
//!
//! Splitting these tiny predicates out of [`super`] keeps both files within the
//! project's 50-line limit while giving the no-first-token clause a single,
//! testable source of truth for "has anything happened yet?".

use crate::WATCHDOG_MAX_RESTARTS;
use crate::WatchdogSource;

/// Returns `true` once the restart budget for this request is spent.
///
/// After [`WATCHDOG_MAX_RESTARTS`] restarts the provider or tool is treated as
/// permanently broken and the watchdog stops intervening.
pub(super) fn restart_budget_exhausted(state: &impl WatchdogSource) -> bool {
    state.restart_count() >= WATCHDOG_MAX_RESTARTS
}

/// Returns `true` if any activity event has arrived since the request started.
///
/// "Activity" is any of: a streamed token (tracked via
/// `current_request_first_token_ms`), or any other `SessionEvent` that refreshed
/// `main_last_event_at` *after* `processing_started_at` — which includes tool
/// calls and `ToolHeartbeat` ticks. When this is `true`, the no-first-token
/// clause is suppressed and the inactivity clause governs instead.
pub(super) fn had_activity_since_start(state: &impl WatchdogSource) -> bool {
    if state.first_token_ms().is_some() {
        return true;
    }
    match (state.request_started_at(), state.last_event_at()) {
        (Some(started), Some(last_event)) => last_event > started,
        _ => false,
    }
}
