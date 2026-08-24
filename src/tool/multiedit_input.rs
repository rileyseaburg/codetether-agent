//! Top-level input validation for multi-file edits.

use super::ToolResult;
use serde_json::{Value, json};

pub(super) fn edits(params: &Value) -> Result<&Vec<Value>, ToolResult> {
    let value = params.get("edits").ok_or_else(|| {
        ToolResult::structured_error(
            "INVALID_ARGUMENT",
            "multiedit",
            "Missing required field 'edits'. Provide an array of edit objects.",
            Some(vec!["edits"]),
            Some(example()),
        )
    })?;
    value.as_array().ok_or_else(|| {
        ToolResult::structured_error(
            "INVALID_ARGUMENT",
            "multiedit",
            "'edits' must be an array, not a single object.",
            Some(vec!["edits"]),
            Some(example()),
        )
    })
}

fn example() -> Value {
    json!({
        "edits": [
            {"file": "src/main.rs", "old_string": "old code", "new_string": "new code"}
        ]
    })
}
