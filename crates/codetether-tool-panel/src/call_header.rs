//! Tool-call timestamp and icon/name header row.
use super::icons::tool_icon_and_color;
use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};
pub(super) fn push(body_lines: &mut Vec<Line<'static>>, timestamp: &str, name: &str) {
    let (icon, color) = tool_icon_and_color(name);
    body_lines.push(Line::from(vec![
        Span::styled("│ ", Style::default().fg(Color::DarkGray).dim()),
        Span::styled(
            format!("[{timestamp}] "),
            Style::default().fg(Color::DarkGray).dim(),
        ),
        Span::styled(format!("{icon} "), Style::default().fg(color).bold()),
        Span::styled(name.to_string(), Style::default().fg(color).bold()),
    ]));
}
