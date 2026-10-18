//! Minimal request fields used by detector regression scenarios.
use crate::{DetectorView, WatchdogNotification, WatchdogState};
use codetether_tool_tracker::ToolCallTracker;
use std::time::{Duration, Instant};

pub(super) const TIMEOUT: Duration = Duration::from_secs(60);

#[derive(Default)]
pub(super) struct Timing {
    pub processing_started_at: Option<Instant>,
    pub current_request_first_token_ms: Option<u64>,
}

#[derive(Default)]
pub(super) struct State {
    pub processing: bool,
    pub approval_waiting: bool,
    pub watchdog: WatchdogState,
    pub request_timing: Timing,
    pub tool_calls: ToolCallTracker,
}

pub(super) fn processing_state() -> State {
    State {
        processing: true,
        ..State::default()
    }
}

pub(super) fn check_watchdog_stall(
    state: &State,
    timeout: Duration,
) -> Option<WatchdogNotification> {
    crate::check_watchdog_stall(
        &DetectorView {
            watchdog: &state.watchdog,
            tool_calls: &state.tool_calls,
            processing: state.processing,
            approval_waiting: state.approval_waiting,
            session_id: None,
            request_started_at: state.request_timing.processing_started_at,
            first_token_ms: state.request_timing.current_request_first_token_ms,
        },
        timeout,
    )
}
