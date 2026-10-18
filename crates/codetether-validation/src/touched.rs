//! Track exactly the file paths reported by supported structured mutations.
use super::{metadata::collect_metadata_paths, paths::normalize_workspace_path};
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

fn is_mutating_tool(tool_name: &str) -> bool {
    matches!(
        tool_name,
        "write"
            | "edit"
            | "advanced_edit"
            | "confirm_edit"
            | "multiedit"
            | "confirm_multiedit"
            | "patch"
    )
}

/// # Arguments
/// Receives the mutable touched set, workspace root, tool name, input and result metadata.
/// # Returns
/// Adds normalized paths from supported mutating tools.
/// # Examples
/// ```
/// let mut files = std::collections::HashSet::new();
/// codetether_validation::track_touched_files(&mut files, std::path::Path::new("/workspace"), "edit", &serde_json::json!({"path":"a.rs"}), None);
/// assert!(files.contains(&std::path::PathBuf::from("/workspace/a.rs")));
/// ```
pub fn track_touched_files(
    touched_files: &mut HashSet<PathBuf>,
    workspace_dir: &Path,
    tool_name: &str,
    tool_input: &Value,
    tool_metadata: Option<&HashMap<String, Value>>,
) {
    if !is_mutating_tool(tool_name) {
        return;
    }

    let mut files = Vec::new();

    super::input_paths::collect(tool_name, tool_input, tool_metadata, &mut files);

    collect_metadata_paths(&mut files, tool_metadata, &["path", "file"]);

    for file in files {
        touched_files.insert(normalize_workspace_path(workspace_dir, &file));
    }
}
