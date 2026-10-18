//! Selected story identifier and title.
use crate::RalphStoryInfo;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
pub(super) fn row(story: &RalphStoryInfo) -> Line<'_> {
    Line::from(vec![
        Span::styled("Story: ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            format!("{} — {}", story.id, story.title),
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
    ])
}
