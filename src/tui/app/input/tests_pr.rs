//! Tests for PR argument construction.

#[cfg(test)]
mod tests {
    use crate::tui::app::input::pr_command::create_pr_args;
    use crate::worktree::WorktreeInfo;
    use std::path::PathBuf;

    #[test]
    fn pr_args_pin_the_base_branch() {
        let wt = WorktreeInfo {
            name: "tui_example".to_string(),
            path: PathBuf::from("."),
            branch: "codetether/tui_example".to_string(),
            active: true,
        };

        let args = create_pr_args(&wt, Some("feature/current"), None, "", &[]);

        assert_eq!(args[0], "pr");
        assert_eq!(args[1], "create");
        assert!(
            args.windows(2)
                .any(|pair| pair == ["--base", "feature/current"])
        );
    }
}
