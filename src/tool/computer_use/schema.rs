//! JSON Schema advertised by the computer_use tool.

use serde_json::{Value, json};

pub(super) fn parameters_schema() -> Value {
    json!({
        "type": "object",
        "properties": {
            "action": {
                "type": "string",
                "enum": ["status", "list_apps", "request_app", "snapshot", "click", "type_text", "press_key", "scroll", "stop"],
                "description": "Computer use action to execute. Currently Windows-only."
            },
            "app": {
                "type": "string",
                "description": "Target app name for request_app only; app-scoped control is not enforced yet."
            },
            "window_title_contains": {
                "type": "string",
                "description": "Reserved for future app/window targeting; currently rejected for control actions."
            },
            "text": {
                "type": "string",
                "description": "Text to type for type_text action. Also accepted by press_key for compatibility."
            },
            "key": {
                "type": "string",
                "description": "SendKeys expression for press_key, e.g. ENTER, ^c, %{TAB}."
            },
            "scroll_amount": {
                "type": "integer",
                "description": "Mouse-wheel delta for scroll; negative scrolls down."
            },
            "x": {
                "type": "number",
                "description": "Finite X coordinate for click actions."
            },
            "y": {
                "type": "number",
                "description": "Finite Y coordinate for click actions."
            }
        },
        "required": ["action"],
        "examples": [
            {"action": "status"},
            {"action": "list_apps"},
            {"action": "snapshot"},
            {"action": "click", "x": 100, "y": 200},
            {"action": "press_key", "key": "^c"},
            {"action": "scroll", "scroll_amount": -120}
        ]
    })
}
