//! Ralph message entry presentation.
use crate::RalphStoryInfo;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
pub(super) fn append<'a>(story: &'a RalphStoryInfo, i: usize, content_lines: &mut Vec<Line<'a>>) {
    let msg = &story.messages[i];

    let (role_color, role_label) = match msg.role.as_str() {
        "user" => (Color::White, "USER"),
        "assistant" => (Color::Cyan, "ASST"),
        "tool" => (Color::Yellow, "TOOL"),
        "system" | "developer" => (Color::DarkGray, "SYS "),
        _ => (Color::White, "    "),
    };
    content_lines.push(Line::from(vec![Span::styled(
        format!(" [{role_label}] "),
        Style::default().fg(role_color).add_modifier(Modifier::BOLD),
    )]));
    for line in msg.content.lines().take(10) {
        content_lines.push(Line::from(vec![
            Span::raw("   "),
            Span::styled(line, Style::default().fg(Color::White)),
        ]));
    }
    if msg.content.lines().count() > 10 {
        content_lines.push(Line::from(vec![
            Span::raw("   "),
            Span::styled("... (truncated)", Style::default().fg(Color::DarkGray)),
        ]));
    }
    content_lines.push(Line::from(""));
}
