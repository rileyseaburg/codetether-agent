//! Regression coverage for user-paced approval waits.

use std::time::{Duration, Instant};

use super::shared::{TIMEOUT, processing_state};
use crate::tui::app::watchdog::detector::check_watchdog_stall;

#[test]
fn pending_approval_suppresses_watchdog_restart() {
    let mut state = processing_state();
    state.approval_waiting = true;
    state.processing_started_at = Some(Instant::now() - Duration::from_secs(120));
    state.main_last_event_at = Some(Instant::now() - Duration::from_secs(120));
    assert!(check_watchdog_stall(&state, TIMEOUT).is_none());
}
