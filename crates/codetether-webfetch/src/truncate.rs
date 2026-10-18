//! truncate for web content fetching.
pub(super) fn truncate(content: String, max_chars: usize) -> (String, bool) {
    let mut out = content;
    let truncated = if out.chars().count() > max_chars {
        // Keep head + tail so navigation/footer and disclaimers can still be spotted.
        let head_chars = (max_chars as f64 * 0.70) as usize;
        let tail_chars = (max_chars as f64 * 0.20) as usize;

        let head: String = out.chars().take(head_chars).collect();
        let tail: String = out
            .chars()
            .rev()
            .take(tail_chars)
            .collect::<String>()
            .chars()
            .rev()
            .collect();

        let total_chars = out.chars().count();
        out = format!(
            "{}\n\n[... truncated {} chars (max_chars={}) ...]\n\n{}",
            head,
            total_chars.saturating_sub(head_chars + tail_chars),
            max_chars,
            tail
        );
        true
    } else {
        false
    };

    (out, truncated)
}
