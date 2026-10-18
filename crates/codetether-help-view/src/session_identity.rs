//! Help session 1 content and presentation.
use crate::HelpContext;
use crate::heading::heading;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
pub(crate) fn append(
    lines: &mut Vec<Line<'static>>,
    context: &HelpContext<'_>,
    session_label: String,
) {
    // ── Session info ──
    lines.push(heading("SESSION"));
    lines.push(Line::from(vec![
        Span::styled("  Session:   ", Style::default().fg(Color::DarkGray)),
        Span::styled(session_label, Style::default().fg(Color::White)),
    ]));
    lines.push(Line::from(vec![
        Span::styled("  Directory: ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            (*context.cwd_display).clone(),
            Style::default().fg(Color::White),
        ),
    ]));
}
