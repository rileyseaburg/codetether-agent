//! Header tab bar showing the spawned-agent tree (main + sibling agents).

use ratatui::{Frame, text::Line, widgets::Paragraph};

use super::agent_tab::{AgentTabMeta, agent_tab};
use crate::tui::app::state::App;
use crate::tui::app::state::agent_tree::dfs_order;

/// Render the agent tab bar into `area` (no-op when `area` has zero height).
pub fn render_agent_bar(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    if area.height == 0 || app.state.spawned_agents.is_empty() {
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
    for node in dfs_order(&app.state.spawned_agents) {
        if let Some(agent) = app.state.spawned_agents.get(&node.name) {
            spans.push(ratatui::text::Span::raw(" "));
            spans.push(agent_tab(AgentTabMeta {
                name: &node.name,
                model_id: agent.model_id.as_deref(),
                session_id: Some(&agent.session.id),
                indent: node.depth + 1,
                selected: active == Some(node.name.as_str()),
                processing: agent.is_processing,
            }));
        }
    }
    f.render_widget(Paragraph::new(Line::from(spans)), area);
}
