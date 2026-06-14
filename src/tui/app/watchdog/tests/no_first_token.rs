//! Tests for the no-first-token clause and its suppression by activity.

use std::time::{Duration, Instant};

use super::shared::{TIMEOUT, processing_state};
use crate::tui::app::watchdog::detector::check_watchdog_stall;

#[test]
fn no_first_token_fires_when_nothing_arrived() {
    let mut state = processing_state();
    state.processing_started_at = Some(Instant::now() - Duration::from_secs(120));
    // No activity event ever landed: main_last_event_at stays None.
    state.main_last_event_at = None;
    let notif = check_watchdog_stall(&state, TIMEOUT).expect("should stall");
    assert!(notif.message.contains("Provider may be unreachable"));
}

#[test]
fn first_token_suppresses_no_first_token_clause() {
    let mut state = processing_state();
    // Request started long ago, but a token arrived.
    state.processing_started_at = Some(Instant::now() - Duration::from_secs(120));
    state.current_request_first_token_ms = Some(50);
    state.main_last_event_at = Some(Instant::now());
    assert!(
        check_watchdog_stall(&state, TIMEOUT).is_none(),
        "active turn must not be killed by the start clock"
    );
}

#[test]
fn tool_activity_suppresses_no_first_token_clause() {
    let mut state = processing_state();
    state.processing_started_at = Some(Instant::now() - Duration::from_secs(120));
    // No streamed token, but a tool call / heartbeat refreshed activity after start.
    state.current_request_first_token_ms = None;
    state.main_last_event_at = Some(Instant::now());
    assert!(
        check_watchdog_stall(&state, TIMEOUT).is_none(),
        "tool heartbeat activity must keep the turn alive"
    );
}
