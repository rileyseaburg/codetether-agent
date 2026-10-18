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
//! event; once any event arrives (including a tool heartbeat)
//! the inactivity clause takes over so long-but-active turns are not killed.
//!
//! Session heartbeat events count as activity just like streamed tokens.

use std::time::Duration;

use crate::WatchdogNotification;
use crate::WatchdogSource;

use crate::activity::{had_activity_since_start, restart_budget_exhausted};

/// Check if the current request is stalled and return a notification if so.
///
/// Returns `None` when the request is healthy, already being restarted, or has
/// exhausted its retry budget.
///
/// # Arguments
/// * `state` — Host state queried lazily through WatchdogSource.
/// * `timeout` — Inactivity and first-response budget.
///
/// # Returns
/// A notification for an eligible stall, otherwise None.
///
/// # Examples
/// ```
/// # fn inspect(state: &impl codetether_watchdog::WatchdogSource) {
/// let _ = codetether_watchdog::check_watchdog_stall(state, std::time::Duration::from_secs(60));
/// # }
/// ```
pub fn check_watchdog_stall(
    state: &impl WatchdogSource,
    timeout: Duration,
) -> Option<WatchdogNotification> {
    if state.skip_stall_check() {
        return None;
    }
    if restart_budget_exhausted(state) {
        return None;
    }
    let now = std::time::Instant::now();
    // Inactivity: last event (token, tool call, or heartbeat) was too long ago.
    let inactive = state
        .last_event_at()
        .map(|t| now.duration_since(t) >= timeout)
        .unwrap_or(false);

    // No-first-token: request started, nothing has arrived yet. Only meaningful
    // before the first activity event — once anything arrives, the inactivity
    // clause governs so active turns are never killed by the start clock.
    let request_started_at = state.request_started_at();
    let no_first_token = !had_activity_since_start(state)
        && request_started_at
            .map(|t| now.duration_since(t) >= timeout)
            .unwrap_or(false);
    if !inactive && !no_first_token {
        return None;
    }
    // A tool that is still inside the runtime it asked for is working, not
    // stalled, even if it has been silent longer than the watchdog budget.
    if state.tool_calls().within_declared_budget() {
        return None;
    }
    let secs = timeout.as_secs();
    // Prefer a precise per-agent tool-call alert when the bus shows an open
    // (un-answered) tool call that has exceeded the timeout.
    if let Some(call) = state.tool_calls().stalled(timeout) {
        let label = crate::labels::tool_stall(call, secs);
        return Some(WatchdogNotification::new(label, state.restart_count()));
    }
    let label = crate::labels::request_stall(no_first_token, secs);
    Some(WatchdogNotification::new(label, state.restart_count()))
}
