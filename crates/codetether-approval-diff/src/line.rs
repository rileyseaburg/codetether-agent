//! Styling of code, context, and metadata lines in approval diffs.

use crate::syntax::SyntaxPair;
use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
};

pub(super) fn context(source: &str, syntax: &mut Option<SyntaxPair<'static>>) -> Line<'static> {
    let spans = syntax
        .as_mut()
        .map(|pair| pair.context(source))
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| vec![Span::raw(source.to_string())]);
    Line::from(spans)
}

pub(super) fn meta(text: &str, color: Color) -> Line<'static> {
    Line::from(Span::styled(
        text.to_string(),
        Style::default().fg(color).add_modifier(Modifier::BOLD),
    ))
}
