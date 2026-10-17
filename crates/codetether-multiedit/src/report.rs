//! Validation failures, write failures and per-edit success summaries.
use super::ToolResult;
use std::{collections::HashMap, path::PathBuf};
pub(super) fn validation(errors: &[String], total_edits: usize) -> ToolResult {
    let error_list = errors.join("\n");
    return ToolResult {
        output: format!(
            "Validation failed for {} of {} edits. No files were modified.\n\n{error_list}",
            errors.len(),
            total_edits
        ),
        success: false,
        metadata: HashMap::new(),
    };
}
pub(super) fn write_errors(write_errors: &[String]) -> ToolResult {
    return ToolResult {
        output: format!(
            "Write errors (some files may have been partially updated):\n{}",
            write_errors.join("\n")
        ),
        success: false,
        metadata: HashMap::new(),
    };
}
pub(super) fn success(
    validated: &[(PathBuf, String, String)],
    unique_files: usize,
    total_edits: usize,
) -> ToolResult {
    let mut summary_lines: Vec<String> = Vec::new();
    for (path, original, new_content) in validated {
        let old_lines = original.lines().count();
        let new_lines = new_content.lines().count();
        let delta = new_lines as i64 - old_lines as i64;
        let sign = if delta >= 0 { "+" } else { "" };
        summary_lines.push(format!("✓ {} ({sign}{delta} lines)", path.display()));
    }

    ToolResult::success(format!(
        "Applied {total_edits} edit(s) across {unique_files} file(s):\n{}",
        summary_lines.join("\n")
    ))
}
