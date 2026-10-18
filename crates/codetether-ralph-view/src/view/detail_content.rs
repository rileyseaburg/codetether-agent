//! Tool history, conversation and results in the scrollable detail body.
use crate::RalphStoryInfo;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
};
pub(super) fn render(f: &mut Frame, story: &RalphStoryInfo, detail_scroll: usize, area: Rect) {
    let mut content_lines: Vec<Line> = Vec::new();

    super::detail_tools::append(story, &mut content_lines);
    super::detail_messages::append(story, &mut content_lines);
    super::detail_output::append(story, &mut content_lines);
    super::detail_merge::append(story, &mut content_lines);
    super::detail_error::append(story, &mut content_lines);
    // If nothing to show
    if content_lines.is_empty() {
        content_lines.push(Line::from(Span::styled(
            "  Waiting for agent activity...",
            Style::default().fg(Color::DarkGray),
        )));
    }

    let content = Paragraph::new(content_lines)
        .block(Block::default().borders(Borders::ALL))
        .wrap(Wrap { trim: false })
        .scroll((detail_scroll as u16, 0));
    f.render_widget(content, area);
}
