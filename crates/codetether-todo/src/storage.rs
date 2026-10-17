//! Per-session task-list storage location.
//!
//! Task lists belong to the calling session. Keying on the checkout root would
//! merge every mux session that shares a checkout (`--no-worktree`) into one
//! list, so the session id injected by the runtime selects the file and the
//! checkout-root file is used only when no session identity is present.

use std::path::{Path, PathBuf};

use anyhow::Result;
use serde_json::Value;

/// Checkout-local filename used when no safe session-specific path is available.
pub const ROOT_FILE: &str = ".codetether-todos.json";

/// Resolve the task-list file for this tool call.
///
/// # Arguments
/// Supply the checkout root and call parameters.
/// # Returns
/// The selected session or fallback path.
/// # Examples
/// ```rust
/// use codetether_todo::storage::resolve;
/// let path = resolve(std::path::Path::new("."), &serde_json::json!({}));
/// assert!(!path.as_os_str().is_empty());
/// ```
pub fn resolve(root: &Path, params: &Value) -> PathBuf {
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
    match session.and_then(|id| codetether_config::Config::data_dir().map(|dir| (dir, id))) {
        Some((dir, id)) => dir.join("todos").join(format!("{id}.json")),
        None => root.join(ROOT_FILE),
    }
}

/// Load the calling session's task list, or an empty list when none exists.
///
/// # Arguments
/// Supply the checkout root and call parameters.
/// # Returns
/// The decoded items, or an empty list when the file is absent.
/// # Errors
/// Returns filesystem or JSON errors.
/// # Examples
/// ```rust,no_run
/// use codetether_todo::storage::load;
/// let _items = load(std::path::Path::new("."), &serde_json::json!({}))?;
/// # Ok::<(), anyhow::Error>(())
/// ```
pub fn load(root: &Path, params: &Value) -> Result<Vec<crate::TodoItem>> {
    let path = resolve(root, params);
    if !path.exists() {
        return Ok(Vec::new());
    }
    Ok(serde_json::from_str(&std::fs::read_to_string(&path)?)?)
}

/// Persist the calling session's task list, creating its directory on demand.
///
/// # Arguments
/// Supply the checkout root and call parameters, plus the full replacement item list.
/// # Returns
/// Unit after the complete item list is written.
/// # Errors
/// Returns filesystem or JSON errors.
/// # Examples
/// ```rust,no_run
/// use codetether_todo::storage::save;
/// save(std::path::Path::new("."), &serde_json::json!({}), &[])?;
/// # Ok::<(), anyhow::Error>(())
/// ```
pub fn save(root: &Path, params: &Value, items: &[crate::TodoItem]) -> Result<()> {
    let path = resolve(root, params);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    Ok(std::fs::write(&path, serde_json::to_string_pretty(items)?)?)
}
