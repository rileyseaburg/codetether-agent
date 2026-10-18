//! Read-only detector inputs borrowed from a frontend request.
use crate::{WatchdogSource, WatchdogState};
use codetether_tool_tracker::ToolCallTracker;
use std::time::Instant;

/// Current request inputs for stall detection without an application state type.
/// Question-runtime lookup remains lazy inside the detector's early gate.
///
/// # Examples
/// ```
/// let watchdog = codetether_watchdog::WatchdogState::default();
/// let calls = codetether_tool_tracker::ToolCallTracker::default();
/// let view = codetether_watchdog::DetectorView {
///     watchdog: &watchdog, tool_calls: &calls, processing: false,
///     approval_waiting: false, session_id: None,
///     request_started_at: None, first_token_ms: None,
/// };
/// assert!(codetether_watchdog::check_watchdog_stall(
///     &view, std::time::Duration::from_secs(60),
/// ).is_none());
/// ```
pub struct DetectorView<'a> {
    /// Existing activity clock, notification and restart budget.
    pub watchdog: &'a WatchdogState,
    /// Open bus tool calls and their declared runtime budgets.
    pub tool_calls: &'a ToolCallTracker,
    /// Whether the frontend is processing a request.
    pub processing: bool,
    /// Whether the request is waiting for human approval.
    pub approval_waiting: bool,
    /// Session identity used only when checking unanswered questions.
    pub session_id: Option<&'a str>,
    /// Start clock of the current request.
    pub request_started_at: Option<Instant>,
    /// First-token latency of the current request, if observed.
    pub first_token_ms: Option<u64>,
}

impl WatchdogSource for DetectorView<'_> {
    fn skip_stall_check(&self) -> bool {
        crate::interaction::skip_stall_check(
            self.processing,
            self.watchdog.notification.is_some(),
            self.approval_waiting,
            self.session_id,
        )
    }
    fn restart_count(&self) -> u32 {
        self.watchdog.restart_count
    }
    fn request_started_at(&self) -> Option<Instant> {
        self.request_started_at
    }
    fn first_token_ms(&self) -> Option<u64> {
        self.first_token_ms
    }
    fn last_event_at(&self) -> Option<Instant> {
        self.watchdog.last_event_at
    }
    fn tool_calls(&self) -> &ToolCallTracker {
        self.tool_calls
    }
}
