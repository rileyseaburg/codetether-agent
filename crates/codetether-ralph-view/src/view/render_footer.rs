//! Ralph render footer presentation.
use crate::RalphViewState;
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
pub(super) fn render_footer(f: &mut Frame, state: &RalphViewState, area: Rect) {
    let content = if let Some(ref status) = state.final_status {
        Line::from(vec![
            Span::styled("Status: ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                status.as_str(),
                Style::default().fg(if status.contains("Completed") {
                    Color::Green
                } else {
                    Color::Yellow
                }),
            ),
        ])
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
        .block(Block::default().borders(Borders::ALL).title(" Status "))
        .wrap(Wrap { trim: true });
    f.render_widget(paragraph, area);
}
