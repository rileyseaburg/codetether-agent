//! JSON schema for Ralph run, status and PRD creation actions.

use serde_json::{Value, json};
pub(crate) fn parameters() -> Value {
    json!({
        "type": "object",
        "properties": {
            "action": {
                "type": "string",
                "enum": ["run", "status", "create-prd"],
                "description": "Action to perform"
            },
            "prd_path": {
                "type": "string",
                "description": "Path to prd.json file (default: prd.json)"
            },
            "feature": {
                "type": "string",
                "description": "Feature name for create-prd action"
            },
            "project": {
                "type": "string",
                "description": "Project name for create-prd action"
            },
            "max_iterations": {
                "type": "integer",
                "description": "Maximum iterations for run action (default: 10)"
            }
        },
        "required": ["action"]
    })
}
