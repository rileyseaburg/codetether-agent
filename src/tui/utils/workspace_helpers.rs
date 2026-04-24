//! Git/FS helpers backing [`super::workspace::WorkspaceSnapshot`].

use std::path::Path;
use std::process::Command;

pub fn should_skip_entry(name: &str) -> bool {
    matches!(
        name,
        ".git" | "node_modules" | "target" | ".next" | "__pycache__" | ".venv"
    )
}

/// Fetch branch and dirty-file count in a single `git` subprocess.
///
/// `git status --porcelain=v1 --branch` prints one header line
/// (`## branch-name...tracking-info` or `## HEAD (no branch)`) followed
/// by one line per dirty path. Parsing both from one spawn halves the
/// subprocess cost of [`super::workspace::WorkspaceSnapshot::capture`].
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
