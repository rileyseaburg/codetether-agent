//! Project managed child display fields into the shared dashboard row.
use crate::managed::Agent;
use codetether_agent_tree::SpawnedAgent;
use ratatui::text::Line;
/// Render one child agent row without depending on its session implementation.
/// # Arguments
/// * `agent` — Spawned-agent metadata and an application-owned session.
/// * `selected` — Whether the row is selected.
/// # Returns
/// A styled, owned terminal line.
/// # Examples
/// ```
/// let agent = codetether_agent_tree::SpawnedAgent {
///     name: "planner".into(), instructions: String::new(), parent: None,
///     depth: 0, session: (), model_id: None, is_processing: false,
/// };
/// let _ = codetether_agent_dashboard::spawned::line(&agent, false);
/// ```
pub fn line<S>(agent: &SpawnedAgent<S>, selected: bool) -> Line<'static> {
    crate::managed::line(&crate::Child::display(agent), selected)
}

impl<S> crate::Child for SpawnedAgent<S> {
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
