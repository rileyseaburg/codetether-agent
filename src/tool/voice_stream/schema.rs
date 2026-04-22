//! JSON Schema for the voice_stream tool parameters.

use serde_json::{Value, json};

/// Return the JSON Schema accepted by `voice_stream`.
pub(crate) fn json_schema() -> Value {
    json!({
        "type": "object",
        "properties": {
            "action": {
                "type": "string",
                "enum": ["speak_stream", "play"],
                "description": "Action to perform"
            },
            "text": {
                "type": "string",
                "description": "Text to speak (required for speak_stream)"
            },
            "voice_id": {
                "type": "string",
                "description": "Voice profile ID (defaults to Riley)"
            },
            "language": {
                "type": "string",
                "description": "Language (default: english)"
            },
            "job_id": {
                "type": "string",
                "description": "Job ID to replay (required for play)"
            }
        },
        "required": ["action"]
    })
}
