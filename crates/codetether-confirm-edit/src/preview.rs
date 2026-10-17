//! Tool result requesting confirmation without writing the file.
use super::{ToolResult, diff::Preview, input::Input};
use serde_json::json;
use std::collections::HashMap;
pub(super) fn result(input: &Input, preview: &Preview) -> ToolResult {
    let mut metadata = HashMap::new();
    metadata.insert("requires_confirmation".to_string(), json!(true));
    metadata.insert("diff".to_string(), json!(preview.diff_output.trim()));
    metadata.insert("added_lines".to_string(), json!(preview.added));
    metadata.insert("removed_lines".to_string(), json!(preview.removed));
    metadata.insert("path".to_string(), json!(input.path));
    metadata.insert("old_string".to_string(), json!(input.old_string));
    metadata.insert("new_string".to_string(), json!(input.new_string));

    return ToolResult {
        output: format!(
            "Changes require confirmation:\n\n{}",
            preview.diff_output.trim()
        ),
        success: true,
        metadata,
    };
}
