//! schema for web search.
use super::*;

pub(super) fn parameters() -> Value {
    json!({
        "type": "object",
        "properties": {
            "query": {"type": "string", "description": "Search query"},
            "max_results": {"type": "integer", "default": 5, "description": "Max results to return"}
        },
        "required": ["query"]
    })
}
