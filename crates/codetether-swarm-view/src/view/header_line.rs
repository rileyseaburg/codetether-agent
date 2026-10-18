//! Swarm header line presentation.
use crate::SwarmViewState;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
pub(super) fn line(
    state: &SwarmViewState,
    pending: usize,
    running: usize,
    completed: usize,
    failed: usize,
    total: usize,
) -> Line<'static> {
    let status_line = Line::from(vec![
        Span::styled("Task: ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            super::truncate::truncate_str(&state.task, 47),
            Style::default().fg(Color::White),
        ),
        Span::raw("  "),
        Span::styled(format!("⏳{}", pending), Style::default().fg(Color::Yellow)),
        Span::raw(" "),
        Span::styled(format!("⚡{}", running), Style::default().fg(Color::Cyan)),
        Span::raw(" "),
        Span::styled(format!("✓{}", completed), Style::default().fg(Color::Green)),
        Span::raw(" "),
        Span::styled(format!("✗{}", failed), Style::default().fg(Color::Red)),
        Span::raw(" "),
        Span::styled(format!("/{}", total), Style::default().fg(Color::DarkGray)),
    ]);

    status_line
}
