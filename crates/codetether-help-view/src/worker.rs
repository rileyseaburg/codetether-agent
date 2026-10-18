//! Help worker content and presentation.
use crate::HelpContext;
use crate::blank::blank;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
pub(crate) fn append(lines: &mut Vec<Line<'static>>, context: &HelpContext<'_>) {
    // Worker info if connected
    if let Some(wid) = context.worker_id {
        lines.push(Line::from(vec![
            Span::styled("  Worker:    ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                (*context.worker_name)
                    .clone()
                    .unwrap_or_else(|| wid.clone()),
                Style::default().fg(Color::Green),
            ),
            if context.a2a_connected {
                Span::styled("  (A2A connected)", Style::default().fg(Color::Green))
            } else {
                Span::styled("  (disconnected)", Style::default().fg(Color::Red))
            },
        ]));
    }
    lines.push(blank());
}
