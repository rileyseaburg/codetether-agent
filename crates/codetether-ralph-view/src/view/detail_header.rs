//! Ralph detail header presentation.
use crate::{RalphStoryInfo, RalphStoryStatus};
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, Paragraph};
pub(super) fn render(f: &mut Frame, story: &RalphStoryInfo, area: Rect) {
    // --- Header ---
    let (status_text, status_color) = match story.status {
        RalphStoryStatus::Pending => ("○ Pending", Color::DarkGray),
        RalphStoryStatus::Blocked => ("⊘ Blocked", Color::Yellow),
        RalphStoryStatus::Running => ("● Running", Color::Cyan),
        RalphStoryStatus::QualityCheck => ("◎ Quality Check", Color::Magenta),
        RalphStoryStatus::Passed => ("✓ Passed", Color::Green),
        RalphStoryStatus::Failed => ("✗ Failed", Color::Red),
    };

    let header_lines = vec![
        super::story_identity::row(story),
        super::story_status::row(story, status_text, status_color),
        super::story_dependencies::row(story),
        super::story_quality::row(story),
    ];

    let title = format!(" Story Detail: {} ", story.id);
    let header = Paragraph::new(header_lines).block(
        Block::default()
            .borders(Borders::ALL)
            .title(title)
            .border_style(Style::default().fg(status_color)),
    );
    f.render_widget(header, area);
}
