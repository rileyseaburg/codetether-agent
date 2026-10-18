//! Title and workspace context for the inspector.
use crate::LatencyView;
use ratatui::text::Line;
pub(crate) fn lines(view: &LatencyView<'_>) -> Vec<Line<'static>> {
    let lines = vec![
        Line::from("Latency Inspector"),
        Line::from(""),
        Line::from("Process-local timing from the current TUI runtime."),
        Line::from("TTFT/TTL measure Enter to first/last assistant text observed in the TUI."),
        Line::from(format!("Workspace: {}", view.cwd_display)),
    ];

    lines
}
