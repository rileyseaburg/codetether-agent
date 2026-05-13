//! Text windowing and query term helpers.

/// Normalize a query into retrieval terms.
pub(super) fn terms(query: &str) -> Vec<String> {
    query
        .split(|c: char| !(c.is_alphanumeric() || c == '_'))
        .filter(|s| s.len() > 2)
        .map(str::to_lowercase)
        .collect()
}

/// Return a line-window around a center line.
pub(super) fn window(lines: &[&str], center: usize, radius: usize) -> (usize, usize, String) {
    let start = center.saturating_sub(radius);
    let end = (center + radius + 1).min(lines.len());
    (start + 1, end.max(start + 1), join(lines, start, end))
}

/// Build coarse fallback text windows.
pub(super) fn windows(lines: &[&str], size: usize, step: usize) -> Vec<(usize, usize, String)> {
    let mut out = Vec::new();
    let mut start = 0;
    while start < lines.len() {
        let end = (start + size).min(lines.len());
        out.push((start + 1, end, join(lines, start, end)));
        start += step.max(1);
    }
    out
}

fn join(lines: &[&str], start: usize, end: usize) -> String {
    lines[start..end].join("\n")
}
