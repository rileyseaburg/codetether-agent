//! schema for web content fetching.
use super::*;

pub(super) fn parameters() -> Value {
    json!({
        "type": "object",
        "properties": {
            "url": {"type": "string"},
            "format": {"type": "string", "enum": ["markdown", "text", "html"], "default": "markdown"},
            "max_chars": {
                "type": "integer",
                "minimum": 1000,
                "default": DEFAULT_MAX_CHARS,
                "description": "Maximum number of characters to return (safety limit to avoid overflowing the model context window)."
            }
        },
        "required": ["url"]
    })
}
