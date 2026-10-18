//! Watchdog notification state and types.

use std::time::Instant;

/// Notification shown when the watchdog detects a stalled request.
///
/// # Examples
/// ```
/// let notification = codetether_watchdog::WatchdogNotification::new("stalled".into(), 1);
/// assert_eq!(notification.restart_count, 1);
/// ```
#[derive(Debug, Clone)]
pub struct WatchdogNotification {
    /// Message shown to the user.
    pub message: String,
    /// Time the notification was created.
    pub detected_at: Instant,
    /// Restart count associated with the detected stall.
    pub restart_count: u32,
}

impl WatchdogNotification {
    /// Capture a stall message and the current time.
    ///
    /// # Arguments
    /// * `message` — User-facing stall detail.
    /// * `restart_count` — Restart count when detected.
    ///
    /// # Returns
    /// A new notification timestamped immediately.
    ///
    /// # Examples
    /// ```
    /// let n = codetether_watchdog::WatchdogNotification::new("stalled".into(), 0);
    /// assert_eq!(n.message, "stalled");
    /// ```
    pub fn new(message: String, restart_count: u32) -> Self {
        Self {
            message,
            detected_at: Instant::now(),
            restart_count,
        }
    }
}
