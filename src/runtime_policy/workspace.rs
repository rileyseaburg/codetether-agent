//! Workspace extraction for policy evaluation.

use serde_json::Value;
use std::path::PathBuf;

pub(super) fn from_args(args: &Value) -> Option<PathBuf> {
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
