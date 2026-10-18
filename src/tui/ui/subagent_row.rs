//! Project managed child display fields into the shared dashboard row.
use crate::tui::app::state::SpawnedAgent;
use codetether_agent_dashboard::managed::Agent;
use ratatui::text::Line;
/// Render one child agent row.
pub fn line(agent: &SpawnedAgent, selected: bool) -> Line<'static> {
    codetether_agent_dashboard::managed::line(
        &codetether_agent_dashboard::Child::display(agent),
        selected,
    )
}

impl codetether_agent_dashboard::Child for SpawnedAgent {
    fn display(&self) -> Agent<'_> {
        Agent {
            name: &self.name,
            parent: self.parent.as_deref(),
            depth: self.depth,
            instructions: &self.instructions,
            model_id: self.model_id.as_deref(),
            is_processing: self.is_processing,
        }
    }
}
