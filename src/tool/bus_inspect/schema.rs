//! JSON Schema for the `bus_inspect` tool parameters.

use serde_json::{Value, json};

/// Parameter schema: optional `limit` and `topic_prefix`.
pub fn parameters() -> Value {
    json!({
        "type": "object",
        "properties": {
            "limit": {
                "type": "integer",
                "description": "Max envelopes to return (newest last). Default 20, max 200.",
                "minimum": 1,
                "maximum": 200
            },
            "topic_prefix": {
                "type": "string",
                "description": "Only return envelopes whose topic starts with this prefix \
                    (e.g. 'swarm.', 'task.', 'agent.')."
            }
        }
    })
}
