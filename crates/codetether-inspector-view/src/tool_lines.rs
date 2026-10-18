//! Recent-tool values and processing-state rows.
use crate::{
    row::row,
    tool_display::{success_display, tool_display},
};
use ratatui::{
    style::{Color, Modifier, Style},
    text::Line,
};
pub(super) fn lines(state: &crate::InspectorView<'_>) -> Vec<Line<'static>> {
    let label = Style::default().fg(Color::DarkGray);
    let mut lines = Vec::new();
    lines.push(Line::styled(
        "Last Tool Call",
        Style::default().add_modifier(Modifier::BOLD),
    ));

    let (tool, tool_color) = tool_display(state);
    lines.push(row("Tool:", tool, tool_color, label));

    let latency = state
        .last_tool_latency_ms
        .map(|ms| format!("{ms}ms"))
        .unwrap_or_else(|| "—".into());
    lines.push(row("Latency:", &latency, Color::White, label));

    let (status_text, status_color) = success_display(state);
    lines.push(row("Status:", status_text, status_color, label));

    lines.push(row(
        "Messages:",
        &state.message_count.to_string(),
        Color::White,
        label,
    ));

    let (lbl, clr) = if state.processing {
        ("Processing", Color::Yellow)
    } else {
        ("Idle", Color::Green)
    };
    lines.push(row("State:", lbl, clr, label));

    lines
}
