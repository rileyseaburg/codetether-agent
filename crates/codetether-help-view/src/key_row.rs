//! Help key row content and presentation.
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
pub(crate) fn key_row(key: &str, desc: &str) -> Line<'static> {
    Line::from(vec![
        Span::styled(
            format!("  {key:<18}"),
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(desc.to_string()),
    ])
}
