//! Metadata attachment for patch results.

use crate::tool::ToolResult;
use serde_json::{Value, json};

/// Attach consistent review metadata to a patch result.
pub(super) fn attach(
    mut result: ToolResult,
    files: &[String],
    hunks: usize,
    preview: &str,
    approval_required: bool,
) -> ToolResult {
    for (key, value) in entries(files, hunks, preview, approval_required) {
        result.metadata.insert(key, value);
    }
    result
}

fn entries(
    files: &[String],
    hunks: usize,
    preview: &str,
    approval_required: bool,
) -> [(String, Value); 4] {
    [
        ("patch_files".into(), json!(files)),
        ("patch_hunks".into(), json!(hunks)),
        ("patch_preview".into(), json!(preview)),
        ("approval_required".into(), json!(approval_required)),
    ]
}
