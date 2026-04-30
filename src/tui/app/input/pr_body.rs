//! PR body construction from prompt and commit bullets.

/// Build the PR body from commit bullets and optional prompt context.
pub(super) fn build_body(prompt: Option<&str>, bullets: &str) -> String {
    let mut parts = vec!["## Summary".to_string(), summary_text(bullets)];
    if !bullets.is_empty() {
        parts.extend(["## Commits".to_string(), bullets.to_string()]);
    }
    if let Some(p) = prompt.map(str::trim).filter(|p| !p.is_empty()) {
        parts.extend(["## Original request".to_string(), fenced(p)]);
    }
    parts.join("\n\n")
}

fn summary_text(bullets: &str) -> String {
    if bullets.is_empty() {
        "Automated CodeTether TUI agent changes.".to_string()
    } else {
        "Automated CodeTether TUI agent changes from the commits below.".to_string()
    }
}

fn fenced(text: &str) -> String {
    let fence = fence_for(text);
    format!("{fence}text\n{text}\n{fence}")
}

fn fence_for(text: &str) -> String {
    let mut ticks = 0;
    let mut run = 0;
    for ch in text.chars() {
        run = if ch == '`' { run + 1 } else { 0 };
        ticks = ticks.max(run);
    }
    "`".repeat(ticks.max(2) + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn body_wraps_prompt_in_fence() {
        let body = build_body(Some("fix login"), "- abc fix");
        assert!(body.contains("## Original request\n\n```text\nfix login\n```"));
    }

    #[test]
    fn body_uses_longer_fence_for_backticks() {
        let body = build_body(Some("contains ``` fence"), "");
        assert!(body.contains("````text\ncontains ``` fence\n````"));
    }

    #[test]
    fn body_with_nothing_has_summary() {
        let body = build_body(None, "");
        assert!(body.contains("Automated CodeTether TUI agent changes"));
    }
}
