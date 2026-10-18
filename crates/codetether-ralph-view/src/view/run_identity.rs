//! Project and feature names in the run header.
use crate::RalphViewState;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
pub(super) fn row(state: &RalphViewState) -> Line<'_> {
    Line::from(vec![
        Span::styled("Project: ", Style::default().fg(Color::DarkGray)),
        Span::styled(&state.project, Style::default().fg(Color::White)),
        Span::raw("  "),
        Span::styled("Feature: ", Style::default().fg(Color::DarkGray)),
        Span::styled(&state.feature, Style::default().fg(Color::White)),
    ])
}
