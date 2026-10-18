//! Missing tool-name and outcome display values.
use ratatui::style::Color;

pub(super) fn tool_display(state: &crate::InspectorView<'_>) -> (String, Color) {
    match state.last_tool_name {
        Some(n) => (n.to_string(), Color::Cyan),
        None => ("none".into(), Color::DarkGray),
    }
}

pub(super) fn success_display(state: &crate::InspectorView<'_>) -> (&'static str, Color) {
    match state.last_tool_success {
        Some(true) => ("OK", Color::Green),
        Some(false) => ("FAIL", Color::Red),
        None => ("—", Color::DarkGray),
    }
}
