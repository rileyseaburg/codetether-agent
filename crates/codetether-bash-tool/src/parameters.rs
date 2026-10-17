//! Schema of accepted bash tool arguments.
use serde_json::{Value, json};
pub(super) fn parameters() -> Value {
    json!({
        "type": "object",
        "properties": {
            "command": {
                "type": "string",
                "description": "The shell command to execute"
            },
            "cwd": {
                "type": "string",
                "description": "Working directory for the command (optional)"
            },
            "timeout": {
                "type": "integer",
                "description": "Timeout in seconds (default: 120)"
            },
            "justification": {
                "type": "string",
                "description": "Why this command is needed; required before an approval prompt in access mode `ask`"
            },
            "prefix_rule": {
                "type": "array",
                "items": {"type": "string"},
                "description": "Reusable command prefix proposed for session approval"
            }
        },
        "required": ["command"],
        "example": {
            "command": "ls -la src/",
            "cwd": "/path/to/project"
        }
    })
}
