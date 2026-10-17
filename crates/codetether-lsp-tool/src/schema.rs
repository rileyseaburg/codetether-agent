//! LSP tool argument schema.
use serde_json::{Value, json};
pub(super) fn parameters() -> Value {
    json!({
        "type": "object",
        "properties": {
            "action": super::schema_action::action(),
            "file_path": {
                "type": "string",
                "description": "The absolute or relative path to the file"
            },
            "path": {
                "type": "string",
                "description": "Alias for file_path"
            },
            "line": {
                "type": "integer",
                "description": "The line number (1-based, as shown in editors)",
                "minimum": 1
            },
            "column": {
                "type": "integer",
                "description": "The character offset/column (1-based, as shown in editors)",
                "minimum": 1
            },
            "query": {
                "type": "string",
                "description": "Search query for workspaceSymbol action"
            },
            "include_declaration": {
                "type": "boolean",
                "description": "For findReferences: include the declaration in results",
                "default": true
            }
        },
        "required": ["action"]
    })
}
