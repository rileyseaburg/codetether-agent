//! Ralph story item presentation.
use crate::{RalphStoryInfo, RalphStoryStatus};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::ListItem;
pub(super) fn item(story: &RalphStoryInfo) -> ListItem<'_> {
    let (icon, color) = match story.status {
        RalphStoryStatus::Pending => ("○", Color::DarkGray),
        RalphStoryStatus::Blocked => ("⊘", Color::Yellow),
        RalphStoryStatus::Running => ("●", Color::Cyan),
        RalphStoryStatus::QualityCheck => ("◎", Color::Magenta),
        RalphStoryStatus::Passed => ("✓", Color::Green),
        RalphStoryStatus::Failed => ("✗", Color::Red),
    };

    let mut spans = vec![
        Span::styled(format!("{} ", icon), Style::default().fg(color)),
        Span::styled(
            format!("[{}] ", story.id),
            Style::default().fg(Color::DarkGray),
        ),
        Span::styled(&story.title, Style::default().fg(Color::White)),
    ];

    super::item_running::append(story, &mut spans);
    super::item_quality::append(story, &mut spans);
    super::item_passed::append(story, &mut spans);
    ListItem::new(Line::from(spans))
}
