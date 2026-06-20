//! Decides which files post-edit validation may flag.
//!
//! Scope is strictly the files THIS agent edited via structured tools
//! (tracked in `touched_files`). The whole working tree's git-dirty diff is
//! deliberately NOT unioned in: in shared worktrees or multi-agent runs a file
//! dirtied by another agent, a concurrent process, or an unrelated commit would
//! otherwise be attributed to this agent and trigger spurious guard rejections
//! (e.g. being told to refactor a file it never touched). Inline shell file
//! writes are blocked by `bash_file_edit_guard`, so `touched_files` is
//! authoritative.

use std::collections::HashSet;
use std::path::PathBuf;

/// Returns the set of files validation is allowed to flag.
///
/// `baseline_git_dirty_files` is accepted for call-site compatibility and to
/// allow future scope tightening; it never widens the set.
pub(crate) fn validation_scope(
    touched_files: &HashSet<PathBuf>,
    baseline_git_dirty_files: &HashSet<PathBuf>,
) -> HashSet<PathBuf> {
    let _ = baseline_git_dirty_files;
    touched_files.clone()
}

#[cfg(test)]
#[path = "validation_scope_tests.rs"]
mod validation_scope_tests;
