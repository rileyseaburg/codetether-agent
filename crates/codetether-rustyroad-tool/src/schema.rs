//! Structured RustyRoad invocation schema; no arbitrary command execution.

use serde_json::{Value, json};

pub(super) fn parameters() -> Value {
    json!({
        "type": "object",
        "properties": {
            "action": {"type": "string", "enum": ["list_tools", "call_tool"]},
            "cwd": {"type": "string", "minLength": 1,
                "description": "RustyRoad project directory; required, not inherited."},
            "environment": {"type": "string", "enum": ["dev", "test", "prod"],
                "default": "dev", "description": "Selects rustyroad.toml or rustyroad.<environment>.toml."},
            "tool_name": {"type": "string",
                "description": "Advertised rustyroad_* tool; required for call_tool."},
            "arguments": {"type": "object",
                "description": "Selected tool's arguments. env must match environment if supplied."},
            "approval_id": {"type": "string"},
            "justification": {"type": "string",
                "description": "Why this operation is needed; required in ask mode."}
        },
        "required": ["action", "cwd"],
        "additionalProperties": false
    })
}
