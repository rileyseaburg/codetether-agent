//! Ralph tool entry presentation.
use crate::RalphStoryInfo;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
pub(super) fn append<'a>(story: &'a RalphStoryInfo, i: usize, content_lines: &mut Vec<Line<'a>>) {
    let tc = &story.tool_call_history[i];

    let icon = if tc.success { "✓" } else { "✗" };
    let icon_color = if tc.success { Color::Green } else { Color::Red };
    content_lines.push(Line::from(vec![
        Span::styled(format!(" {icon} "), Style::default().fg(icon_color)),
        Span::styled(format!("#{} ", i + 1), Style::default().fg(Color::DarkGray)),
        Span::styled(
            &tc.tool_name,
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ),
    ]));
    super::tool_previews::append(story, i, content_lines);
}
