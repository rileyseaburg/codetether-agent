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
    PR_PHRASES.iter().any(|phrase| lower.contains(phrase))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_explicit_pr_request() {
        assert!(wants_pr("commit this and create a PR"));
    }

    #[test]
    fn ignores_regular_work_request() {
        assert!(!wants_pr("fix the failing tests"));
    }
}
