//! Required summary input for the context-reset tool.
use serde_json::{Value, json};
pub(super) fn parameters() -> Value {
    json!({
        "type": "object",
        "properties": {
            "summary": {
                "type": "string",
                "description": "Compressed task state in your own words: \
                                goal, key decisions, open questions, \
                                must-remember details. This replaces \
                                the earlier transcript as the anchor \
                                for subsequent turns."
            }
        },
        "required": ["summary"]
    })
}
