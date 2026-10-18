//! Iteration and story outcome counts in the run header.
use crate::RalphViewState;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
pub(super) fn row(
    state: &RalphViewState,
    pending: usize,
    running: usize,
    passed: usize,
    failed: usize,
    total: usize,
) -> Line<'_> {
    Line::from(vec![
        Span::styled(
            format!(
                "Iteration: {}/{}",
                state.current_iteration, state.max_iterations
            ),
            Style::default().fg(Color::Cyan),
        ),
        Span::raw("  "),
        Span::styled(format!("⏳{}", pending), Style::default().fg(Color::Yellow)),
        Span::raw(" "),
        Span::styled(format!("⚡{}", running), Style::default().fg(Color::Cyan)),
        Span::raw(" "),
        Span::styled(format!("✓{}", passed), Style::default().fg(Color::Green)),
        Span::raw(" "),
        Span::styled(format!("✗{}", failed), Style::default().fg(Color::Red)),
        Span::raw(" "),
        Span::styled(format!("/{}", total), Style::default().fg(Color::DarkGray)),
    ])
}
