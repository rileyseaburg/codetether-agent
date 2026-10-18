//! Native session specialization of shared spawned-agent rendering.
use crate::tui::app::state::SpawnedAgent;
use ratatui::text::Line;
/// Render one child agent row.
pub fn line(agent: &SpawnedAgent, selected: bool) -> Line<'static> {
    codetether_agent_dashboard::spawned::line(agent, selected)
}
