//! Project a managed child's identity and transcript into shared formatting.
use crate::tui::app::state::SpawnedAgent;
use codetether_agent_dashboard::Child;
use ratatui::text::Line;
pub(super) fn lines(agent: &SpawnedAgent) -> Vec<Line<'static>> {
    codetether_agent_dashboard::managed_detail::lines(&agent.display(), &agent.session.messages)
}
