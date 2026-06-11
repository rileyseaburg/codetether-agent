//! Runtime policy guard for destructive undo operations.

use anyhow::Result;
use serde_json::Value;
use std::path::Path;
use std::process::Command;

pub(super) fn default_steps() -> usize {
    1
}

pub(super) async fn blocked(args: &Value) -> Option<crate::tool::ToolResult> {
    crate::runtime_policy::evaluate_tool_invocation("undo", args).await
}

pub(super) fn reset(cwd: &Path, steps: usize) -> Result<std::process::Output> {
    Ok(Command::new("git")
        .args(["reset", "--hard", &format!("HEAD~{steps}")])
        .current_dir(cwd)
        .output()?)
}
