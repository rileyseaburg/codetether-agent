//! Preview commit messages and affected paths without modifying the repository.
use anyhow::Result;
use codetether_tool_core::ToolResult;
use std::{collections::HashMap, path::Path, process::Command};
pub(super) fn show(
    cwd: &Path,
    steps: usize,
    commit_count: usize,
    commit_list: &[String],
) -> Result<ToolResult> {
    let mut preview = format!("Would undo {} commit(s):\n\n", commit_count);
    for commit in commit_list {
        preview.push_str(&format!("  {}\n", commit));
    }

    // Show what files would be affected
    let diff_output = Command::new("git")
        .args(["diff", &format!("HEAD~{}", steps), "--name-only"])
        .current_dir(&cwd)
        .output()?;

    if diff_output.status.success() {
        let files = String::from_utf8_lossy(&diff_output.stdout);
        if !files.trim().is_empty() {
            preview.push_str("\nFiles that would be affected:\n");
            for file in files.lines() {
                preview.push_str(&format!("  {}\n", file));
            }
        }
    }

    return Ok(ToolResult {
        output: preview,
        success: true,
        metadata: HashMap::new(),
    });
}
