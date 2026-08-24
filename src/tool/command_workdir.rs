//! Workspace-bound working-directory validation for command tools.

use super::ToolResult;
use serde_json::Value;
use std::path::{Path, PathBuf};

pub(crate) fn result(tool: &str, args: &Value) -> Option<ToolResult> {
    let root = crate::tool::network_access::trusted_workspace(args).map(PathBuf::from)?;
    let target = match tool {
        "bash" => path(args, "cwd"),
        "exec_command" => path(args, "workdir").or_else(|| path(args, "cwd")),
        _ => None,
    }?;
    match within(&root, &target) {
        Ok(true) => None,
        Ok(false) => Some(denied(tool, &root, &target, "outside the session workspace")),
        Err(error) => Some(denied(tool, &root, &target, &error.to_string())),
    }
}

fn path(args: &Value, key: &str) -> Option<PathBuf> {
    args.get(key).and_then(Value::as_str).map(PathBuf::from)
}

fn within(root: &Path, target: &Path) -> anyhow::Result<bool> {
    let root = root.canonicalize()?;
    let target = if target.is_absolute() {
        target.canonicalize()?
    } else {
        root.join(target).canonicalize()?
    };
    Ok(target.starts_with(root))
}

fn denied(tool: &str, root: &Path, target: &Path, reason: &str) -> ToolResult {
    ToolResult::structured_error(
        "COMMAND_WORKDIR_DENIED",
        tool,
        &format!("command working directory {} is denied: {reason}", target.display()),
        Some(vec!["cwd", "workdir"]),
        Some(serde_json::json!({"workspace": root, "requested": target})),
    )
}

#[cfg(test)]
#[path = "command_workdir_tests.rs"]
mod tests;