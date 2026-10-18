//! Project managed child display fields into the shared dashboard row.
use crate::tui::app::state::SpawnedAgent;
use codetether_agent_dashboard::managed::Agent;
use ratatui::text::Line;
/// Render one child agent row.
pub fn line(agent: &SpawnedAgent, selected: bool) -> Line<'static> {
    codetether_agent_dashboard::managed::line(
        &Agent {
            name: &agent.name,
            parent: agent.parent.as_deref(),
            depth: agent.depth,
            instructions: &agent.instructions,
            model_id: agent.model_id.as_deref(),
            is_processing: agent.is_processing,
        },
        selected,
    )
}
