//! Sanitize titles and provide a compact fallback label.

/// # Arguments
/// `raw` is a proposed stored title.
/// # Returns
/// Normalized text or `None` for blank/bootstrap input.
/// # Examples
/// ```rust
/// use codetether_session_title::normalize_title;
/// let value = normalize_title(" Review\n storage ");
/// # let _ = value;
/// ```
pub fn normalize_title(raw: &str) -> Option<String> {
    let mut title = String::with_capacity(raw.len());
    for word in raw.split(|ch: char| ch.is_whitespace() || ch.is_control()) {
        if word.is_empty() {
            continue;
        }
        if !title.is_empty() {
            title.push(' ');
        }
        title.push_str(word);
    }
    super::extract::is_title_candidate(&title).then_some(title)
}

/// # Arguments
/// `title` is a stored candidate; `id` is the fallback identity.
/// # Returns
/// A safe normalized title, shortened ID, or `new`.
/// # Examples
/// ```rust
/// use codetether_session_title::label;
/// let value = label(None, "abcdefghijk");
/// # let _ = value;
/// ```
pub fn label(title: Option<&str>, id: &str) -> String {
    title
        .and_then(normalize_title)
        .unwrap_or_else(|| short_id(id))
}

fn short_id(id: &str) -> String {
    let id = id.trim();
    if id.is_empty() {
        "new".to_string()
    } else {
        id.chars().take(8).collect()
    }
}
