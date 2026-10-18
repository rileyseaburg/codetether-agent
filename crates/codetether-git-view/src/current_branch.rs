//! Branch detection helpers for TUI worktree PRs.
//!
//! Captures the branch the user launched the TUI task from so
//! follow-up GitHub pull requests can target the correct base.
//!
//! # Examples
//!
//! ```no_run
//! let base = codetether_git_view::current_branch(std::path::Path::new("."));
//! assert!(base.is_some());
//! ```

use std::path::Path;
use std::process::Command;

/// Read the current git branch for TUI PR targeting.
///
/// Returns `None` when git is unavailable, the command fails,
/// or the repository is in a detached `HEAD` state.
///
/// # Examples
///
/// ```no_run
/// let base = current_branch(std::path::Path::new("."));
/// ```
///
/// # Arguments
/// * `dir` — Repository working directory.
///
/// # Returns
/// Current branch name or None for detached HEAD, empty output, or Git failure.
///
pub fn current_branch(dir: &Path) -> Option<String> {
    let output = Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .current_dir(dir)
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let branch = String::from_utf8_lossy(&output.stdout).trim().to_string();
    (!branch.is_empty() && branch != "HEAD").then_some(branch)
}
