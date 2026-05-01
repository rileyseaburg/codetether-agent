//! JSON Schema advertised by the computer_use tool.

use serde_json::{Value, json};

pub(super) fn parameters_schema() -> Value {
    json!({
        "type": "object",
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
                "description": "Action to execute. Snapshot saves JPEG to temp. Use bring_to_front before interacting."
            },
            "app": {"type": "string", "description": "App name for request_app."},
            "hwnd": {"type": "integer", "description": "Window handle from list_apps."},
            "text": {"type": "string", "description": "Text for type_text."},
            "key": {"type": "string", "description": "SendKeys for press_key: ^c, %{TAB}, ENTER."},
            "scroll_amount": {"type": "integer", "description": "Wheel delta; negative=down."},
            "x": {"type": "number", "description": "X coord for click/drag start."},
            "y": {"type": "number", "description": "Y coord for click/drag start."},
            "x2": {"type": "number", "description": "End X for drag."},
            "y2": {"type": "number", "description": "End Y for drag."},
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
