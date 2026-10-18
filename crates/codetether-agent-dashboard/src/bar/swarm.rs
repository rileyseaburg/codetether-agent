//! Direct and command-swarm workers rendered in the main agent bar.

use super::tab::{AgentTabMeta, agent_tab};
use codetether_swarm_core::SubTaskStatus;
use codetether_swarm_view::SubTaskInfo;
use ratatui::text::Span;

pub(super) fn push_swarm_agents(
    spans: &mut Vec<Span<'static>>,
    subtasks: &[SubTaskInfo],
    model: Option<String>,
    active: Option<&str>,
) {
    for task in subtasks {
        let name = task.agent_name.as_deref().unwrap_or(&task.name);
        spans.push(Span::raw(" "));
        spans.push(agent_tab(AgentTabMeta {
            name,
            model_id: model.as_deref(),
            session_id: Some(&task.id),
            identity: None,
            indent: 1,
            selected: active == Some(name),
            processing: task.status == SubTaskStatus::Running,
        }));
    }
}
