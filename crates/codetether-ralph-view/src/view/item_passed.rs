//! Ralph item passed presentation.
use crate::{RalphStoryInfo, RalphStoryStatus};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Span;
pub(super) fn append<'a>(story: &'a RalphStoryInfo, spans: &mut Vec<Span<'a>>) {
    // Show passed quality checks for completed stories
    if story.status == RalphStoryStatus::Passed && !story.quality_checks.is_empty() {
        let all_passed = story.quality_checks.iter().all(|(_, p)| *p);
        if all_passed {
            spans.push(Span::styled(
                " ✓QC",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::DIM),
            ));
        }
    }
}
