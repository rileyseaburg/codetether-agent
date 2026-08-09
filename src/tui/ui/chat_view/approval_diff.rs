//! Syntax-aware unified-diff rendering for approval previews.

use ratatui::{style::Color, text::Line};

use super::approval_diff_syntax::SyntaxPair;

pub(super) fn lines(diff: &str) -> Vec<Line<'static>> {
    let mut syntax = None;
    diff.lines()
        .map(|line| render_line(line, &mut syntax))
        .collect()
}

fn render_line(line: &str, syntax: &mut Option<SyntaxPair<'static>>) -> Line<'static> {
    if let Some(path) = line.strip_prefix("+++ ") {
        *syntax = SyntaxPair::for_path(path.strip_prefix("b/").unwrap_or(path));
        return super::approval_diff_line::meta(line, Color::Cyan);
    }
    if line.starts_with("--- ") {
        return super::approval_diff_line::meta(line, Color::Cyan);
    }
    if line.starts_with("@@") {
        return super::approval_diff_line::meta(line, Color::Magenta);
    }
    match line.chars().next() {
        Some('+') => super::approval_diff_line::code('+', &line[1..], syntax, true),
        Some('-') => super::approval_diff_line::code('-', &line[1..], syntax, false),
        Some(' ') => super::approval_diff_line::context(&line[1..], syntax),
        _ => Line::raw(line.to_string()),
    }
}
