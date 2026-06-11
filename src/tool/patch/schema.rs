//! JSON schema for the apply_patch tool.

use serde_json::{Value, json};

/// Return the apply_patch parameter schema.
pub(super) fn parameters() -> Value {
    json!({
        "type": "object",
        "properties": {
            "patch": {
                "type": "string",
                "description": "Unified diff patch content"
            },
            "dry_run": {
                "type": "boolean",
                "default": false,
                "description": "Preview without applying"
            },
            "preview": {
                "type": "boolean",
                "default": false,
                "description": "Alias for dry_run"
            },
            "approval_id": {
                "type": "string",
                "description": "External approval token for required patch writes"
            }
        },
        "required": ["patch"]
    })
}
