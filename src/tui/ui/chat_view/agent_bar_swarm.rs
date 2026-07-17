//! Direct and command-swarm workers rendered in the main agent bar.

use super::super::agent_tab::{AgentTabMeta, agent_tab};
use crate::swarm::SubTaskStatus;
use crate::tui::app::state::App;
use ratatui::text::Span;

pub(super) fn push_swarm_agents(spans: &mut Vec<Span<'static>>, app: &App) {
    let model = crate::tui::app::spawn_agent::model::current_model_id(app);
    let active = app.state.active_spawned_agent.as_deref();
    for task in &app.state.swarm.subtasks {
        let name = task.agent_name.as_deref().unwrap_or(&task.name);
        spans.push(Span::raw(" "));
        spans.push(agent_tab(AgentTabMeta {
            name,
            model_id: model.as_deref(),
            session_id: None,
            indent: 1,
            selected: active == Some(name),
            processing: task.status == SubTaskStatus::Running,
        }));
    }
}
