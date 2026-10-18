//! Capture branch identity and dirty paths in one Git status command.
use std::{path::Path, process::Command};

/// Fetch branch and dirty-file count in a single `git` subprocess.
///
/// `git status --porcelain=v1 --branch` prints one header line
/// (`## branch-name...tracking-info` or `## HEAD (no branch)`) followed
/// by one line per dirty path. Parsing both from one spawn halves the
/// subprocess cost of workspace snapshot capture.
/// # Arguments
/// * `root` - Workspace directory.
/// # Returns
/// Branch name and count of dirty entries; unavailable status returns None and zero.
/// # Examples
/// ```no_run
/// let _status = codetether_workspace_snapshot::detect_git_status(std::path::Path::new("."));
/// ```
pub fn detect_git_status(root: &Path) -> (Option<String>, usize) {
    let Ok(output) = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(["status", "--porcelain=v1", "--branch"])
        .output()
    else {
        return (None, 0);
    };
    if !output.status.success() {
        return (None, 0);
    }
    let text = String::from_utf8_lossy(&output.stdout);
    let mut branch = None;
    let mut dirty = 0usize;
    for line in text.lines() {
        if let Some(rest) = line.strip_prefix("## ") {
            let name = rest.split("...").next().unwrap_or(rest).trim().to_string();
            if !name.is_empty() && name != "HEAD (no branch)" {
                branch = Some(name);
            }
        } else if !line.trim().is_empty() {
            dirty += 1;
        }
    }
    (branch, dirty)
}
