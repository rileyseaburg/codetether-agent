//! Ralph item running presentation.
use crate::{RalphStoryInfo, RalphStoryStatus};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Span;
pub(super) fn append<'a>(story: &'a RalphStoryInfo, spans: &mut Vec<Span<'a>>) {
    // Show current tool for running stories
    if story.status == RalphStoryStatus::Running {
        if let Some(ref tool) = story.current_tool {
            spans.push(Span::styled(
                format!(" → {}", tool),
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::DIM),
            ));
        }
        if story.steps > 0 {
            spans.push(Span::styled(
                format!(" (step {})", story.steps),
                Style::default().fg(Color::DarkGray),
            ));
        }
    }
}
