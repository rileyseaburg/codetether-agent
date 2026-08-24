//! Reconstruct proposed file contents without mutating the workspace.

#[path = "path.rs"]
mod path;

use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use serde_json::Value;

pub(super) fn collect(
    workspace: &Path,
    tool: &str,
    args: &Value,
) -> Result<Vec<(PathBuf, String)>> {
    match tool {
        "write" => write(workspace, args).map(|file| vec![file]),
        "apply_patch" | "patch" => patch(workspace, args),
        _ => Ok(Vec::new()),
    }
}

fn write(workspace: &Path, args: &Value) -> Result<(PathBuf, String)> {
    let raw_path = required_string(args, "path")?;
    let content = args
        .get("content")
        .and_then(Value::as_str)
        .context("content is required for pre-approval validation")?;
    let path = PathBuf::from(raw_path);
    let path = if path.is_absolute() {
        path
    } else {
        workspace.join(path)
    };
    Ok((path::confined(workspace, path)?, content.to_string()))
}

fn patch(workspace: &Path, args: &Value) -> Result<Vec<(PathBuf, String)>> {
    let patch = required_string(args, "patch")?;
    crate::tool::patch::proposed::contents(workspace, patch)
        .context("failed to reconstruct proposed patch contents")
}

fn required_string<'a>(args: &'a Value, field: &str) -> Result<&'a str> {
    let value = args.get(field).and_then(Value::as_str).unwrap_or("");
    if value.is_empty() {
        bail!("{field} is required for pre-approval validation");
    }
    Ok(value)
}
