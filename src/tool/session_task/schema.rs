//! JSON schema for the legacy session task surface.

use serde_json::{Value, json};

pub(super) fn value() -> Value {
    json!({
        "type": "object",
        "properties": {
            "action": {"type": "string", "enum": [
                "set_goal", "reaffirm", "clear_goal",
                "task_add", "task_status", "list"
            ]},
            "objective": {"type": "string"},
            "success_criteria": {"type": "array", "items": {"type": "string"}},
            "forbidden": {"type": "array", "items": {"type": "string"}},
            "progress_note": {"type": "string"},
            "reason": {"type": "string"},
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
