//! Static changed-file and changed-line metrics for swarm worktrees.

use anyhow::{Context, Result};
use std::collections::HashSet;
use std::path::Path;

pub(super) fn files(worktree: &Path) -> Result<HashSet<String>> {
    let output = git(worktree, &["diff", "--name-only"])?;
    if !output.status.success() {
        return Ok(HashSet::new());
    }
    Ok(String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(str::to_string)
        .collect())
}

pub(super) fn lines(worktree: &Path) -> Result<u32> {
    let output = git(worktree, &["diff", "--numstat"])?;
    if !output.status.success() {
        return Ok(0);
    }
    Ok(String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(|line| {
            line.split('\t')
                .take(2)
                .filter_map(|part| part.parse::<u32>().ok())
                .sum::<u32>()
        })
        .sum())
}

fn git(worktree: &Path, args: &[&str]) -> Result<std::process::Output> {
    crate::tool::git::process::output_blocking_refs(worktree, args, false)
        .with_context(|| format!("Failed to collect Git metrics in {}", worktree.display()))
}
