//! GitHub PR command builders for TUI worktrees.
//!
//! Keeps CLI argument construction separate from process
//! execution so the base-branch selection logic is testable.
//!
//! # Examples
//!
//! ```ignore
//! let args = create_pr_args(&worktree, Some("feature/current"));
//! assert!(args.iter().any(|arg| arg == "--base"));
//! ```

use crate::worktree::WorktreeInfo;

/// Build the `gh pr create` arguments for a TUI worktree.
///
/// When a base branch is available, it is passed explicitly so
/// GitHub compares the worktree branch against the branch the
/// user started from instead of the repository default branch.
///
/// # Examples
///
/// ```ignore
/// let args = create_pr_args(&worktree, Some("feature/current"));
/// ```
pub(super) fn create_pr_args(wt: &WorktreeInfo, base_branch: Option<&str>) -> Vec<String> {
    let mut args = vec![
        "pr".to_string(),
        "create".to_string(),
        "--head".to_string(),
        wt.branch.clone(),
    ];
    if let Some(base) = base_branch.filter(|branch| !branch.is_empty()) {
        args.push("--base".to_string());
        args.push(base.to_string());
    }
    args.extend([
        "--title".to_string(),
        format!("codetether: {}", wt.name),
        "--body".to_string(),
        format!(
            "Automated PR from CodeTether TUI agent.\n\nBranch: `{}`",
            wt.branch
        ),
        "--fill-verbose".to_string(),
    ]);
    args
}
