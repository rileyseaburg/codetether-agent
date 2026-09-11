//! Per-session task-list storage location.
//!
//! Task lists belong to the calling session. Keying on the checkout root would
//! merge every mux session that shares a checkout (`--no-worktree`) into one
//! list, so the session id injected by the runtime selects the file and the
//! checkout-root file is used only when no session identity is present.

use std::path::{Path, PathBuf};

use anyhow::Result;
use serde_json::Value;

const ROOT_FILE: &str = ".codetether-todos.json";

/// Resolve the task-list file for this tool call.
pub(super) fn resolve(root: &Path, params: &Value) -> PathBuf {
    let session = params
        .get("__ct_session_id")
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|id| !id.is_empty())
        .map(String::from)
        .or_else(|| std::env::var("CODETETHER_SESSION_ID").ok())
        .filter(|id| {
            id.chars()
                .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_'))
        });
    match session.and_then(|id| crate::config::Config::data_dir().map(|dir| (dir, id))) {
        Some((dir, id)) => dir.join("todos").join(format!("{id}.json")),
        None => root.join(ROOT_FILE),
    }
}

/// Load the calling session's task list, or an empty list when none exists.
pub(super) fn load(root: &Path, params: &Value) -> Result<Vec<super::TodoItem>> {
    let path = resolve(root, params);
    if !path.exists() {
        return Ok(Vec::new());
    }
    Ok(serde_json::from_str(&std::fs::read_to_string(&path)?)?)
}

/// Persist the calling session's task list, creating its directory on demand.
pub(super) fn save(root: &Path, params: &Value, items: &[super::TodoItem]) -> Result<()> {
    let path = resolve(root, params);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    Ok(std::fs::write(&path, serde_json::to_string_pretty(items)?)?)
}

#[cfg(test)]
#[path = "todo_path_tests.rs"]
mod tests;
