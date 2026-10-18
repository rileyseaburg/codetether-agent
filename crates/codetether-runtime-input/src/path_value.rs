//! Workspace-relative tool path handling.
use super::resolve_workspace_path;
use serde_json::{Value, json};
use std::path::Path;

/// Normalize a single JSON path value against a workspace root.
///
/// # Examples
///
/// ```rust
/// use codetether_runtime_input::normalize_path_value;
/// use serde_json::json;
/// use std::path::Path;
///
/// let mut value = json!("src/lib.rs");
/// normalize_path_value(&mut value, Path::new("/workspace"));
///
/// assert_eq!(value, "/workspace/src/lib.rs");
/// ```
/// # Arguments
/// Receives one JSON value and workspace root.
/// # Returns
/// Normalizes string paths in place, preserving non-string values.
pub fn normalize_path_value(value: &mut Value, workspace_dir: &Path) {
    let Some(raw) = value.as_str() else {
        return;
    };
    let resolved = resolve_workspace_path(workspace_dir, raw);
    if !resolved.as_os_str().is_empty() {
        *value = json!(resolved.display().to_string());
    }
}
