//! JSON schema advertised for command-session input and polling.

use serde_json::{Value, json};

pub(super) fn schema() -> Value {
    json!({
        "type": "object",
        "properties": {
            "session_id": {"type": "integer", "description": "Identifier returned by exec_command."},
            "chars": {"type": "string", "description": "Bytes to write; empty polls without writing."},
            "yield_time_ms": {"type": "integer", "description": "Wait before yielding recent output."},
            "max_output_tokens": {"type": "integer", "description": "Output token budget; defaults to 10000 and is policy-capped."}
        },
        "required": ["session_id"],
        "additionalProperties": false
    })
}
