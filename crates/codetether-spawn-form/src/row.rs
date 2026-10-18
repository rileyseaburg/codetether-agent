//! Single field display and focus marker.
//! Form state and terminal presentation require no application runtime.

use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};

pub(crate) fn field_row(label: &str, value: &str, active: bool) -> Line<'static> {
    let style = if active {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Gray)
    };
    let cursor = if active { "█" } else { "" };
    Line::from(vec![
        Span::styled(format!("  {label:<14}: "), Style::default().fg(Color::Cyan)),
        Span::styled(format!("{value}{cursor}"), style),
    ])
}
