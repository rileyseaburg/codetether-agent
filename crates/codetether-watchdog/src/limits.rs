//! Inactivity budget for the outer interactive-session watchdog.
//!
//! Session keepalive events refresh the activity clock during healthy long
//! model calls. This budget applies when the runtime emits no activity at all.
/// Seconds without a session event before the main request is restarted.
///
/// This outer timeout exceeds the session's 180-second cold-stall budget.
pub const MAIN_PROCESSING_WATCHDOG_TIMEOUT_SECS: u64 = 240;
