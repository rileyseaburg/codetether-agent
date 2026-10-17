//! Format successful or failed reset results.
use codetether_tool_core::ToolResult;
use std::{collections::HashMap, process::Output};
pub(super) fn format(
    revert_output: Output,
    commit_count: usize,
    commit_list: &[String],
) -> ToolResult {
    if revert_output.status.success() {
        let mut result = format!("Successfully undid {} commit(s):\n\n", commit_count);
        for commit in commit_list {
            result.push_str(&format!("  {}\n", commit));
        }

        ToolResult {
            output: result,
            success: true,
            metadata: HashMap::new(),
        }
    } else {
        let error = String::from_utf8_lossy(&revert_output.stderr);
        ToolResult {
            output: format!("Failed to undo commits: {}", error),
            success: false,
            metadata: HashMap::new(),
        }
    }
}
