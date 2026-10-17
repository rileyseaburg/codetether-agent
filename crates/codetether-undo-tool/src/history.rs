//! Inspect repository membership and the recent commit list.
use anyhow::Result;
use codetether_tool_core::ToolResult;
use std::{collections::HashMap, path::Path, process::Command};
pub(super) fn read(cwd: &Path, steps: usize) -> Result<Result<(usize, Vec<String>), ToolResult>> {
    // Check if we're in a git repository
    let status = Command::new("git")
        .args(["rev-parse", "--git-dir"])
        .current_dir(&cwd)
        .status()?;

    if !status.success() {
        return Ok(Err(ToolResult {
            output: "Error: Not in a git repository".to_string(),
            success: false,
            metadata: HashMap::new(),
        }));
    }

    // Get the last N commits
    let log_output = Command::new("git")
        .args(["log", "--oneline", "--max-count", &steps.to_string()])
        .current_dir(&cwd)
        .output()?;

    if !log_output.status.success() {
        return Ok(Err(ToolResult {
            output: "Error: Failed to get git log".to_string(),
            success: false,
            metadata: HashMap::new(),
        }));
    }

    let commits = String::from_utf8_lossy(&log_output.stdout);
    if commits.trim().is_empty() {
        return Ok(Err(ToolResult {
            output: "No commits found to undo".to_string(),
            success: false,
            metadata: HashMap::new(),
        }));
    }

    let commit_count = commits.lines().count();
    let commit_list: Vec<String> = commits.lines().map(|s| s.to_string()).collect();

    Ok(Ok((commit_count, commit_list)))
}
