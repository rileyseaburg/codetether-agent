//! Native request inputs for the crate-owned watchdog detector view.
use super::state::WatchdogNotification;
use crate::tui::app::state::AppState;
use std::time::Duration;
/// Check host state for a stalled request.
pub fn check_watchdog_stall(state: &AppState, timeout: Duration) -> Option<WatchdogNotification> {
    codetether_watchdog::check_watchdog_stall(
        &codetether_watchdog::DetectorView {
            watchdog: &state.watchdog,
            tool_calls: &state.tool_calls,
            processing: state.processing,
            approval_waiting: state.approval_waiting,
            session_id: state.session_id.as_deref(),
            request_started_at: state.request_timing.processing_started_at,
            first_token_ms: state.request_timing.current_request_first_token_ms,
        },
        timeout,
    )
}
