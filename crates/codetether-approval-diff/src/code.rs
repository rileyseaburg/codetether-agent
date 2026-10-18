//! Addition/deletion highlighting with background and sign styles.
use crate::syntax::SyntaxPair;
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
