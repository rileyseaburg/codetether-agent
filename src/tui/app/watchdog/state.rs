//! Watchdog notification state and types.

use std::time::Instant;

/// Notification shown when the watchdog detects a stalled request.
#[derive(Debug, Clone)]
pub struct WatchdogNotification {
    pub message: String,
    pub detected_at: Instant,
    pub restart_count: u32,
}

impl WatchdogNotification {
    pub fn new(message: String, restart_count: u32) -> Self {
        Self {
            message,
            detected_at: Instant::now(),
            restart_count,
        }
    }
}
