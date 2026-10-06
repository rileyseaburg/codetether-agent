//! Task-only schema; goal control belongs to the dedicated goal tools.

use serde_json::{Value, json};

pub(super) fn value() -> Value {
    json!({
        "type": "object",
        "properties": {
            "action": {"type": "string", "enum": [
                "task_add", "task_status", "list"
            ]},
            "id": {"type": "string"},
            "content": {"type": "string"},
            "parent_id": {"type": "string"},
            "status": {"type": "string",
                "enum": ["pending", "in_progress", "done", "blocked", "cancelled"]},
            "note": {"type": "string"}
        },
        "required": ["action"]
    })
}

#[cfg(test)]
#[path = "separation_tests.rs"]
mod tests;
