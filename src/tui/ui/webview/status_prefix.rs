use ratatui::{
    style::{Color, Style},
    text::Span,
};

use crate::tui::app::state::{App, approval_queue};

pub fn build(app: &App) -> Vec<Span<'static>> {
    let state = if app.state.processing {
        (" RUN ", Color::Yellow)
    } else {
        (" READY ", Color::Green)
    };
    let mut spans = vec![
        badge(state.0, state.1),
        Span::raw(" "),
        Span::styled(
            format!(" {} ", app.state.status),
            Style::default().fg(Color::Gray),
        ),
        Span::styled(" Ctrl+B Layout ", Style::default().fg(Color::DarkGray)),
    ];
    if let Some(tool) = app.state.pending_tool_name.as_deref() {
        spans.push(badge(&format!(" TOOL {tool} "), Color::Yellow));
    }
    if let Some(request) = approval_queue::active() {
        let id = request.id.get(..8).unwrap_or(&request.id);
        let extra = approval_queue::len().saturating_sub(1);
        let extra = if extra == 0 {
            String::new()
        } else {
            format!(" +{extra}")
        };
        spans.push(badge(&format!(" APPROVE Ctrl+A {id}{extra} "), Color::Red));
    }
    spans.push(Span::raw("│ "));
    spans
}

fn badge(text: &str, color: Color) -> Span<'static> {
    Span::styled(text.to_string(), Style::default().fg(color))
}
