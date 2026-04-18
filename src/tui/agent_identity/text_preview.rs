//! Generic text preview: size-limited excerpt for terminal display.

/// Build a stable, size-limited text preview.
///
/// Returns `(preview_text, was_truncated)`.
#[allow(dead_code)]
pub fn build_text_preview(text: &str, max_lines: usize, max_bytes: usize) -> (String, bool) {
    if max_lines == 0 || max_bytes == 0 || text.is_empty() {
        return (String::new(), !text.is_empty());
    }
    let mut out = String::new();
    let mut truncated = false;
    let mut remaining = max_bytes;
    let mut iter = text.lines();
    for i in 0..max_lines {
        let Some(line) = iter.next() else { break };
        if i > 0 {
            if remaining == 0 { truncated = true; break; }
            out.push('\n');
            remaining = remaining.saturating_sub(1);
        }
        if remaining == 0 { truncated = true; break; }
        if line.len() <= remaining {
            out.push_str(line);
            remaining = remaining.saturating_sub(line.len());
        } else {
            let mut end = remaining;
            while end > 0 && !line.is_char_boundary(end) { end -= 1; }
            out.push_str(&line[..end]);
            truncated = true;
            break;
        }
    }
    if !truncated && iter.next().is_some() { truncated = true; }
    (out, truncated)
}
