//! Mutable monitoring and recovery state for the current request.
use crate::WatchdogNotification;
use std::time::Instant;

/// Activity and retry state retained between watchdog checks.
///
/// Detection and recovery policies decide when to change these values. The
/// restart count is kept separately from notification dismissal so dismissing
/// a notice does not implicitly reset the recovery budget.
///
/// # Examples
/// ```
/// let state = codetether_watchdog::WatchdogState::default();
/// assert!(state.notification.is_none());
/// assert!(state.last_event_at.is_none());
/// assert!(state.inflight_prompt.is_none());
/// assert_eq!(state.restart_count, 0);
/// ```
#[derive(Default)]
pub struct WatchdogState {
    /// Pending stall notification, if one is displayed.
    pub notification: Option<WatchdogNotification>,
    /// Last activity observed for the current request.
    pub last_event_at: Option<Instant>,
    /// Number of recovery restarts recorded for the current recovery budget.
    pub restart_count: u32,
    /// Prompt retained for retry or stream reconnection.
    pub inflight_prompt: Option<String>,
}
