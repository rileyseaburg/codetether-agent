//! PR title construction from branch names and commit summaries.

/// Build a stable PR title from worktree metadata.
pub(super) fn build_title(worktree_name: &str, commits: &[String]) -> String {
    commits
        .iter()
        .find_map(|c| commit_subject(c))
        .map(clean_subject)
        .filter(|t| !t.is_empty())
        .unwrap_or_else(|| format!("codetether: {worktree_name}"))
}

fn commit_subject(line: &String) -> Option<&str> {
    line.split_once(' ').map(|(_, subject)| subject.trim())
}

fn clean_subject(subject: &str) -> String {
    let title = subject
        .strip_prefix("codetether: ")
        .unwrap_or(subject)
        .trim();
    truncate_title(title, 72).trim().to_string()
}

/// Truncate `s` to at most `max` characters on a word boundary.
pub(super) fn truncate_title(s: &str, max: usize) -> &str {
    if s.len() <= max {
        return s;
    }
    let boundary = s[..=max].rfind(char::is_whitespace).unwrap_or(max);
    &s[..boundary]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn title_prefers_commit_subject() {
        let commits = vec!["abc123 fix: clean PR title generation".into()];
        assert_eq!(
            build_title("tui_abc", &commits),
            "fix: clean PR title generation"
        );
    }

    #[test]
    fn title_uses_worktree_name_when_commits_are_empty() {
        assert_eq!(build_title("tui_abc", &[]), "codetether: tui_abc");
    }
}
