//! Voice tool argument schema.
use serde_json::{Value, json};
pub(super) fn parameters() -> Value {
    json!({
        "type": "object",
        "properties": {
            "action": {
                "type": "string",
                "enum": ["speak", "transcribe", "list_voices", "health"],
                "description": "Action to perform"
            },
            "text": {
                "type": "string",
                "description": "Text to speak (required for 'speak' action)"
            },
            "voice_id": {
                "type": "string",
                "description": "Voice profile ID (optional for 'speak', defaults to Riley voice)"
            },
            "language": {
                "type": "string",
                "description": "Language for speech (default: english)",
                "default": "english"
            },
            "file_path": {
                "type": "string",
                "description": "Path to audio/video file (required for 'transcribe' action)"
            }
        },
        "required": ["action"]
    })
}
