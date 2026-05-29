//! Blender select-frame result metadata helpers.

use serde_json::{Value, json};

use crate::tool::computer_use::input::ComputerUseInput;

pub fn build(
    input: &ComputerUseInput,
    hwnd: i64,
    target: (i32, i32),
    scripted: Value,
    ui_recovery: Value,
) -> Value {
    json!({
        "sequence": super::blender_sequence::SELECT_FRAME,
        "scripted_selection_state": scripted,
        "ui_recovery_state": ui_recovery,
        "focused_hwnd": hwnd,
        "viewport_child_hwnd": input.viewport_child_hwnd,
        "client_area": input.client_area,
        "target": {"x": target.0, "y": target.1},
    })
}

pub fn available(state: &Value) -> bool {
    state["available"].as_bool().unwrap_or(false)
}

pub fn final_state<'a>(scripted: &'a Value, recovery: &'a Value) -> &'a Value {
    if recovery.is_null() {
        scripted
    } else {
        recovery
    }
}
