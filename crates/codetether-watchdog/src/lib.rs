//! Stall detection and notification presentation for interactive requests.
//!
//! Implement WatchdogSource for the host, call check_watchdog_stall, and show
//! any returned WatchdogNotification with render_watchdog_notification.
mod activity;
mod detector;
mod labels;
mod notification;
mod source;
mod state;
pub use detector::check_watchdog_stall;
pub use notification::render_watchdog_notification;
pub use source::WatchdogSource;
pub use state::WatchdogNotification;
/// Maximum automatic restarts before the detector stops intervening.
pub const WATCHDOG_MAX_RESTARTS: u32 = 3;

/// Cancellation state, exact-prompt retries and permanent-failure reporting.
pub mod recovery;

/// User-paced question waits and stall suppression.
pub mod interaction;
