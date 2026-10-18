//! Detect explicit user intent to publish a pull request.

const PR_PHRASES: [&str; 6] = [
    "create a pr",
    "create pr",
    "open a pr",
    "open pr",
    "create a pull request",
    "open a pull request",
];

/// Return true when the prompt explicitly asks to publish a PR.
///
/// # Arguments
/// * `prompt` — prompt input.
///
/// # Returns
/// The prepared text, arguments, or policy result described above.
///
/// # Examples
/// ```
/// assert!(codetether_github_pr::worktree::request::wants_pr("create a PR"));
/// ```
pub fn wants_pr(prompt: &str) -> bool {
    let lower = prompt.to_ascii_lowercase();
    PR_PHRASES
        .iter()
        .any(|phrase| contains_phrase(&lower, phrase))
}

fn contains_phrase(text: &str, phrase: &str) -> bool {
    text.match_indices(phrase)
        .any(|(idx, _)| has_boundaries(text, phrase, idx))
}

fn has_boundaries(text: &str, phrase: &str, idx: usize) -> bool {
    let before = text[..idx].chars().next_back();
    let after = text[idx + phrase.len()..].chars().next();
    !before.is_some_and(char::is_alphanumeric) && !after.is_some_and(char::is_alphanumeric)
}
#[cfg(test)]
#[path = "request_tests.rs"]
mod tests;
