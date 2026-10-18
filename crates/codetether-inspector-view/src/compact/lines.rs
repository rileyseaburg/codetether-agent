use ratatui::prelude::*;

use super::View;

/// Build compact inspector lines from borrowed session state.
///
/// # Arguments
/// * `state` - Session, tool and agent values to display.
/// # Returns
/// Owned lines with established field fallbacks and shortcut labels.
/// # Examples
/// ```
/// let lines = codetether_inspector_view::compact::build(&Default::default());
/// assert!(!lines.is_empty());
/// ```
pub fn build(state: &View<'_>) -> Vec<Line<'static>> {
    let status = state.processing.then_some("Processing").unwrap_or("Idle");
    let tool = state.pending_tool.or(state.last_tool).unwrap_or("none");
    let model = state.model.unwrap_or("auto");
    let agent = state.active_agent.unwrap_or("build");
    let active = state.active_agent.unwrap_or("None");
    let registered = state.registered;
    let protocol = format!("{registered} registered");
    let context = format!(
        "{}/{}",
        state.context_used.unwrap_or(0),
        state.context_budget.unwrap_or(0)
    );
    let sync = state.sync.unwrap_or("disabled");
    vec![
        row("Status", status),
        row("Tool", tool),
        Line::from(""),
        Line::styled("Session", Style::default().fg(Color::DarkGray)),
        row("ID", state.session_id.unwrap_or("#new")),
        row("Model", model),
        row("Agent", agent),
        row("Messages", &state.message_count.to_string()),
        row("Context", &context),
        row("Protocol", &protocol),
        row("Remote sync", sync),
        Line::from(""),
        Line::styled("Sub-agents", Style::default().fg(Color::DarkGray)),
        row("Active", active),
        row("Count", &state.agent_count.to_string()),
        Line::from(""),
        Line::styled("Shortcuts", Style::default().fg(Color::DarkGray)),
        Line::from("F3      Inspector"),
        Line::from("Ctrl+B  Layout"),
        Line::from("Ctrl+Y  Copy"),
        Line::from("Ctrl+S  Swarm"),
        Line::from("?       Help"),
    ]
}

fn row(label: &str, value: &str) -> Line<'static> {
    Line::from(format!("{label}: {value}"))
}
