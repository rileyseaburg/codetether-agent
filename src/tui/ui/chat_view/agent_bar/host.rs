//! Native session identity and registry access for the shared header.
use crate::{
    session::Session,
    tui::app::state::{App, SpawnedAgent},
};
use codetether_agent_view::AgentSnapshot;
use codetether_swarm_view::SubTaskInfo;
use std::collections::HashMap;
impl codetether_agent_dashboard::bar::SessionId for Session {
    fn session_id(&self) -> &str {
        &self.id
    }
}
impl codetether_agent_dashboard::bar::Host for App {
    type Session = Session;
    fn agents(&self) -> &HashMap<String, SpawnedAgent> {
        &self.state.agent_tree.spawned
    }
    fn tool_agents(&self) -> Vec<AgentSnapshot> {
        self.state
            .session_id
            .as_deref()
            .map(crate::tool::agent::bridge::list_agent_tool_agents_for_parent)
            .unwrap_or_else(crate::tool::agent::bridge::list_agent_tool_agents)
    }
    fn subtasks(&self) -> &[SubTaskInfo] {
        &self.state.swarm.subtasks
    }
    fn active(&self) -> Option<&str> {
        self.state.agent_tree.active.as_deref()
    }
    fn model(&self) -> Option<String> {
        crate::tui::app::spawn_agent::model::current_model_id(self)
    }
    fn session_id(&self) -> Option<&str> {
        self.state.session_id.as_deref()
    }
    fn processing(&self) -> bool {
        self.state.processing
    }
}
