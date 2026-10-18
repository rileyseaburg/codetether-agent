//! PR body construction from prompt and commit bullets.

/// Build the PR body from commit bullets and optional prompt context.
///
/// # Arguments
/// * `prompt` — prompt input.
/// * `bullets` — bullets input.
///
/// # Returns
/// The prepared text, arguments, or policy result described above.
///
/// # Examples
/// ```
/// assert!(codetether_github_pr::worktree::body::build_body(None, "").contains("## Summary"));
/// ```
pub fn build_body(prompt: Option<&str>, bullets: &str) -> String {
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
#[path = "body_tests.rs"]
mod tests;
