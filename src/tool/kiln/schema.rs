use serde_json::{Value, json};

pub fn parameters() -> Value {
    json!({
        "type": "object",
        "properties": {
            "path": {
                "type": "string",
                "description": "Path to a Kiln .kl plugin file"
            },
            "source": {
                "type": "string",
                "description": "Inline Kiln plugin source; used instead of path when provided"
            },
            "hook": {
                "type": "string",
                "description": "Top-level Kiln function to call"
            },
            "args": {
                "type": "array",
                "description": "JSON arguments converted to Kiln values",
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
