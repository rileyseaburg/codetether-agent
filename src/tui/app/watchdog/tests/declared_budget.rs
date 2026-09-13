//! A tool still inside its declared runtime is never a stall.

use std::time::{Duration, Instant};

use super::shared::{TIMEOUT, processing_state};
use crate::bus::BusMessage;
use crate::tui::app::watchdog::check_watchdog_stall;

fn request(tool: &str, timeout_secs: u64) -> BusMessage {
    BusMessage::ToolRequest {
        request_id: "r1".into(),
        agent_id: "build".into(),
        tool_name: tool.into(),
        arguments: serde_json::json!({ "command": "sleep", "timeout": timeout_secs }),
        step: 1,
    }
}

/// Silence since `ago` with the request started, no first token yet.
fn silent_for(ago: Duration) -> crate::tui::app::state::AppState {
    let mut state = processing_state();
    let then = Instant::now() - ago;
    state.processing_started_at = Some(then);
    state.main_last_event_at = Some(then);
    state
}

#[test]
fn long_declared_bash_is_not_a_stall_before_its_budget() {
    let mut state = silent_for(TIMEOUT * 2);
    state
        .tool_calls
        .observe(&request("bash", TIMEOUT.as_secs() * 10));
    assert!(check_watchdog_stall(&state, TIMEOUT).is_none());
}

#[test]
fn a_tool_that_outlives_its_own_budget_still_trips() {
    let mut state = silent_for(TIMEOUT * 2);
    state.tool_calls.observe(&request("bash", 1));
    std::thread::sleep(Duration::from_millis(1100));
    // Budget spent (1s < 1.1s): the gate no longer shields the call, so the
    // generic inactivity stall fires. The per-tool label needs the call to
    // also exceed the watchdog timeout, which is exercised in tool_calls_tests.
    assert!(check_watchdog_stall(&state, TIMEOUT).is_some());
}

#[test]
fn undeclared_tool_uses_the_watchdog_budget_alone() {
    let mut state = silent_for(TIMEOUT * 2);
    state.tool_calls.observe(&BusMessage::ToolRequest {
        request_id: "r2".into(),
        agent_id: "build".into(),
        tool_name: "read".into(),
        arguments: serde_json::Value::Null,
        step: 1,
    });
    assert!(check_watchdog_stall(&state, TIMEOUT).is_some());
}
