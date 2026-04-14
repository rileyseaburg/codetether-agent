use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
};

use crate::tui::app::state::AppState;

pub fn render_tool_history(f: &mut ratatui::Frame, state: &AppState, area: Rect) {
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
        &state.messages.len().to_string(),
        Color::White,
        label,
    ));

    let (lbl, clr) = if state.processing {
        ("Processing", Color::Yellow)
    } else {
        ("Idle", Color::Green)
    };
    lines.push(row("State:", lbl, clr, label));

    f.render_widget(
        Paragraph::new(lines)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Tool History "),
            )
            .wrap(Wrap { trim: true }),
        area,
    );
}

fn tool_display(state: &AppState) -> (String, Color) {
    match &state.last_tool_name {
        Some(n) => (n.clone(), Color::Cyan),
        None => ("none".into(), Color::DarkGray),
    }
}

fn success_display(state: &AppState) -> (&str, Color) {
    match state.last_tool_success {
        Some(true) => ("OK", Color::Green),
        Some(false) => ("FAIL", Color::Red),
        None => ("—", Color::DarkGray),
    }
}

fn row(key: &str, val: impl AsRef<str>, color: Color, label: Style) -> Line<'static> {
    Line::from(vec![
        Span::styled(format!("{key} "), label),
        Span::styled(val.as_ref().to_string(), Style::default().fg(color)),
    ])
}
