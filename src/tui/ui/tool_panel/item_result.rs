//! Render a `ToolResult` row: timestamp + status icon + output preview.

use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};

use super::preview::push_preview_lines;

#[allow(clippy::too_many_arguments)]
pub(super) fn render_tool_result(
    body_lines: &mut Vec<Line<'static>>,
    timestamp: &str,
    name: &str,
    output: &str,
    success: bool,
    duration_ms: Option<u64>,
    preview_width: usize,
) {
    let (icon, color, status) = if success {
        ("✅ ", Color::Green, "success")
    } else {
        ("❌ ", Color::Red, "error")
    };
    let duration_label = duration_ms
        .map(|ms| format!(" • {ms}ms"))
        .unwrap_or_default();
    body_lines.push(Line::from(vec![
        Span::styled("│ ", Style::default().fg(Color::DarkGray).dim()),
        Span::styled(
            format!("[{timestamp}] "),
            Style::default().fg(Color::DarkGray).dim(),
        ),
        Span::styled(icon, Style::default().fg(color).bold()),
        Span::styled(
            format!("{name} • {status}{duration_label}"),
            Style::default().fg(color).bold(),
        ),
    ]));
    push_preview_lines(
        body_lines,
        output,
        preview_width,
        Style::default().fg(color).dim(),
        "(empty output)",
    );
}
