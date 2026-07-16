//! Direct and command-swarm workers rendered in the main agent bar.

use super::super::agent_tab::{AgentTabMeta, agent_tab};
use crate::swarm::SubTaskStatus;
use crate::tui::app::state::App;
use ratatui::text::Span;

pub(super) fn push_swarm_agents(spans: &mut Vec<Span<'static>>, app: &App) {
    for task in &app.state.swarm.subtasks {
        let name = task.agent_name.as_deref().unwrap_or(&task.name);
        spans.push(Span::raw(" "));
        spans.push(agent_tab(AgentTabMeta {
            name,
            model_id: None,
            session_id: None,
            indent: 1,
            selected: false,
            processing: task.status == SubTaskStatus::Running,
        }));
    }
}
