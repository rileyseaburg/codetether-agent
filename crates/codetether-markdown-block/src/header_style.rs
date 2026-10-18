//! Header markers and emphasis colors for terminal Markdown.
use ratatui::{
    style::{Color, Modifier, Style},
    text::Span,
};

pub(crate) fn header_spans(hashes: usize, text: &str) -> Vec<Span<'static>> {
    let (marker, color) = match hashes {
        1 => ("▌ ", Color::Cyan),
        2 => ("▎ ", Color::Cyan),
        _ => ("› ", Color::Blue),
    };
    vec![
        Span::styled(marker, Style::default().fg(color)),
        Span::styled(
            text.to_string(),
            Style::default().fg(color).add_modifier(Modifier::BOLD),
        ),
    ]
}
