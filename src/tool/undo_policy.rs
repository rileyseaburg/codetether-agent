//! Runtime policy guard for destructive undo operations.

#[cfg(test)]
#[path = "undo_policy_tests.rs"]
mod tests;
use anyhow::Result;
use serde_json::Value;
use std::path::Path;

pub(super) fn default_steps() -> usize {
    1
}

pub(super) async fn blocked(args: &Value, cwd: &Path) -> Option<crate::tool::ToolResult> {
    if let Err(error) = crate::tool::git::process::preflight(cwd, true) {
        return Some(crate::tool::ToolResult::error(error.to_string()));
    }
    if let Some(blocked) = crate::runtime_policy::evaluate_tool_invocation("undo", args).await {
        return Some(blocked);
    }
    crate::approval::use_once::claim("undo", args)
        .err()
        .map(|error| crate::tool::ToolResult::error(format!("approval claim failed: {error}")))
}

pub(super) fn reset(cwd: &Path, steps: usize) -> Result<std::process::Output> {
    let revision = format!("HEAD~{steps}");
    crate::tool::git::process::output_blocking_refs(cwd, &["reset", "--hard", &revision], true)
}
