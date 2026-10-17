//! Require exact snippets unless the Morph path is available.
use super::{ToolResult, op::EditOp};
use serde_json::json;
pub(super) fn validate(op: &EditOp, i: usize) -> Result<(), ToolResult> {
    let file = &op.file;
    if !op.use_morph && (op.old_string.is_none() || op.new_string.is_none()) {
        return Err(ToolResult::structured_error(
            "INVALID_ARGUMENT",
            "multiedit",
            &format!(
                "edits[{i}] ({file}): provide old_string/new_string, or enable Morph backend and provide instruction/update"
            ),
            Some(vec!["edits"]),
            Some(json!({
                "file": op.file,
                "old_string": "exact text to find",
                "new_string": "replacement text",
                "instruction": "Optional Morph instruction",
                "update": "Optional Morph update snippet"
            })),
        ));
    }

    Ok(())
}
