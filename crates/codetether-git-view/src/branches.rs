//! Capture `git branch -v` for the TUI git view.

use std::path::Path;

/// Local branches with last-commit info.
/// # Arguments
/// * `root` - Workspace directory.
/// # Returns
/// Local branch lines, or an empty vector when Git cannot be launched.
/// # Examples
/// ```no_run
/// let _branches = codetether_git_view::capture_branches(std::path::Path::new("."));
/// ```
pub fn capture_branches(root: &Path) -> Vec<String> {
    let Ok(output) = std::process::Command::new("git")
        .args(["-C", root.to_string_lossy().as_ref()])
        .args(["branch", "-v"])
        .output()
    else {
        return vec![];
    };
    String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(String::from)
        .collect()
}
