//! Tests for the inactivity clause, healthy paths, and budget guards.

use std::time::{Duration, Instant};

use super::shared::{TIMEOUT, processing_state};
use crate::tui::app::state::AppState;
use crate::tui::app::watchdog::detector::check_watchdog_stall;
use crate::tui::constants::WATCHDOG_MAX_RESTARTS;

#[test]
fn healthy_request_does_not_stall() {
    let mut state = processing_state();
    state.processing_started_at = Some(Instant::now());
    state.main_last_event_at = Some(Instant::now());
    assert!(check_watchdog_stall(&state, TIMEOUT).is_none());
}

#[test]
fn not_processing_never_stalls() {
    let mut state = AppState::default();
    state.processing = false;
    state.processing_started_at = Some(Instant::now() - Duration::from_secs(120));
    assert!(check_watchdog_stall(&state, TIMEOUT).is_none());
}

#[test]
fn inactivity_after_activity_still_stalls() {
    let mut state = processing_state();
    state.processing_started_at = Some(Instant::now() - Duration::from_secs(200));
    state.current_request_first_token_ms = Some(50);
    // Last activity was long ago: heartbeats stopped.
    state.main_last_event_at = Some(Instant::now() - Duration::from_secs(120));
    let notif = check_watchdog_stall(&state, TIMEOUT).expect("should stall on inactivity");
    assert!(notif.message.contains("Stalled request detected"));
}

#[test]
fn exhausted_restart_budget_disables_watchdog() {
    let mut state = processing_state();
    state.processing_started_at = Some(Instant::now() - Duration::from_secs(120));
    state.main_last_event_at = None;
    state.main_watchdog_restart_count = WATCHDOG_MAX_RESTARTS;
    assert!(check_watchdog_stall(&state, TIMEOUT).is_none());
}

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
