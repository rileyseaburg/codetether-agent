use serde_json::{Value, json};

pub fn parameters() -> Value {
    json!({
        "type": "object",
        "properties": {
            "path": {
                "type": "string",
                "description": "Path to a TetherScript .tether plugin file; legacy .kl files are accepted during migration"
            },
            "source": {
                "type": "string",
                "description": "Inline TetherScript plugin source; used instead of path when provided"
            },
            "hook": {
                "type": "string",
                "description": "Top-level TetherScript function to call"
            },
            "args": {
                "type": "array",
                "description": "JSON arguments converted to TetherScript values",
                "items": {}
            },
            "timeout_secs": {
                "type": "integer",
                "description": "Maximum wall-clock seconds to wait for the hook; capped at 60"
            }
        },
        "required": ["hook"]
    })
}
