//! Styling of code, context, and metadata lines in approval diffs.

use super::approval_diff_syntax::SyntaxPair;
use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
};

pub(super) fn code(
    prefix: char,
    source: &str,
    syntax: &mut Option<SyntaxPair<'static>>,
    added: bool,
) -> Line<'static> {
    let color = if added { Color::Green } else { Color::Red };
    let mut spans = vec![Span::styled(
        prefix.to_string(),
        Style::default().fg(color).add_modifier(Modifier::BOLD),
    )];
    let highlighted = syntax.as_mut().map(|pair| {
        if added {
            pair.addition(source)
        } else {
            pair.deletion(source)
        }
    });
    spans.extend(
        highlighted
            .filter(|value| !value.is_empty())
            .unwrap_or_else(|| vec![Span::raw(source.to_string())]),
    );
    let background = if added {
        Color::Rgb(20, 52, 34)
    } else {
        Color::Rgb(62, 28, 32)
    };
    Line::from(spans).style(Style::default().bg(background))
}

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
