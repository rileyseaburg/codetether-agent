//! Watchdog stalled-request recovery module.

pub mod detector;
pub mod handler;
pub mod notification;
pub mod state;

pub use detector::check_watchdog_stall;
pub use handler::{handle_watchdog_cancel, handle_watchdog_dismiss};
pub use notification::render_watchdog_notification;
pub use state::WatchdogNotification;
