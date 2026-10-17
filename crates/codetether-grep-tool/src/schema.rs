//! schema for bounded grep searches.
use super::*;

pub(super) fn parameters() -> Value {
    json!({
        "type": "object",
        "properties": {
            "pattern": {
                "type": "string",
                "description": "The text or regex pattern to search for"
            },
            "path": {
                "type": "string",
                "description": "Directory or file to search in (default: current directory)"
            },
            "is_regex": {
                "type": "boolean",
                "description": "Whether the pattern is a regex (default: false)"
            },
            "include": {
                "type": "string",
                "description": "Glob pattern to include files (e.g., *.rs)"
            },
            "limit": {
                "type": "integer",
                "description": "Maximum number of matches to return"
            },
            "timeout_secs": {
                "type": "integer",
                "description": "Maximum search time in seconds before returning partial results"
            }
        },
        "required": ["pattern"],
        "example": {
            "pattern": "fn main",
            "path": "src/",
            "include": "*.rs"
        }
    })
}
