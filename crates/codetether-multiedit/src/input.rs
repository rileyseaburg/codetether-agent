//! Validate the top-level nonempty edits array.
use super::ToolResult;
use serde_json::{Value, json};
pub(super) fn parse(params: &Value) -> Result<&Vec<Value>, ToolResult> {
    let edits_val = match params.get("edits") {
        Some(v) => v,
        None => {
            return Err(ToolResult::structured_error(
                "INVALID_ARGUMENT",
                "multiedit",
                "Missing required field 'edits'. Provide an array of {file, old_string, new_string} objects.",
                Some(vec!["edits"]),
                Some(json!({
                    "edits": [
                        {"file": "src/main.rs", "old_string": "old code", "new_string": "new code"}
                    ]
                })),
            ));
        }
    };

    let edits_arr = match edits_val.as_array() {
        Some(a) => a,
        None => {
            return Err(ToolResult::structured_error(
                "INVALID_ARGUMENT",
                "multiedit",
                "'edits' must be an array, not a single object.",
                Some(vec!["edits"]),
                Some(json!({
                    "edits": [
                        {"file": "src/main.rs", "old_string": "old code", "new_string": "new code"}
                    ]
                })),
            ));
        }
    };

    if edits_arr.is_empty() {
        return Err(ToolResult::error(
            "No edits provided. 'edits' array is empty.",
        ));
    }

    Ok(edits_arr)
}
