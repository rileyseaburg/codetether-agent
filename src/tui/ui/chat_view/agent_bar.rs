//! Header tab bar showing the spawned-agent tree (main + sibling agents).
//!
//! Merges both registries: TUI `/spawn` agents and agent-tool-spawned agents
//! (issue #295 / #297 Part A).

#[path = "agent_bar_presence.rs"]
pub(super) mod presence;
#[path = "agent_bar_swarm.rs"]
mod swarm;

use ratatui::{Frame, text::Line, widgets::Paragraph};

use super::agent_bar_tool::push_tool_agents;
use super::agent_bar_tui::push_tui_agents;
use super::agent_tab::{AgentTabMeta, agent_tab};
use crate::tui::app::state::App;

/// Render the agent tab bar into `area` (no-op when empty and zero height).
pub fn render_agent_bar(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let tool_agents = tool_agents(app);
    let has_tui = !app.state.spawned_agents.is_empty();
    let has_swarm = !app.state.swarm.subtasks.is_empty();
    if area.height == 0 || (!has_tui && !has_swarm && tool_agents.is_empty()) {
        return;
    }
    let active = app.state.active_spawned_agent.as_deref();
    let model = crate::tui::app::spawn_agent::model::current_model_id(app);
    let mut spans = vec![agent_tab(AgentTabMeta {
        name: "main",
        model_id: model.as_deref(),
        session_id: app.state.session_id.as_deref(),
        indent: 0,
        selected: active.is_none(),
        processing: app.state.processing,
    })];
    push_tui_agents(&mut spans, app, active);
    push_tool_agents(&mut spans, &tool_agents, app, active);
    swarm::push_swarm_agents(&mut spans, app);
    f.render_widget(Paragraph::new(Line::from(spans)), area);
}

fn tool_agents(app: &App) -> Vec<crate::tool::agent::bridge::AgentSnapshot> {
    app.state
        .session_id
        .as_deref()
        .map(crate::tool::agent::bridge::list_agent_tool_agents_for_parent)
        .unwrap_or_else(crate::tool::agent::bridge::list_agent_tool_agents)
}
