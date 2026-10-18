//! Ralph tool previews presentation.
use crate::RalphStoryInfo;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
pub(super) fn append<'a>(story: &'a RalphStoryInfo, i: usize, content_lines: &mut Vec<Line<'a>>) {
    let tc = &story.tool_call_history[i];
    if !tc.input_preview.is_empty() {
        content_lines.push(Line::from(vec![
            Span::raw("     "),
            Span::styled("in: ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                super::truncate::truncate_str(&tc.input_preview, 80),
                Style::default().fg(Color::White),
            ),
        ]));
    }
    if !tc.output_preview.is_empty() {
        content_lines.push(Line::from(vec![
            Span::raw("     "),
            Span::styled("out: ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                super::truncate::truncate_str(&tc.output_preview, 80),
                Style::default().fg(Color::White),
            ),
        ]));
    }
}
