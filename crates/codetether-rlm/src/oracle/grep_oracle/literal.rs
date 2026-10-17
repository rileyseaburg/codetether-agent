//! Infer explicitly quoted and occurrence literals.

use regex::Regex;
pub(super) fn infer(query: &str) -> Option<String> {
    // First, try to extract an explicit quoted literal pattern from the query.
    // Examples:
    // - Find all occurrences of 'async fn' in file.rs
    // - grep for "Result<"
    let quoted_re = Regex::new(
        r#"(?i)(?:occurrences?\s+of|grep(?:\s+for)?|matching|containing)\s+['"`]([^'"`]+)['"`]"#,
    )
    .ok()?;
    if let Some(caps) = quoted_re.captures(query)
        && let Some(m) = caps.get(1)
    {
        return Some(regex::escape(m.as_str()));
    }

    // Fallback: capture any quoted literal in the query.
    let any_quoted_re = Regex::new(r#"['"`]([^'"`]+)['"`]"#).ok()?;
    if let Some(caps) = any_quoted_re.captures(query)
        && let Some(m) = caps.get(1)
    {
        return Some(regex::escape(m.as_str()));
    }

    // Fallback: handle unquoted "occurrences of X in ..." forms.
    let bare_occurrences_re = Regex::new(r"(?i)occurrences?\s+of\s+(.+?)(?:\s+in\b|$)").ok()?;
    if let Some(caps) = bare_occurrences_re.captures(query)
        && let Some(m) = caps.get(1)
    {
        let candidate = m.as_str().trim().trim_matches(&['"', '\'', '`'][..]);
        if !candidate.is_empty() {
            return Some(regex::escape(candidate));
        }
    }

    None
}
