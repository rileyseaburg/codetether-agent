use ratatui::prelude::*;

use crate::tui::app::state::App;

pub fn build(app: &App) -> Vec<Line<'static>> {
    let state = &app.state;
    let status = state.processing.then_some("Processing").unwrap_or("Idle");
    let tool = state
        .pending_tool_name
        .as_deref()
        .or(state.last_tool_name.as_deref())
        .unwrap_or("none");
    let model = state.last_completion_model.as_deref().unwrap_or("auto");
    let agent = state.active_spawned_agent.as_deref().unwrap_or("build");
    let active = state.active_spawned_agent.as_deref().unwrap_or("None");
    let protocol = format!("{} registered", state.worker_bridge_registered_agents.len());
    let context = format!(
        "{}/{}",
        state.context_used.unwrap_or(0),
        state.context_budget.unwrap_or(0)
    );
    let sync = state.chat_sync_status.as_deref().unwrap_or("disabled");
    vec![
        row("Status", status),
        row("Tool", tool),
        Line::from(""),
        Line::styled("Session", Style::default().fg(Color::DarkGray)),
        row("ID", state.session_id.as_deref().unwrap_or("#new")),
        row("Model", model),
        row("Agent", agent),
        row("Messages", &state.messages.len().to_string()),
        row("Context", &context),
        row("Protocol", &protocol),
        row("Remote sync", sync),
        Line::from(""),
        Line::styled("Sub-agents", Style::default().fg(Color::DarkGray)),
        row("Active", active),
        row("Count", &state.spawned_agents.len().to_string()),
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
