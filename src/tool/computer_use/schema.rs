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
                "description": "Target app name for actions requiring app focus."
            },
            "window_title_contains": {
                "type": "string",
                "description": "Filter windows by title containing this text."
            },
            "text": {
                "type": "string",
                "description": "Text to type for type_text action."
            },
            "x": {
                "type": "number",
                "description": "X coordinate for click actions."
            },
            "y": {
                "type": "number",
                "description": "Y coordinate for click actions."
            }
        },
        "required": ["action"],
        "examples": [
            {"action": "status"},
            {"action": "list_apps"},
            {"action": "snapshot", "app": "Chrome"},
            {"action": "click", "app": "Calculator", "x": 100, "y": 200},
            {"action": "type_text", "app": "Notepad", "text": "Hello World"}
        ]
    })
}
