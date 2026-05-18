//! Render a `ToolCall` row: timestamp + icon/name header + smart preview.

use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};

use super::arg_preview::smart_arg_preview;
use super::diff_render::{push_create_preview, push_edit_diff};
use super::icons::tool_icon_and_color;

pub(super) fn render_tool_call(
    body_lines: &mut Vec<Line<'static>>,
    timestamp: &str,
    name: &str,
    arguments: &str,
    preview_width: usize,
) {
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
    let preview = smart_arg_preview(name, arguments);
    let body = if preview.is_empty() {
        "(no arguments)".to_string()
    } else {
        preview
    };
    body_lines.push(Line::from(vec![
        Span::styled("│   ", Style::default().fg(Color::DarkGray).dim()),
        Span::styled(body, Style::default().fg(Color::DarkGray).dim()),
    ]));

    match name {
        "replace_string_in_file" | "edit_file" => {
            push_edit_diff(body_lines, arguments, preview_width)
        }
        "create_file" | "write_file" => push_create_preview(body_lines, arguments, preview_width),
        _ => {}
    }
}
