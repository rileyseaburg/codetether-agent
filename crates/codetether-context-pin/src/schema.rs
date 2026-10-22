//! Pin action and turn-index parameter schema.
use serde_json::{Value, json};
pub(super) fn parameters() -> Value {
    json!({
        "type": "object",
        "properties": {
            "action": { "type": "string", "enum": ["pin", "unpin", "supersede", "correct"],
                "description": "Pin a source, remove its pin, or replace it with a later user correction." },
            "turn_index": { "type": "integer", "minimum": 0,
                "description": "Absolute 0-based index into the session transcript." },
            "replacement_index": { "type": "integer", "minimum": 0,
                "description": "For supersede or correct: absolute index of the later user source." },
            "source_text": { "type": "string", "description": "For correct: exact, nonempty, unique text span being replaced. Other text remains active." },
            "source_hash": { "type": "string", "description": "For correct: full original source SHA-256 from the pinned source header, preventing stale edits." }
        },
        "required": ["action", "turn_index"]
    })
}
