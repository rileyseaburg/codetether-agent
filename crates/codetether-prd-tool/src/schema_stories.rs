//! schema stories for PRD generation and validation.
use super::*;
pub(super) fn value() -> Value {
    json!({
        "type": "array",
        "description": "User stories (for generate/validate/save)",
        "items": {
            "type": "object",
            "properties": {
                "id": {"type": "string"},
                "title": {"type": "string"},
                "description": {"type": "string"},
                "acceptance_criteria": {"type": "array", "items": {"type": "string"}},
                "verification_steps": {"type": "array", "items": {"type": "object"}},
                "priority": {"type": "integer"},
                "depends_on": {"type": "array", "items": {"type": "string"}},
                "complexity": {"type": "integer"}
            },
            "required": ["id", "title", "description"]
        }
    })
}
