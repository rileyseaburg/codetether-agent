//! schema for PRD construction and validation.
use super::*;

pub(super) fn parameters() -> Value {
    // Keep `verification_steps` permissive; the enum is validated by serde when PRD JSON is parsed.
    json!({
        "type": "object",
        "properties": {
            "action": {
                "type": "string",
                "enum": ["analyze", "generate", "validate", "save"],
                "description": "Action to perform"
            },
            "task_description": {
                "type": "string",
                "description": "Description of the complex task (for analyze)"
            },
            "project": {
                "type": "string",
                "description": "Project name (for generate/save)"
            },
            "feature": {
                "type": "string",
                "description": "Feature name (for generate/save)"
            },
            "stories": schema_stories::value(),
            "quality_checks": {
                "type": "object",
                "properties": {
                    "typecheck": {"type": "string"},
                    "test": {"type": "string"},
                    "lint": {"type": "string"},
                    "build": {"type": "string"}
                }
            },
            "prd_path": {
                "type": "string",
                "description": "Path to PRD file (validate from file, or save destination)"
            },
            "prd_json": {
                "type": "string",
                "description": "Raw PRD JSON string to validate (alternative to prd_path)"
            }
        },
        "required": ["action"]
    })
}
