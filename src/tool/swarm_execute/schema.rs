use serde_json::{Value, json};

pub(super) fn parameters() -> Value {
    json!({
        "type": "object",
        "properties": {
            "tasks": {
                "type": "array",
                "minItems": 1,
                "description": "Independent tasks. Prefer plain instruction strings; use objects only for optional metadata.",
                "items": {
                    "oneOf": [
                        {"type": "string", "description": "Instruction for one sub-agent"},
                        {
                            "type": "object",
                            "properties": {
                                "instruction": {"type": "string"},
                                "name": {"type": "string", "description": "Optional display name"},
                                "id": {"type": "string", "description": "Optional stable identifier"},
                                "specialty": {"type": "string", "description": "Optional agent role"}
                            },
                            "required": ["instruction"]
                        }
                    ]
                }
            },
            "concurrency_limit": {
                "type": "integer", "minimum": 1, "maximum": 20,
                "description": "Optional maximum parallel agents (default: 5)"
            },
            "aggregation_strategy": {
                "type": "string", "enum": ["all", "first_error", "best_effort"],
                "description": "Optional failure policy (default: best_effort)"
            },
            "model": {"type": "string", "description": "Optional provider/model override"},
            "max_steps": {"type": "integer", "description": "Optional steps per agent (default: 50)"},
            "timeout_secs": {"type": "integer", "description": "Optional timeout per agent (default: 300)"}
        },
        "required": ["tasks"],
        "examples": [{"tasks": ["Inspect the API", "Run focused tests"]}]
    })
}
