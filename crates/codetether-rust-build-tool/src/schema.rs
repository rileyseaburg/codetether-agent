//! Typed discoverable input, intentionally excluding grants and shell snippets.
use serde_json::{Value, json};

pub(super) fn parameters() -> Value {
    json!({
        "type": "object",
        "properties": {
            "action": {"enum": ["build", "run", "test", "check", "cache-build", "cache-publish", "status", "doctor", "stop"]},
            "cwd": {"type": "string", "description": "Absolute Cargo workspace directory."},
            "toolchain": {"type": "string", "default": "stable"},
            "package": {"type": "string"},
            "bin": {"type": "string"},
            "args": {"type": "array", "items": {"type": "string"}, "description": "Literal run/test arguments after --."}
        },
        "required": ["action", "cwd"],
        "additionalProperties": false
    })
}
