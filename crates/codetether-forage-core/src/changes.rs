//! Read Git change sets used to identify files affected by forage execution.
use std::collections::BTreeSet;
use std::process::Command;

/// Snapshot tracked and untracked changed paths in the current Git worktree.
///
/// # Arguments
/// This function takes no arguments and uses the process working directory.
/// # Returns
/// A sorted set of changed paths, or no snapshot when Git/workspace detection fails.
/// # Examples
/// ```no_run
/// let changed = codetether_forage_core::changes::snapshot_git_changed_files();
/// ```
pub fn snapshot_git_changed_files() -> Option<BTreeSet<String>> {
    let cwd = std::env::current_dir().ok()?;
    let is_git_repo = Command::new("git")
        .args(["rev-parse", "--is-inside-work-tree"])
        .current_dir(&cwd)
        .output()
        .ok()?
        .status
        .success();
    if !is_git_repo {
        return None;
    }

    let mut changed = BTreeSet::new();
    changed.extend(git_name_list(&cwd, &["diff", "--name-only"]));
    changed.extend(git_name_list(&cwd, &["diff", "--name-only", "--cached"]));
    changed.extend(git_name_list(
        &cwd,
        &["ls-files", "--others", "--exclude-standard"],
    ));
    Some(changed)
}

fn git_name_list(cwd: &std::path::Path, args: &[&str]) -> Vec<String> {
    let Ok(output) = Command::new("git").args(args).current_dir(cwd).output() else {
        return Vec::new();
    };
    if !output.status.success() {
        return Vec::new();
    }

    String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(ToString::to_string)
        .collect()
}
