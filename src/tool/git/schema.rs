//! JSON Schema for the git tool's parameters.

use serde_json::{Value, json};

/// Parameter schema describing the `git` tool's accepted arguments.
pub(super) fn parameters() -> Value {
    json!({
        "type": "object",
        "properties": {
            "op": {
                "type": "string",
                "enum": ["status", "diff", "diff_staged", "log", "branch", "show", "commit"],
                "description": "The git operation to perform"
            },
            "path": { "type": "string", "description": "Limit diff to a path (for op=diff)" },
            "message": { "type": "string", "description": "Commit message (required for op=commit)" },
            "paths": {
                "type": "array",
                "items": { "type": "string" },
                "description": "Files to stage for commit; defaults to all changes"
            },
            "cwd": { "type": "string", "description": "Repository directory (default: current)" }
        },
        "required": ["op"]
    })
}
