//! Native watchdog state projection and detector binding.
use super::state::WatchdogNotification;
use crate::tui::app::state::AppState;
use std::time::{Duration, Instant};
impl codetether_watchdog::WatchdogSource for AppState {
    fn skip_stall_check(&self) -> bool {
        super::question_wait::skip_stall_check(self)
    }
    fn restart_count(&self) -> u32 {
        self.main_watchdog_restart_count
    }
    fn request_started_at(&self) -> Option<Instant> {
        self.request_timing.processing_started_at
    }
    fn first_token_ms(&self) -> Option<u64> {
        self.request_timing.current_request_first_token_ms
    }
    fn last_event_at(&self) -> Option<Instant> {
        self.main_last_event_at
    }
    fn tool_calls(&self) -> &codetether_tool_tracker::ToolCallTracker {
        &self.tool_calls
    }
}
/// Check host state for a stalled request.
pub fn check_watchdog_stall(state: &AppState, timeout: Duration) -> Option<WatchdogNotification> {
    codetether_watchdog::check_watchdog_stall(state, timeout)
}
