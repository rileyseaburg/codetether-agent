//! Collect commit messages from a worktree for PR descriptions.
//!
//! Runs `git log --oneline` inside the worktree to gather a
//! summary of all commits since branching from the base.

use std::path::Path;

/// Return the one-line commit log between `base` and HEAD.
///
/// Falls back to the last 20 commits if `base` is `None`.
///
/// # Arguments
/// * `dir` — dir input.
/// * `base` — base input.
///
/// # Returns
/// The prepared text, arguments, or policy result described above.
///
/// # Examples
/// ```no_run
/// # async fn example() {
/// let _commits = codetether_github_pr::worktree::description::collect_commit_log(std::path::Path::new("."), None).await;
/// # }
/// ```
pub async fn collect_commit_log(dir: &Path, base: Option<&str>) -> Vec<String> {
    let range = base
        .map(|b| format!("{b}..HEAD"))
        .unwrap_or_else(|| "HEAD~20..HEAD".to_string());

    let output = tokio::process::Command::new("git")
        .args(["log", "--oneline", "--no-decorate", &range])
        .current_dir(dir)
        .output()
        .await;

    match output {
        Ok(o) if o.status.success() => String::from_utf8_lossy(&o.stdout)
            .lines()
            .filter(|l| !l.is_empty())
            .map(String::from)
            .collect(),
        _ => vec![],
    }
}

/// Format a commit log as a markdown bullet list.
///
/// # Arguments
/// * `commits` — commits input.
///
/// # Returns
/// The prepared text, arguments, or policy result described above.
///
/// # Examples
/// ```
/// assert_eq!(codetether_github_pr::worktree::description::format_commit_bullets(&["abc change".into()]), "- abc change");
/// ```
pub fn format_commit_bullets(commits: &[String]) -> String {
    commits
        .iter()
        .map(|c| format!("- {c}"))
        .collect::<Vec<_>>()
        .join("\n")
}
