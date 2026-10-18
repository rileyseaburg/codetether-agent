//! Derive the legacy import title from its first nonempty line.

/// Use the first nonempty line, truncating long titles to 60 bytes.
/// # Arguments
/// * `text` — Candidate user message.
/// # Returns
/// A trimmed title, or None if no nonempty line exists.
/// # Panics
/// Panics when byte 57 is not a UTF-8 boundary in a title over 60 bytes.
/// # Examples
/// ```
/// assert_eq!(codetether_codex_discovery::derive_title_from_text("\n task "), Some("task".into()));
/// ```
pub fn derive_title_from_text(text: &str) -> Option<String> {
    let first_line = text.lines().map(str::trim).find(|line| !line.is_empty())?;
    let mut title = first_line.to_string();
    if title.len() > 60 {
        title.truncate(57);
        title.push_str("...");
    }
    Some(title)
}
