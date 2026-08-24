//! Artifact inventory derived from a worktree's actual contents.
//!
//! A sub-agent's self-report is narration: it may claim success without
//! writing anything, or report a blocker after writing dozens of files
//! when only its *verification* failed. This module measures what was
//! written by asking git, so orchestration reads artifacts instead of
//! trusting prose.

use std::path::Path;

/// Lists files the sub-agent created or modified in `worktree`.
///
/// Returns paths relative to the worktree root, sorted and deduplicated.
/// Returns an empty vector when git is unavailable or nothing changed.
pub async fn written_files(worktree: &Path) -> Vec<String> {
    let Ok(output) = crate::tool::git::process::output_refs(
        worktree,
        &["status", "--porcelain", "--untracked-files=all"],
        false,
    )
    .await
    else {
        return Vec::new();
    };
    if !output.status.success() {
        return Vec::new();
    }
    let mut files = String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter_map(super::inventory_parse::changed_path)
        .collect::<Vec<_>>();
    files.sort();
    files.dedup();
    files
}

#[cfg(test)]
#[path = "worktree_inventory_tests.rs"]
mod tests;
