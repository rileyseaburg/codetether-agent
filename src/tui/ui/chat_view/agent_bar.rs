//! Header tab bar showing the spawned-agent tree (main + sibling agents).
//!
//! Merges both registries: TUI `/spawn` agents and agent-tool-spawned agents
//! (issue #295 / #297 Part A).

use ratatui::{
    Frame,
    text::{Line, Span},
    widgets::Paragraph,
};

use super::agent_bar_tool::push_tool_agents;
use super::agent_bar_tui::push_tui_agents;
use super::agent_tab::{AgentTabMeta, agent_tab};
use crate::tui::app::state::App;

/// Render the agent tab bar into `area` (no-op when empty and zero height).
pub fn render_agent_bar(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let tool_agents = crate::tool::agent::bridge::list_agent_tool_agents();
    let has_tui = !app.state.spawned_agents.is_empty();
    if area.height == 0 || (!has_tui && tool_agents.is_empty()) {
        return;
    }
    let active = app.state.active_spawned_agent.as_deref();
    let mut spans = vec![agent_tab(AgentTabMeta {
        name: "main",
        model_id: app.state.last_completion_model.as_deref(),
        session_id: app.state.session_id.as_deref(),
        indent: 0,
        selected: active.is_none(),
        processing: app.state.processing,
    })];
    push_tui_agents(&mut spans, app, active);
    push_tool_agents(&mut spans, &tool_agents, app, active);
    f.render_widget(Paragraph::new(Line::from(spans)), area);
}
