//! Help session 4 content and presentation.
use crate::HelpContext;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
pub(crate) fn append(lines: &mut Vec<Line<'static>>, context: &HelpContext<'_>) {
    lines.push(Line::from(vec![
        Span::styled("  Network:   ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            if context.allow_network { " ON" } else { " OFF" },
            if context.allow_network {
                Style::default().fg(Color::Green)
            } else {
                Style::default().fg(Color::Yellow)
            },
        ),
        Span::styled("  (/network)", Style::default().fg(Color::DarkGray)),
    ]));
}
