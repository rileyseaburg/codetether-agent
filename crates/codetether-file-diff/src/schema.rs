//! schema for file comparisons.
use super::*;
pub(super) fn parameters() -> Value {
    json!({
        "type": "object",
        "properties": {
            "file1": {
                "type": "string",
                "description": "First file to compare (or file for git diff)"
            },
            "file2": {
                "type": "string",
                "description": "Second file to compare"
            },
            "git": {
                "type": "boolean",
                "description": "Show git diff for uncommitted changes (default: false)",
                "default": false
            },
            "staged": {
                "type": "boolean",
                "description": "Show git diff for staged changes (default: false)",
                "default": false
            },
            "context": {
                "type": "integer",
                "description": "Lines of context around changes (default: 3)",
                "default": 3
            }
        },
        "example": {
            "git": true,
            "file1": "src/main.rs"
        }
    })
}
