//! Classification of tools whose proposed file contents can be reconstructed.
//!
//! Concrete reconstruction and filesystem access are supplied by tool implementations.

/// Whether a tool supports reconstruction of its proposed file contents.
///
/// # Arguments
/// * `tool` — Registered tool identifier.
/// # Returns
/// Whether the standard tool implementation supports file-content previews.
/// # Examples
/// ```
/// assert!(codetether_tool_core::proposed::supported("write"));
/// assert!(!codetether_tool_core::proposed::supported("read"));
/// ```
pub fn supported(tool: &str) -> bool {
    matches!(
        tool,
        "apply_patch" | "patch" | "write" | "edit" | "multiedit"
    )
}

use anyhow::Result;
use serde_json::Value;
use std::path::{Path, PathBuf};

/// Resolve an absolute or workspace-relative proposed file path.
///
/// # Arguments
/// * `root` - Base directory for relative paths.
/// * `path` - Requested file path.
/// # Returns
/// The absolute input or its path joined to the base directory.
/// # Examples
/// ```rust
/// let path = codetether_tool_core::proposed::resolve(std::path::Path::new("workspace"), "file.rs");
/// assert_eq!(path, std::path::PathBuf::from("workspace/file.rs"));
/// ```
pub fn resolve(root: &Path, path: &str) -> PathBuf {
    let path = Path::new(path);
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        root.join(path)
    }
}

/// Read a required text field from proposed-edit arguments.
///
/// # Arguments
/// * `args` - Tool arguments containing the field.
/// * `key` - Required string field name.
/// # Returns
/// The string borrowed from the arguments.
/// # Errors
/// Returns an error if the field is absent or is not a string.
/// # Examples
/// ```rust
/// let args = serde_json::json!({"path":"file.rs"});
/// assert_eq!(codetether_tool_core::proposed::text(&args, "path").unwrap(), "file.rs");
/// ```
pub fn text<'a>(args: &'a Value, key: &str) -> Result<&'a str> {
    args.get(key)
        .and_then(Value::as_str)
        .ok_or_else(|| anyhow::anyhow!("missing `{key}` argument"))
}
