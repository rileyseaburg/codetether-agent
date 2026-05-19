//! Tool parameter schema for session_recall.

use serde_json::{Value, json};

/// JSON Schema for `session_recall` tool parameters.
pub fn parameters() -> Value {
    json!({
        "type": "object",
        "properties": {
            "query": {
                "type": "string",
                "description": "Natural-language question about past session content"
            },
            "session_id": {
                "type": "string",
                "description": "Specific session UUID to recall from"
            },
            "limit": {
                "type": "integer",
                "description": "Recent sessions to include (default 3, max 5)",
                "default": 3
            }
        },
        "required": ["query"]
    })
}
