//! Convert syntax highlighter foregrounds to owned terminal spans.
use ratatui::{
    style::{Color, Style},
    text::Span,
};
use syntect::easy::HighlightLines;

pub(crate) fn render(highlighter: &mut HighlightLines<'_>, code: &str) -> Vec<Span<'static>> {
    highlighter
        .highlight_line(code, crate::assets::syntaxes())
        .unwrap_or_default()
        .into_iter()
        .map(|(style, text)| {
            Span::styled(
                text.to_string(),
                Style::default().fg(Color::Rgb(
                    style.foreground.r,
                    style.foreground.g,
                    style.foreground.b,
                )),
            )
        })
        .collect()
}
