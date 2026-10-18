//! Read-only snapshots of Git dirty paths for caller bookkeeping.
use super::paths::normalize_workspace_path;
use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};
use tokio::process::Command;

/// # Arguments
/// Receives the workspace directory used for Git queries.
/// # Returns
/// The normalized union of staged, unstaged and untracked paths; failed queries are skipped.
/// # Examples
/// ```no_run
/// # async fn example() {
/// let paths = codetether_validation::capture_git_dirty_files(std::path::Path::new(".")).await;
/// # }
/// ```
pub async fn capture_git_dirty_files(workspace_dir: &Path) -> HashSet<PathBuf> {
    let mut files = HashSet::new();
    for args in [
        &["diff", "--name-only", "--relative", "--"][..],
        &["diff", "--cached", "--name-only", "--relative", "--"][..],
        &["ls-files", "--others", "--exclude-standard"][..],
    ] {
        let output = Command::new("git")
            .args(args)
            .current_dir(workspace_dir)
            .output()
            .await;
        let Ok(output) = output else {
            continue;
        };
        if !output.status.success() {
            continue;
        }
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
        {
            files.insert(normalize_workspace_path(workspace_dir, line));
        }
    }
    files
}
