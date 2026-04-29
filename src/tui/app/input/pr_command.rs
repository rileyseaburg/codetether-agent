//! GitHub PR command builders for TUI worktrees.

use crate::worktree::WorktreeInfo;

use super::pr_body::build_body;
use super::pr_title::build_title;

/// Build the `gh pr create` arguments for a TUI worktree.
pub(super) fn create_pr_args(
    wt: &WorktreeInfo,
    base_branch: Option<&str>,
    prompt: Option<&str>,
    commit_bullets: &str,
    commits: &[String],
) -> Vec<String> {
    let mut args = vec![
        "pr".into(),
        "create".into(),
        "--head".into(),
        wt.branch.clone(),
    ];
    if let Some(base) = base_branch.filter(|b| !b.is_empty()) {
        args.extend(["--base".to_string(), base.to_string()]);
    }
    args.extend(pr_text_args(wt, prompt, commit_bullets, commits));
    args
}

fn pr_text_args(
    wt: &WorktreeInfo,
    prompt: Option<&str>,
    commit_bullets: &str,
    commits: &[String],
) -> Vec<String> {
    vec![
        "--title".into(),
        build_title(&wt.name, commits),
        "--body".into(),
        build_body(prompt, commit_bullets),
    ]
}
