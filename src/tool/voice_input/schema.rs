//! JSON Schema for the voice_input tool parameters.

use serde_json::{Value, json};

/// Return the JSON Schema accepted by `voice_input`.
pub(crate) fn json_schema() -> Value {
    json!({
        "type": "object",
        "properties": {
            "action": {
                "type": "string",
                "enum": ["record_then_transcribe"],
                "description": "Action to perform"
            },
            "max_duration_secs": {
                "type": "integer",
                "description": "Max recording duration in seconds (default 60, max 300)",
                "minimum": 1,
                "maximum": 300
            }
        },
        "required": ["action"]
    })
}
