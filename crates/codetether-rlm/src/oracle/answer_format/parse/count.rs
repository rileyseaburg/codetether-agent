//! Extract a count from natural-language answer text.

/// Extract a count number from natural language text.
pub(super) fn extract_count_from_text(text: &str) -> Option<usize> {
    // Look for patterns like "15 functions", "count: 42", "Found 7"
    let re = regex::Regex::new(r"(?i)(?:found|count:?\s*)\s*(\d+)|(\d+)\s+(?:functions?|matches?|occurrences?|items?|results?)").ok()?;

    for cap in re.captures_iter(text) {
        // Try first group (found/count)
        if let Some(m) = cap.get(1)
            && let Ok(n) = m.as_str().parse()
        {
            return Some(n);
        }
        // Try second group (number before word)
        if let Some(m) = cap.get(2)
            && let Ok(n) = m.as_str().parse()
        {
            return Some(n);
        }
    }

    None
}
