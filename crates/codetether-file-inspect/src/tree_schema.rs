//! tree schema for filesystem inspection.
use super::*;
pub(super) fn parameters() -> Value {
    json!({
        "type": "object",
        "properties": {
            "path": {
                "type": "string",
                "description": "The root directory to display"
            },
            "depth": {
                "type": "integer",
                "description": "Maximum depth to traverse (default: 3)",
                "default": 3
            },
            "show_hidden": {
                "type": "boolean",
                "description": "Show hidden files (default: false)",
                "default": false
            },
            "show_size": {
                "type": "boolean",
                "description": "Show file sizes (default: false)",
                "default": false
            },
            "gitignore": {
                "type": "boolean",
                "description": "Respect .gitignore rules (default: true)",
                "default": true
            }
        },
        "required": ["path"],
        "example": {
            "path": "src/",
            "depth": 2,
            "show_size": true
        }
    })
}
