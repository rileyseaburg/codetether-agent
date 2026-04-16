//! GitHub PR command builders for TUI worktrees.
//!
//! Keeps CLI argument construction separate from process
//! execution so the base-branch selection logic is testable.

use crate::worktree::WorktreeInfo;

use super::pr_body::build_body;
use super::pr_title::build_title;

/// Build the `gh pr create` arguments for a TUI worktree.
///
/// Uses `prompt` for the PR title when available, falling back
/// to the worktree name.  The body includes the original prompt
/// and commit bullet points when provided.
pub(super) fn create_pr_args(
    wt: &WorktreeInfo,
    base_branch: Option<&str>,
    prompt: Option<&str>,
    commit_bullets: &str,
) -> Vec<String> {
    let mut args = vec![
        "pr".to_string(),
        "create".to_string(),
        "--head".to_string(),
        wt.branch.clone(),
    ];
    if let Some(base) = base_branch.filter(|b| !b.is_empty()) {
        args.push("--base".to_string());
        args.push(base.to_string());
    }
    let title = build_title(prompt, &wt.name);
    let body = build_body(prompt, commit_bullets);
    args.extend(["--title".to_string(), title, "--body".to_string(), body]);
    args
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn create_pr_args_uses_prompt_as_title() {
        let wt = WorktreeInfo {
            name: "tui_abc".into(),
            path: PathBuf::from("."),
            branch: "codetether/tui_abc".into(),
            active: true,
        };
        let args = create_pr_args(&wt, None, Some("fix login bug"), "");
        let idx = args.iter().position(|a| a == "--title").unwrap();
        assert_eq!(args[idx + 1], "fix login bug");
    }
}
