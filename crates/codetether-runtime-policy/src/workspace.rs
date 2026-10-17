//! Workspace extraction for policy evaluation.

use serde_json::Value;
use std::path::PathBuf;

/// Select the workspace used to load policy configuration.
///
/// # Arguments
/// * `args` — Invocation arguments containing an injected workspace, cwd or path.
/// # Returns
/// The first applicable path, with injected workspace taking precedence over cwd.
/// # Examples
/// ```
/// let args = serde_json::json!({"cwd": "/workspace"});
/// assert_eq!(codetether_runtime_policy::workspace::from_args(&args), Some(std::path::PathBuf::from("/workspace")));
/// ```
pub fn from_args(args: &Value) -> Option<PathBuf> {
    string_path(args, "__ct_parent_workspace")
        .or_else(|| string_path(args, "cwd"))
        .or_else(|| path_parent(args))
}

fn string_path(args: &Value, key: &str) -> Option<PathBuf> {
    args.get(key)
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(PathBuf::from)
}

fn path_parent(args: &Value) -> Option<PathBuf> {
    let path = string_path(args, "path")?;
    if path.is_dir() {
        return Some(path);
    }
    path.parent().map(std::path::Path::to_path_buf)
}
