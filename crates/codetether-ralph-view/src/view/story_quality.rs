//! Quality-check results for the selected story.
use crate::RalphStoryInfo;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
pub(super) fn row(story: &RalphStoryInfo) -> Line<'_> {
    Line::from({
        let mut spans = vec![Span::styled(
            "Quality: ",
            Style::default().fg(Color::DarkGray),
        )];
        if story.quality_checks.is_empty() {
            spans.push(Span::styled(
                "not run yet",
                Style::default().fg(Color::DarkGray),
            ));
        } else {
            for (name, passed) in &story.quality_checks {
                let (icon, color) = if *passed {
                    ("✓", Color::Green)
                } else {
                    ("✗", Color::Red)
                };
                spans.push(Span::styled(
                    format!("{}{} ", icon, name),
                    Style::default().fg(color),
                ));
            }
        }
        spans
    })
}
