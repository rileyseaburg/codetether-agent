//! Text helpers for checkpoint state extraction.

/// Scan text for the last URL that looks like a browser page.
pub(super) fn extract_browser_url(text: &str, url: &mut Option<String>) {
    for line in text.lines().rev() {
        if let Some(found) = line.split_whitespace().find(|s| s.starts_with("https://")) {
            *url = Some(clean_url(found));
            return;
        }
        if let Some(found) = line.split_whitespace().find(|s| s.starts_with("http://")) {
            *url = Some(clean_url(found));
            return;
        }
    }
}

/// Truncate text to the first non-empty line, capped at `max` chars.
pub(super) fn truncate_to_line(text: &str, max: usize) -> String {
    let line = text.lines().find(|l| !l.trim().is_empty()).unwrap_or("");
    if line.len() <= max {
        line.to_string()
    } else {
        format!("{}...", &line[..max.saturating_sub(3)])
    }
}

fn clean_url(found: &str) -> String {
    found
        .trim_end_matches(',')
        .trim_end_matches('.')
        .to_string()
}
