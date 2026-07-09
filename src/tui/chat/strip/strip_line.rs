//! Per-line stripping helpers for [`super::strip_tui_artifacts`].

/// Box-drawing characters produced by ratatui Block borders.
pub(super) const BOX_CHARS: &[char] =
    &['│', '┌', '┐', '└', '┘', '─', '├', '┤', '┬', '┴', '┼'];

/// Return `true` when every non-whitespace character on the line is a
/// box-drawing character — i.e. it is a pure border/separator line.
pub(super) fn is_border_line(line: &str) -> bool {
    let trimmed = line.trim();
    !trimmed.is_empty() && trimmed.chars().all(|c| BOX_CHARS.contains(&c))
}

/// Strip leading box-drawing border characters and trailing whitespace
/// from a single line, returning the inner content.
pub(super) fn strip_line(line: &str) -> &str {
    let s = line.trim_end();
    let s = s.trim_start_matches(|c: char| BOX_CHARS.contains(&c) || c == ' ');
    s.trim_end_matches(|c: char| BOX_CHARS.contains(&c) || c == ' ')
}
