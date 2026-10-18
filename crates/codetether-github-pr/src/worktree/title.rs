//! PR title construction from branch names and commit summaries.

/// Build a stable PR title from worktree metadata.
///
/// # Arguments
/// * `worktree_name` — worktree name input.
/// * `commits` — commits input.
///
/// # Returns
/// The prepared text, arguments, or policy result described above.
///
/// # Examples
/// ```
/// assert_eq!(codetether_github_pr::worktree::title::build_title("task", &[]), "codetether: task");
/// ```
pub fn build_title(worktree_name: &str, commits: &[String]) -> String {
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
///
/// # Arguments
/// * `s` — s input.
/// * `max` — max input.
///
/// # Returns
/// The prepared text, arguments, or policy result described above.
///
/// # Examples
/// ```
/// assert_eq!(codetether_github_pr::worktree::title::truncate_title("short", 72), "short");
/// ```
pub fn truncate_title(s: &str, max: usize) -> &str {
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
#[path = "title_tests.rs"]
mod tests;
