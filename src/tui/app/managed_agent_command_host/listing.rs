//! Project legacy sessions and parent-scoped agent snapshots.
use super::Native;
use codetether_agent_command::{Listing, roster::LegacyAgent};
impl Listing for Native<'_> {
    fn legacy_rows(&self) -> Vec<LegacyAgent> {
        self.app
            .state
            .agent_tree
            .spawned
            .iter()
            .map(|(key, agent)| LegacyAgent {
                key: key.clone(),
                name: agent.name.clone(),
                message_count: agent.session.history().len(),
                model: agent.session.metadata.model.clone(),
            })
            .collect()
    }
    fn managed_rows(&self) -> Vec<codetether_agent_view::AgentSnapshot> {
        self.app
            .state
            .session_id
            .as_deref()
            .map(crate::tool::agent::bridge::list_agent_tool_agents_for_parent)
            .unwrap_or_default()
    }
    fn swarm_count(&self) -> usize {
        self.app.state.swarm.subtasks.len()
    }
}
