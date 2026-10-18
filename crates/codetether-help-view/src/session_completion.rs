//! Help session 5 content and presentation.
use crate::HelpContext;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
pub(crate) fn append(lines: &mut Vec<Line<'static>>, context: &HelpContext<'_>) {
    lines.push(Line::from(vec![
        Span::styled("  Autocomplete:", Style::default().fg(Color::DarkGray)),
        Span::styled(
            if context.slash_autocomplete {
                " ON"
            } else {
                " OFF"
            },
            if context.slash_autocomplete {
                Style::default().fg(Color::Green)
            } else {
                Style::default().fg(Color::Yellow)
            },
        ),
        Span::styled("  (/autocomplete)", Style::default().fg(Color::DarkGray)),
    ]));
}
