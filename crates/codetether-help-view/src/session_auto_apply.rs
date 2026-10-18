//! Help session 3 content and presentation.
use crate::HelpContext;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
pub(crate) fn append(lines: &mut Vec<Line<'static>>, context: &HelpContext<'_>) {
    lines.push(Line::from(vec![
        Span::styled("  Auto-apply:", Style::default().fg(Color::DarkGray)),
        Span::styled(
            if context.auto_apply_edits {
                " ON"
            } else {
                " OFF"
            },
            if context.auto_apply_edits {
                Style::default().fg(Color::Green)
            } else {
                Style::default().fg(Color::Yellow)
            },
        ),
        Span::styled(
            "  (/autoapply or /aa)",
            Style::default().fg(Color::DarkGray),
        ),
    ]));
}
