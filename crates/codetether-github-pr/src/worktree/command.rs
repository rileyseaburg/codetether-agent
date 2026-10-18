//! GitHub PR command builders for TUI worktrees.

use codetether_worktree::WorktreeInfo;

use super::body::build_body;
use super::title::build_title;

/// Build the `gh pr create` arguments for a TUI worktree.
///
/// # Arguments
/// * `wt` — wt input.
/// * `base_branch` — base branch input.
/// * `prompt` — prompt input.
/// * `commit_bullets` — commit bullets input.
/// * `commits` — commits input.
///
/// # Returns
/// The prepared text, arguments, or policy result described above.
///
/// # Examples
/// ```
/// # fn example(wt: &codetether_worktree::WorktreeInfo) {
/// let args = codetether_github_pr::worktree::command::create_pr_args(wt, None, None, "", &[]);
/// assert_eq!(args[0], "pr");
/// # }
/// ```
pub fn create_pr_args(
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

#[cfg(test)]
#[path = "command_tests.rs"]
mod tests;
