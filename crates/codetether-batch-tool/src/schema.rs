//! Batch tool argument schema.
use serde_json::{Value, json};
pub(super) fn parameters() -> Value {
    json!({
        "type": "object",
        "properties": {
            "calls": {
                "type": "array",
                "description": "Small array of tool calls to execute. Keep arguments short and use at most three calls. Preferred keys are `tool` + `args`; aliases `name` + `arguments` are also accepted for compatibility.",
                "items": {
                    "type": "object",
                    "properties": {
                        "tool": {"type": "string", "description": "Tool ID to call (alias: `name`)"},
                        "args": {"type": "object", "description": "Arguments for the tool (alias: `arguments`)"},
                        "name": {"type": "string", "description": "Alias for `tool`"},
                        "arguments": {"type": "object", "description": "Alias for `args`"}
                    },
                    "anyOf": [
                        { "required": ["tool", "args"] },
                        { "required": ["name", "arguments"] }
                    ]
                }
            }
        },
        "required": ["calls"]
    })
}
