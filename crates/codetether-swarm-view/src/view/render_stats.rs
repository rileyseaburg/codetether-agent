//! Swarm render stats presentation.
use crate::SwarmViewState;
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
pub(super) fn render_stats(f: &mut Frame, state: &SwarmViewState, area: Rect) {
    let content = if let Some(ref stats) = state.stats {
        super::stats_line::line(stats)
    } else if let Some(ref err) = state.error {
        Line::from(vec![Span::styled(
            format!("Error: {}", err),
            Style::default().fg(Color::Red),
        )])
    } else {
        Line::from(vec![Span::styled(
            "Executing...",
            Style::default().fg(Color::DarkGray),
        )])
    };

    let paragraph = Paragraph::new(content)
        .block(Block::default().borders(Borders::ALL).title(" Stats "))
        .wrap(Wrap { trim: true });

    f.render_widget(paragraph, area);
}
