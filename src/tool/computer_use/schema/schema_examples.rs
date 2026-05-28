//! JSON Schema examples for computer_use.

use serde_json::{Value, json};

pub fn examples() -> Value {
    json!([
        {"action": "snapshot"}, {"action": "list_apps"},
        {"action": "bring_to_front", "hwnd": 123456},
        {"action": "click", "hwnd": 123456, "x": 80, "y": 30},
        {"action": "right_click", "x": 300, "y": 400},
        {"action": "double_click", "x": 500, "y": 300},
        {"action": "drag", "x": 100, "y": 200, "x2": 400, "y2": 500},
        {"action": "drag", "hwnd": 123456, "button": "middle", "client_area": true, "x": 960, "y": 540, "x2": 1240, "y2": 540, "steps": 12, "duration_ms": 500},
        {"action": "drag", "hwnd": 123456, "button": "middle", "modifiers": ["shift"], "client_area": true, "x": 960, "y": 540, "x2": 960, "y2": 720, "steps": 12},
        {"action": "mouse_down", "hwnd": 123456, "button": "middle", "x": 960, "y": 540},
        {"action": "mouse_move", "hwnd": 123456, "x": 1040, "y": 620},
        {"action": "mouse_up", "button": "middle"},
        {"action": "scroll", "hwnd": 123456, "x": 1780, "y": 260, "scroll_amount": -600},
        {"action": "press_key", "key": "^c"},
        {"action": "type_text", "text": "hello world"},
        {"action": "wait_ms", "ms": 500}
    ])
}
