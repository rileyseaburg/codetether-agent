//! JSON Schema advertised by the computer_use tool.

use serde_json::{Value, json};

pub(super) fn parameters_schema() -> Value {
    json!({
        "type": "object",
        "description": "Controls the real OS cursor/keyboard. Use snapshot metadata for physical screen pixel coordinates.",
        "properties": {
            "action": {
                "type": "string",
                "enum": [
                    "status", "list_apps", "request_app",
                    "snapshot", "window_snapshot",
                    "click", "right_click", "double_click", "drag",
                    "type_text", "press_key", "scroll",
                    "bring_to_front", "wait_ms", "stop"
                ],
                "description": "Action to execute. Snapshot returns physical screen pixel bounds and real cursor position. Use bring_to_front before interacting."
            },
            "app": {"type": "string", "description": "App name for request_app."},
            "hwnd": {"type": "integer", "description": "Window handle from list_apps."},
            "text": {"type": "string", "description": "Text for type_text."},
            "key": {"type": "string", "description": "SendKeys for press_key: ^c, %{TAB}, ENTER."},
            "scroll_amount": {"type": "integer", "description": "Wheel delta; negative=down."},
            "x": {"type": "number", "description": "Physical screen pixel X for click/drag start."},
            "y": {"type": "number", "description": "Physical screen pixel Y for click/drag start."},
            "x2": {"type": "number", "description": "Physical screen pixel end X for drag."},
            "y2": {"type": "number", "description": "Physical screen pixel end Y for drag."},
            "ms": {"type": "integer", "description": "Ms to wait for wait_ms."}
        },
        "required": ["action"],
        "examples": [
            {"action": "snapshot"}, {"action": "list_apps"},
            {"action": "bring_to_front", "hwnd": 123456},
            {"action": "click", "x": 100, "y": 200},
            {"action": "right_click", "x": 300, "y": 400},
            {"action": "double_click", "x": 500, "y": 300},
            {"action": "drag", "x": 100, "y": 200, "x2": 400, "y2": 500},
            {"action": "press_key", "key": "^c"},
            {"action": "type_text", "text": "hello world"},
            {"action": "wait_ms", "ms": 500}
        ]
    })
}
