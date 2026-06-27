//! Regression tests for watchdog re-arming after notification clear.

use std::time::{Duration, Instant};

use super::shared::{TIMEOUT, processing_state};
use crate::tui::app::watchdog::detector::check_watchdog_stall;

/// Regression: a pending notification suppresses the detector, but once the
/// retry path clears it (as `watchdog_retry::execute` now does on resubmit)
/// the detector must re-arm. Without re-arming the UI stayed `processing=true`
/// forever and silently queued every user prompt.
#[test]
fn cleared_notification_rearms_watchdog() {
    let mut state = processing_state();
    state.processing_started_at = Some(Instant::now() - Duration::from_secs(120));
    state.main_last_event_at = Some(Instant::now() - Duration::from_secs(120));
    // A pending notification suppresses further stall detection.
    state.watchdog_notification = Some(crate::tui::app::watchdog::WatchdogNotification::new(
        "stalled".into(),
        1,
    ));
    assert!(check_watchdog_stall(&state, TIMEOUT).is_none());
    // Retry resubmits and clears the notification → detector re-arms.
    state.watchdog_notification = None;
    assert!(check_watchdog_stall(&state, TIMEOUT).is_some());
}
