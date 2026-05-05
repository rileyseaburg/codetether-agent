use serde_json::{Value, json};

/// JSON Schema for the `tetherscript_plugin` tool parameters.
///
/// Includes optional browser capability fields that map to
/// `tetherscript::browser_cap::BrowserAuthority`.
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
            },
            "grant_browser": {
                "type": "string",
                "description": "Browser bridge endpoint to grant browser capability (e.g. http://127.0.0.1:41707/browser)"
            },
            "browser_origin": {
                "type": "array",
                "items": { "type": "string" },
                "description": "Allowed origins for browser capability (e.g. [\"http://localhost:5173\"])"
            },
            "browser_scope": {
                "type": "array",
                "items": { "type": "string" },
                "description": "Allowed scopes for browser capability (e.g. [\"browser.navigate\", \"browser.interact\"]). Omit for default scopes."
            }
        },
        "required": ["hook"]
    })
}
