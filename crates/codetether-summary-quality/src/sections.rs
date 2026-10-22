//! Preserve Markdown summary sections independently when enforcing size limits.
pub(super) fn split(text: &str) -> Vec<&str> {
    let mut starts = vec![0];
    let mut offset = 0;
    let mut fenced = false;
    for line in text.split_inclusive('\n') {
        let trimmed = line.trim_start();
        if trimmed.starts_with("```") || trimmed.starts_with("~~~") {
            fenced = !fenced;
        } else if !fenced && offset > 0 && trimmed.starts_with('#') {
            starts.push(offset);
        }
        offset += line.len();
    }
    starts.push(text.len());
    starts
        .windows(2)
        .map(|range| &text[range[0]..range[1]])
        .collect()
}
