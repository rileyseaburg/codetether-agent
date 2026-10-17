//! Skill tool argument schema.
use serde_json::{Value, json};
pub(super) fn parameters() -> Value {
    json!({
        "type": "object",
        "properties": {
            "action": {
                "type": "string",
                "description": "Action to perform: 'list' (show available skills) or 'load' (load a skill)",
                "enum": ["list", "load"]
            },
            "skill_name": {
                "type": "string",
                "description": "Name of the skill to load (required for 'load' action)"
            }
        },
        "required": ["action"]
    })
}
