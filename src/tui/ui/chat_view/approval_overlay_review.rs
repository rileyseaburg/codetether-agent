//! Reviewer verdict panel for the approval popup (advise mode).

mod style;

use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    text::Line,
    widgets::{Block, Borders, Paragraph},
};

use crate::review::ReviewVerdict;

/// Rows needed: 0 when no review was requested, 3 while running, else
/// reason + findings, bounded so the diff keeps most of the popup.
pub(super) fn height(review: Option<&Option<ReviewVerdict>>) -> u16 {
    match review {
        None => 0,
        Some(None) => 3,
        Some(Some(verdict)) => (verdict.findings.len() as u16 + 3).min(7),
    }
}

pub(super) fn render(f: &mut Frame, area: Rect, review: Option<&Option<ReviewVerdict>>) {
    let Some(review) = review else {
        return;
    };
    let (title, color, lines) = match review {
        None => (
            "Reviewer · inspecting…".to_string(),
            Color::Yellow,
            vec![Line::raw("reading touched files and running read-only checks")],
        ),
        Some(verdict) => (
            format!("Reviewer · {}", verdict.outcome.label()),
            style::color_for(verdict.outcome),
            style::lines_for(verdict),
        ),
    };
    f.render_widget(
        Paragraph::new(lines)
            .style(Style::default().fg(color))
            .block(Block::default().borders(Borders::ALL).title(title)),
        area,
    );
}
