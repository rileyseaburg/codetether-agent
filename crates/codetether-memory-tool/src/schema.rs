//! Memory action argument schema.
use serde_json::{Value, json};
pub(super) fn parameters() -> Value {
    json!({
        "type": "object",
        "properties": {
            "action": {
                "type": "string",
                "description": "Action to perform: 'save' (store new memory), 'search' (find memories), 'get' (retrieve specific memory), 'list' (show recent), 'tags' (show categories), 'delete' (remove), 'stats' (show statistics)",
                "enum": ["save", "search", "get", "list", "tags", "delete", "stats"]
            },
            "content": {
                "type": "string",
                "description": "Memory content to save (required for 'save' action)"
            },
            "tags": {
                "type": "array",
                "items": {"type": "string"},
                "description": "Tags for categorization (optional for 'save')"
            },
            "query": {
                "type": "string",
                "description": "Search query (for 'search' action)"
            },
            "scope": {
                "type": "string",
                "description": "Project/context scope. Defaults to a stable git scope, not the transient worktree path. Use 'all' for unscoped search/list."
            },
            "importance": {
                "type": "integer",
                "description": "Importance level 1-5 (optional for 'save', default 3)"
            },
            "id": {
                "type": "string",
                "description": "Memory ID (required for 'get' and 'delete')"
            },
            "limit": {
                "type": "integer",
                "description": "Maximum results to return (default 10, for 'search' and 'list')"
            }
        },
        "required": ["action"]
    })
}
