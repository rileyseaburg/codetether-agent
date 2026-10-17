//! schema for workspace code search.
use super::*;

pub(super) fn parameters() -> Value {
    json!({
        "type": "object",
        "properties": {
            "pattern": {"type": "string", "description": "Search pattern (regex supported)"},
            "path": {"type": "string", "description": "Subdirectory to search in"},
            "file_pattern": {"type": "string", "description": "Glob pattern for files (e.g., *.rs)"},
            "context_lines": {"type": "integer", "default": 2, "description": "Lines of context"},
            "case_sensitive": {"type": "boolean", "default": false}
        },
        "required": ["pattern"]
    })
}
