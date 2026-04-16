//! PR body construction from prompt and commit bullets.

/// Build the PR body from prompt and commit bullets.
///
/// Returns a generic message when both are empty.
///
/// # Examples
///
/// ```ignore
/// let body = build_body(Some("fix bug"), "- abc123 fix\n");
/// assert!(body.contains("**Prompt:**"));
/// ```
pub(super) fn build_body(prompt: Option<&str>, bullets: &str) -> String {
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
    fn body_with_prompt_and_bullets() {
        let body = build_body(Some("fix login"), "- abc fix\n");
        assert!(body.contains("**Prompt:** fix login"));
        assert!(body.contains("- abc fix"));
    }

    #[test]
    fn body_with_nothing() {
        let body = build_body(None, "");
        assert!(body.contains("CodeTether TUI agent"));
    }
}
