//! schema for shared swarm results.
use super::*;

pub(super) fn parameters() -> Value {
    json!({
        "type": "object",
        "properties": {
            "action": {
                "type": "string",
                "enum": ["publish", "get", "query_tags", "query_prefix", "list"],
                "description": "Action to perform"
            },
            "key": {
                "type": "string",
                "description": "Result key (for publish/get)"
            },
            "value": {
                "description": "Result value to publish (any JSON value)"
            },
            "tags": {
                "type": "array",
                "items": {"type": "string"},
                "description": "Tags for publish or query_tags"
            },
            "prefix": {
                "type": "string",
                "description": "Key prefix for query_prefix"
            },
            "producer": {
                "type": "string",
                "description": "Producer subtask ID to filter by (for query)"
            }
        },
        "required": ["action"]
    })
}
