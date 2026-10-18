//! Help agents content and presentation.
use crate::blank::blank;
use crate::heading::heading;
use crate::key_row::key_row;
use crate::separator::separator;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
pub(crate) fn append(lines: &mut Vec<Line<'static>>) {
    lines.push(heading("SWARM / RALPH"));
    lines.push(separator());
    lines.push(key_row("Up / Down", "Select sub-agent / story"));
    lines.push(key_row("Enter", "Open detail view"));
    lines.push(key_row("PgUp / PgDn", "Scroll detail content"));
    lines.push(key_row("Esc", "Exit detail"));
    lines.push(blank());

    lines.push(separator());
    lines.push(Line::from(Span::styled(
        "  Press Esc to return to chat",
        Style::default().fg(Color::DarkGray),
    )));
    lines.push(blank());
}
