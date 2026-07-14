//! Detail-row assembly for children spawned through the agent tool.

use ratatui::style::Stylize;
use ratatui::text::Line;

use crate::tui::app::state::AppState;

pub(super) fn lines(state: &AppState, name: &str) -> Vec<Line<'static>> {
    let Some(parent_id) = state.session_id.as_deref() else {
        return missing(name);
    };
    let Some(agent) = crate::tool::agent::bridge::find_agent_tool_agent_for_parent(name, parent_id)
    else {
        return missing(name);
    };
    let messages = crate::tool::agent::bridge::agent_tool_transcript_for_parent(name, parent_id)
        .unwrap_or_default();
    let parent = agent.parent.as_deref().unwrap_or("main");
    let model = agent.model_id.as_deref().unwrap_or("default model");
    let status = if agent.is_processing {
        "working"
    } else {
        "idle"
    };
    let mut rows =
        super::subagent_detail_metadata::lines(name, parent, status, model, &agent.instructions);
    super::subagent_message_lines::append(&mut rows, &messages);
    rows
}

fn missing(name: &str) -> Vec<Line<'static>> {
    vec![Line::from(
        format!("Agent @{name} is no longer available.").red(),
    )]
}
