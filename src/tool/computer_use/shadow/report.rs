//! Honest queue-only outcomes, capabilities, and logical state reporting.
use super::{replay::Outcome, types::State};
use crate::tool::ToolResult;
use serde_json::{Value, json};

pub(super) fn result(
    hwnd: Option<i64>,
    state: State,
    outcome: Outcome,
    observation: Value,
) -> ToolResult {
    let success = outcome.error.is_none();
    let output = json!({
        "input_mode": "shadow", "hwnd": hwnd,
        "messages_queued": outcome.queued,
        "application_effect_unverified": true,
        "logical_state": state, "error": outcome.error,
        "observation": observation,
        "capabilities": {
            "actions": ["status", "stop", "click", "right_click", "double_click", "mouse_move", "mouse_down", "mouse_up", "drag", "scroll", "type_text", "press_key"],
            "keys": ["Enter", "Tab", "Escape", "Backspace", "Space", "Left", "Right", "Up", "Down", "Home", "End", "PageUp", "PageDown", "Insert", "Delete"],
            "modifiers": false, "physical_fallback": false,
            "max_drag_steps": 240, "max_duration_ms": 30000,
            "max_text_utf16_units": 8192
        },
        "limitations": "PostMessage queue acceptance is not application acknowledgement. UIPI can deny posting; raw-input or focus-dependent apps may ignore messages. Global keyboard state, mouse capture, activation, and non-client hit testing are not synthesized. Target the actual child control HWND when needed. Stop only releases shadow-held messages, not hardware input. Queued messages can trigger app-side focus changes; unchanged focus is never guaranteed. Concurrent user movement/focus changes remain possible.",
        "coordinates": "client_area=false: outer-window-relative; true: client-relative; mouse LPARAM: client; wheel LPARAM: screen; physical pixels in per-monitor DPI context; signed 16-bit packing"
    }).to_string();
    if success {
        ToolResult::success(output)
    } else {
        ToolResult::error(output)
    }
}
pub(super) fn failure(error: anyhow::Error) -> ToolResult {
    ToolResult::error(json!({
        "input_mode": "shadow", "messages_queued": 0,
        "application_effect_unverified": true, "logical_state": null,
        "error": format!("{error:#}"), "physical_fallback": false,
        "limitations": "No posts attempted; existing logical state was not inspected or cleared. UIPI and unsupported applications can deny or ignore posted input."
    }).to_string())
}