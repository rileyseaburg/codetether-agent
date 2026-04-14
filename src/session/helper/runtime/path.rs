use serde_json::{Map, Value, json};
use std::path::{Path, PathBuf};

const PATH_KEYS: &[&str] = &["path", "file", "file_path", "filePath", "cwd"];
const PATH_COLLECTION_KEYS: &[&str] = &["edits", "changes", "operations"];

/// Normalize supported path fields in a tool input object against a workspace root.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::session::helper::runtime::normalize_workspace_paths;
/// use serde_json::{Map, Value, json};
/// use std::path::Path;
///
/// let mut obj = Map::<String, Value>::new();
/// obj.insert("path".to_string(), json!("src/lib.rs"));
/// normalize_workspace_paths(&mut obj, Path::new("/workspace"));
///
/// assert_eq!(obj["path"], "/workspace/src/lib.rs");
/// ```
pub fn normalize_workspace_paths(obj: &mut Map<String, Value>, workspace_dir: &Path) {
    for key in PATH_KEYS {
        if let Some(value) = obj.get_mut(*key) {
            normalize_path_value(value, workspace_dir);
        }
    }
    for key in PATH_COLLECTION_KEYS {
        if let Some(items) = obj.get_mut(*key).and_then(Value::as_array_mut) {
            for item in items.iter_mut().filter_map(Value::as_object_mut) {
                for nested_key in PATH_KEYS {
                    if let Some(value) = item.get_mut(*nested_key) {
                        normalize_path_value(value, workspace_dir);
                    }
                }
            }
        }
    }
}

/// Normalize a single JSON path value against a workspace root.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::session::helper::runtime::normalize_path_value;
/// use serde_json::json;
/// use std::path::Path;
///
/// let mut value = json!("src/lib.rs");
/// normalize_path_value(&mut value, Path::new("/workspace"));
///
/// assert_eq!(value, "/workspace/src/lib.rs");
/// ```
pub fn normalize_path_value(value: &mut Value, workspace_dir: &Path) {
    let Some(raw) = value.as_str() else {
        return;
    };
    let resolved = resolve_workspace_path(workspace_dir, raw);
    if !resolved.as_os_str().is_empty() {
        *value = json!(resolved.display().to_string());
    }
}

/// Resolve a raw path string against a workspace root.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::session::helper::runtime::resolve_workspace_path;
/// use std::path::{Path, PathBuf};
///
/// let path = resolve_workspace_path(Path::new("/workspace"), "src/lib.rs");
/// assert_eq!(path, PathBuf::from("/workspace/src/lib.rs"));
/// ```
pub fn resolve_workspace_path(workspace_dir: &Path, raw: &str) -> PathBuf {
    if raw.is_empty() || raw.contains("://") || raw.starts_with("data:") || raw.starts_with('~') {
        return PathBuf::from(raw);
    }
    let candidate = PathBuf::from(raw);
    if candidate.is_absolute() {
        candidate
    } else {
        workspace_dir.join(candidate)
    }
}
