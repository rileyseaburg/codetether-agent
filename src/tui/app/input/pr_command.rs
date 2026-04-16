//! GitHub PR command builders for TUI worktrees.
//!
//! Keeps CLI argument construction separate from process
//! execution so the base-branch selection logic is testable.

use crate::worktree::WorktreeInfo;

/// Truncate `s` to at most `max` characters on a word boundary.
fn truncate_title(s: &str, max: usize) -> &str {
    if s.len() <= max {
        return s;
    }
    let boundary = s[..=max]
        .rfind(|c: char| c.is_whitespace())
        .unwrap_or(max);
    &s[..boundary]
}

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
    let title = prompt
        .map(|p| truncate_title(p.trim(), 72).to_string())
        .filter(|t| !t.is_empty())
        .unwrap_or_else(|| format!("codetether: {}", wt.name));
    let body = build_body(prompt, commit_bullets);
    args.extend(["--title".to_string(), title, "--body".to_string(), body]);
    args
}

/// Build the PR body from prompt and commit bullets.
fn build_body(prompt: Option<&str>, bullets: &str) -> String {
    let mut parts = Vec::new();
    if let Some(p) = prompt {
        parts.push(format!("**Prompt:** {p}"));
    }
    if !bullets.is_empty() {
        parts.push(bullets.to_string());
    }
    if parts.is_empty() {
        "Automated PR from CodeTether TUI agent.".to_string()
    } else {
        parts.join("\n\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn truncate_title_short_input_unchanged() {
        assert_eq!(truncate_title("fix bug", 72), "fix bug");
    }

    #[test]
    fn truncate_title_long_input_on_word() {
        let long = "fix the login bug that causes users to see a blank screen after submitting credentials";
        let result = truncate_title(long, 40);
        assert!(result.len() <= 40);
        assert!(!result.ends_with(' '));
    }

    #[test]
    fn create_pr_args_uses_prompt_as_title() {
        let wt = WorktreeInfo {
            name: "tui_abc".into(),
            path: std::path::PathBuf::from("."),
            branch: "codetether/tui_abc".into(),
            active: true,
        };
        let args = create_pr_args(&wt, None, Some("fix login bug"), "");
        let title_idx = args.iter().position(|a| a == "--title").unwrap();
        assert_eq!(args[title_idx + 1], "fix login bug");
    }
}
