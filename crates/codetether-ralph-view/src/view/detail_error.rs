//! Ralph detail error presentation.
use crate::RalphStoryInfo;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
pub(super) fn append<'a>(story: &'a RalphStoryInfo, content_lines: &mut Vec<Line<'a>>) {
    // Error
    if let Some(ref err) = story.error {
        content_lines.push(Line::from(Span::styled(
            "─── Error ───",
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        )));
        content_lines.push(Line::from(""));
        for line in err.lines() {
            content_lines.push(Line::from(Span::styled(
                line,
                Style::default().fg(Color::Red),
            )));
        }
        content_lines.push(Line::from(""));
    }
}
