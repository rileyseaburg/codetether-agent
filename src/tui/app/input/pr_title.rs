//! PR title construction from user prompts.
//!
//! Truncates the prompt to a GitHub-compatible title length
//! on a word boundary.

/// Truncate `s` to at most `max` characters on a word boundary.
///
/// Returns the input unchanged if it fits within `max`.
///
/// # Examples
///
/// ```ignore
/// assert_eq!(truncate_title("fix bug", 72), "fix bug");
/// ```
pub(super) fn truncate_title(s: &str, max: usize) -> &str {
    if s.len() <= max {
        return s;
    }
    let boundary = s[..=max]
        .rfind(|c: char| c.is_whitespace())
        .unwrap_or(max);
    &s[..boundary]
}

/// Build the PR title from a user prompt.
///
/// Falls back to `"codetether: {name}"` when no prompt
/// is available or the prompt is empty.
pub(super) fn build_title(
    prompt: Option<&str>,
    fallback_name: &str,
) -> String {
    prompt
        .map(|p| truncate_title(p.trim(), 72).to_string())
        .filter(|t| !t.is_empty())
        .unwrap_or_else(|| format!("codetether: {fallback_name}"))
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
        let long = "fix the login bug that causes users to see a blank screen after submitting";
        let result = truncate_title(long, 40);
        assert!(result.len() <= 40);
        assert!(!result.ends_with(' '));
    }
}
