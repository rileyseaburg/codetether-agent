//! Workspace-relative display labels for shared files.
use std::path::Path;
/// Format a file path relative to the workspace when possible.
/// # Arguments
/// * `path` - Path to display.
/// * `workspace_dir` - Preferred path prefix to omit.
/// # Returns
/// The relative path, or the original path when it is outside the workspace.
/// # Examples
/// ```
/// use std::path::Path;
/// assert_eq!(codetether_file_share::display_path_for_workspace(Path::new("/work/a"), Path::new("/work")), "a");
/// ```
pub fn display_path_for_workspace(path: &Path, workspace_dir: &Path) -> String {
    path.strip_prefix(workspace_dir)
        .unwrap_or(path)
        .display()
        .to_string()
}
