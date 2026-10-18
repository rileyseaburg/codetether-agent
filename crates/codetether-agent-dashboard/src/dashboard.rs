//! Composition of managed, tool-agent and swarm dashboard rows.
use crate::child::Child;
use codetether_agent_view::AgentSnapshot;
use codetether_swarm_view::SwarmViewState;
use ratatui::{style::Stylize, text::Line};
use std::collections::HashMap;

/// Borrowed inputs for one dashboard render.
/// # Examples
/// ```no_run
/// fn view<C: codetether_agent_dashboard::Child>(agents: &std::collections::HashMap<String, C>, swarm: &codetether_swarm_view::SwarmViewState) {
///     let dashboard = codetether_agent_dashboard::dashboard::Dashboard { agents, swarm, active_tasks: 0, selected: None };
///     assert_eq!(dashboard.active_tasks, 0);
/// }
/// ```
pub struct Dashboard<'a, C> {
    /// Managed children keyed by their registered names.
    pub agents: &'a HashMap<String, C>,
    /// Active or most recent swarm state.
    pub swarm: &'a SwarmViewState,
    /// Number of active tasks.
    pub active_tasks: usize,
    /// Focused child name, if any.
    pub selected: Option<&'a str>,
}

/// Compose dashboard rows, deduplicating tool children by managed-map keys.
/// # Arguments
/// * `state` - Borrowed dashboard inputs.
/// * `tool_agents` - Current tool-agent snapshots.
/// # Returns
/// Summary, navigation, child rows and an empty-state hint when appropriate.
/// # Examples
/// ```no_run
/// fn rows<C: codetether_agent_dashboard::Child>(view: &codetether_agent_dashboard::dashboard::Dashboard<'_, C>) {
///     assert!(!codetether_agent_dashboard::dashboard::compose(view, &[]).is_empty());
/// }
/// ```
pub fn compose<C: Child>(
    state: &Dashboard<'_, C>,
    tool_agents: &[AgentSnapshot],
) -> Vec<Line<'static>> {
    let tool_count = tool_agents
        .iter()
        .filter(|agent| !state.agents.contains_key(&agent.name))
        .count();
    let managed_count = state.agents.len() + tool_count;
    let swarm_count = state.swarm.subtasks.len();
    let mut rows = vec![Line::from(
        format!(
            "active tasks: {} · managed: {managed_count} · swarm: {swarm_count}",
            state.active_tasks
        )
        .green(),
    )];
    rows.push(Line::from(
        "Tab/↑↓: child · Enter: transcript · /swarm: worker details".dim(),
    ));
    rows.push(Line::from(""));
    crate::managed_section::append(&mut rows, state, tool_agents);
    crate::swarm::append(&mut rows, &state.swarm);
    if managed_count == 0 && swarm_count == 0 {
        rows.push(Line::from(
            "No agents yet. Use /spawn, the agent tool, or /swarm <task>.".dim(),
        ));
    }
    rows
}
