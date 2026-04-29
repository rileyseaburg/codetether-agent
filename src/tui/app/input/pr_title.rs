//! PR title construction from branch names and commit summaries.

/// Build a stable PR title from worktree metadata.
pub(super) fn build_title(worktree_name: &str, commits: &[String]) -> String {
    commits
        .iter()
        .find_map(|c| commit_subject(c.as_str()))
        .map(clean_subject)
        .filter(|t| !t.is_empty())
        .unwrap_or_else(|| format!("codetether: {worktree_name}"))
}

fn commit_subject(line: &str) -> Option<&str> {
    line.split_once(' ').map(|(_, subject)| subject.trim())
}

fn clean_subject(subject: &str) -> String {
    let title = subject.strip_prefix("codetether: ").unwrap_or(subject);
    truncate_title(title, 72).trim().to_string()
}

/// Truncate `s` to at most `max` bytes on a UTF-8-safe word boundary.
pub(super) fn truncate_title(s: &str, max: usize) -> &str {
    let Some(prefix) = utf8_prefix(s, max) else {
        return s;
    };
    let boundary = prefix.rfind(char::is_whitespace).unwrap_or(prefix.len());
    &prefix[..boundary]
}

fn utf8_prefix(s: &str, max: usize) -> Option<&str> {
    if s.len() <= max {
        return None;
    }
    let boundary = s
        .char_indices()
        .map(|(i, _)| i)
        .take_while(|i| *i <= max)
        .last()?;
    Some(&s[..boundary])
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
    fn truncate_title_handles_unicode() {
        let result = truncate_title("fix 🦀 unicode title boundary", 8);
        assert_eq!(result, "fix");
    }
}
