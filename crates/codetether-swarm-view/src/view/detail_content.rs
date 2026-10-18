//! Swarm detail content presentation.
use crate::SubTaskInfo;
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
pub(super) fn render(f: &mut Frame, task: &SubTaskInfo, detail_scroll: usize, area: Rect) {
    let mut content_lines: Vec<Line> = Vec::new();

    super::detail_tools::append(task, &mut content_lines);
    super::detail_messages::append(task, &mut content_lines);
    super::detail_output::append(task, &mut content_lines);
    super::detail_error::append(task, &mut content_lines);
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
