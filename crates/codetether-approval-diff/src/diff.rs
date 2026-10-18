//! Syntax-aware unified-diff rendering for approval previews.

use ratatui::{style::Color, text::Line};

use crate::syntax::SyntaxPair;

/// Render a unified diff as terminal lines with separate old/new syntax state.
/// # Arguments
/// * `diff` - Unified diff or plain preview text.
/// # Returns
/// Styled owned lines, preserving headers, content and context order.
/// # Examples
/// ```
/// let lines = codetether_approval_diff::lines("plain preview");
/// assert_eq!(lines[0].to_string(), "plain preview");
/// ```
pub fn lines(diff: &str) -> Vec<Line<'static>> {
    let mut syntax = None;
    diff.lines()
        .map(|line| render_line(line, &mut syntax))
        .collect()
}

fn render_line(line: &str, syntax: &mut Option<SyntaxPair<'static>>) -> Line<'static> {
    if let Some(path) = line.strip_prefix("+++ ") {
        *syntax = SyntaxPair::for_path(path.strip_prefix("b/").unwrap_or(path));
        return crate::line::meta(line, Color::Cyan);
    }
    if line.starts_with("--- ") {
        return crate::line::meta(line, Color::Cyan);
    }
    if line.starts_with("@@") {
        return crate::line::meta(line, Color::Magenta);
    }
    match line.chars().next() {
        Some('+') => crate::code::code('+', &line[1..], syntax, true),
        Some('-') => crate::code::code('-', &line[1..], syntax, false),
        Some(' ') => crate::line::context(&line[1..], syntax),
        _ => Line::raw(line.to_string()),
    }
}
