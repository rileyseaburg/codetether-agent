//! Parameter schema for [`ContextSummarizeTool`].

use serde_json::{Value, json};

/// JSON Schema for `context_summarize` tool parameters.
pub fn parameters() -> Value {
    json!({
        "type": "object",
        "properties": {
            "start": { "type": "integer", "minimum": 0,
                "description": "Start index (inclusive)." },
            "end": { "type": "integer", "minimum": 1,
                "description": "End index (exclusive)." },
            "target_tokens": { "type": "integer", "minimum": 64,
                "description": "Target token budget (default 512)." }
        },
        "required": ["start", "end"]
    })
}
