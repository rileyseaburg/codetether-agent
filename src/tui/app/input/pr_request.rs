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
pub(super) fn wants_pr(prompt: &str) -> bool {
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
mod tests {
    use super::*;

    #[test]
    fn detects_explicit_pr_request() {
        assert!(wants_pr("commit this and create a PR"));
    }

    #[test]
    fn rejects_substring_matches() {
        assert!(!wants_pr("create a program"));
        assert!(!wants_pr("open product notes"));
    }
}
