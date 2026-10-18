//! Dependencies of the selected story.
use crate::RalphStoryInfo;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
pub(super) fn row(story: &RalphStoryInfo) -> Line<'_> {
    Line::from(vec![
        Span::styled("Deps: ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            if story.depends_on.is_empty() {
                "none".to_string()
            } else {
                story.depends_on.join(", ")
            },
            Style::default().fg(Color::DarkGray),
        ),
    ])
}
