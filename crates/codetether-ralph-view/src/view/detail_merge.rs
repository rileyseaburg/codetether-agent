//! Ralph detail merge presentation.
use crate::RalphStoryInfo;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
pub(super) fn append<'a>(story: &'a RalphStoryInfo, content_lines: &mut Vec<Line<'a>>) {
    // Merge summary
    if let Some(ref summary) = story.merge_summary {
        content_lines.push(Line::from(Span::styled(
            "─── Merge ───",
            Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::BOLD),
        )));
        content_lines.push(Line::from(""));
        content_lines.push(Line::from(Span::styled(
            summary.as_str(),
            Style::default().fg(Color::White),
        )));
        content_lines.push(Line::from(""));
    }
}
