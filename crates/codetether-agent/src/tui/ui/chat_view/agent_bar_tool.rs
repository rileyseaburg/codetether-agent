//! Native registry specialization of shared tool-agent tabs.
use crate::{tool::agent::bridge::AgentSnapshot, tui::app::state::App};
use ratatui::text::Span;
/// Append tool-agent tabs, deduplicating local names.
pub fn push_tool_agents(
    spans: &mut Vec<Span<'static>>,
    snapshots: &[AgentSnapshot],
    app: &App,
    active: Option<&str>,
) {
    codetether_agent_dashboard::bar::push_tool_agents(
        spans,
        snapshots,
        &app.state.agent_tree.spawned,
        active,
    );
}
