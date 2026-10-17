//! Assemble a multi-file confirmation response without writing.
use super::{EditPreview, ToolResult};
use serde_json::json;
use std::collections::HashMap;
pub(super) fn result(
    previews: &[EditPreview],
    total_added: usize,
    total_removed: usize,
) -> ToolResult {
    let mut all_diffs = String::new();
    for preview in previews {
        all_diffs.push_str(&format!("\n=== {} ===\n{}", preview.file, preview.diff));
    }

    let mut metadata = HashMap::new();
    metadata.insert("requires_confirmation".to_string(), json!(true));
    metadata.insert("total_files".to_string(), json!(previews.len()));
    metadata.insert("total_added".to_string(), json!(total_added));
    metadata.insert("total_removed".to_string(), json!(total_removed));
    metadata.insert("previews".to_string(), json!(previews));

    return ToolResult {
        output: format!(
            "Multi-file changes require confirmation:{}\n\nTotal: {} files, +{} lines, -{} lines",
            all_diffs,
            previews.len(),
            total_added,
            total_removed
        ),
        success: true,
        metadata,
    };
}
