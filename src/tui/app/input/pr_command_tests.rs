//! Tests for PR command text quality.

#[cfg(test)]
mod tests {
    use crate::tui::app::input::pr_command::create_pr_args;
    use crate::worktree::WorktreeInfo;
    use std::path::PathBuf;

    #[test]
    fn create_pr_args_uses_commit_as_title() {
        let wt = WorktreeInfo {
            name: "tui_abc".into(),
            path: PathBuf::from("."),
            branch: "codetether/tui_abc".into(),
            active: true,
        };
        let commits = vec!["abc123 fix: login bug".into()];
        let args = create_pr_args(&wt, None, Some("raw user prompt"), "", &commits);
        let idx = args.iter().position(|a| a == "--title").unwrap();
        assert_eq!(args[idx + 1], "fix: login bug");
    }
}
