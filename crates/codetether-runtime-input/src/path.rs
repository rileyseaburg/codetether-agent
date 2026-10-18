//! Workspace-relative tool path handling.
use super::normalize_path_value;
use serde_json::{Map, Value};
use std::path::Path;
const PATH_KEYS: &[&str] = &["path", "file", "file_path", "filePath", "cwd", "workdir"];
const PATH_COLLECTION_KEYS: &[&str] = &["edits", "changes", "operations"];

/// Normalize supported path fields in a tool input object against a workspace root.
///
/// # Examples
///
/// ```rust
/// use codetether_runtime_input::normalize_workspace_paths;
/// use serde_json::{Map, Value, json};
/// use std::path::Path;
///
/// let mut obj = Map::<String, Value>::new();
/// obj.insert("path".to_string(), json!("src/lib.rs"));
/// normalize_workspace_paths(&mut obj, Path::new("/workspace"));
///
/// assert_eq!(obj["path"], "/workspace/src/lib.rs");
/// ```
/// # Arguments
/// Receives a tool object and workspace root.
/// # Returns
/// Normalizes supported top-level and collection path fields in place.
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
