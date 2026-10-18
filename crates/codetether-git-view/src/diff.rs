//! Capture `git diff --stat` for the TUI git view.

use std::path::Path;

/// Staged + unstaged diff stat summary.
/// # Arguments
/// * `root` - Workspace directory.
/// # Returns
/// Diff statistics, a clean-tree message, or an unavailable marker.
/// # Examples
/// ```no_run
/// let _diff = codetether_git_view::capture_diff_stat(std::path::Path::new("."));
/// ```
pub fn capture_diff_stat(root: &Path) -> String {
    let Ok(output) = std::process::Command::new("git")
        .args(["-C", root.to_string_lossy().as_ref()])
        .args(["diff", "--stat"])
        .output()
    else {
        return "(unavailable)".into();
    };
    let stdout = String::from_utf8_lossy(&output.stdout);
    if stdout.trim().is_empty() {
        "Working tree clean.".into()
    } else {
        stdout.trim().to_string()
    }
}

/// Full staged + unstaged diff for a single `file` (relative or absolute).
///
/// Returns the combined `git diff HEAD -- <file>` patch text, or a short
/// message when the file has no changes or git is unavailable.
/// # Arguments
/// * `root` - Workspace directory.
/// * `file` - Path of the requested file.
/// # Returns
/// Patch text or the existing fallback message.
/// # Examples
/// ```no_run
/// let _patch = codetether_git_view::capture_file_diff(std::path::Path::new("."), std::path::Path::new("README.md"));
/// ```
pub fn capture_file_diff(root: &Path, file: &Path) -> String {
    let Ok(output) = std::process::Command::new("git")
        .args(["-C", root.to_string_lossy().as_ref()])
        .args(["diff", "HEAD", "--", file.to_string_lossy().as_ref()])
        .output()
    else {
        return "(git unavailable)".into();
    };
    let stdout = String::from_utf8_lossy(&output.stdout);
    if stdout.trim().is_empty() {
        format!("No changes vs HEAD for {}", file.display())
    } else {
        stdout.trim_end().to_string()
    }
}
