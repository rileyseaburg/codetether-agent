//! announce operations for the shared bus runtime.
use super::*;
impl BusHandle {
    /// Announce this agent as ready.
    pub fn announce_ready(&self, capabilities: Vec<String>) -> usize {
        self.send(
            "broadcast",
            BusMessage::AgentReady {
                agent_id: self.agent_id.clone(),
                capabilities,
            },
        )
    }

    /// Record the tool schemas this agent can call, for training fidelity.
    pub fn announce_tool_catalog(&self, tools: Vec<tool_catalog::ToolSchema>) -> usize {
        self.send(
            "broadcast",
            BusMessage::AgentToolCatalog {
                agent_id: self.agent_id.clone(),
                tools,
            },
        )
    }

    /// Announce this agent is shutting down.
    pub fn announce_shutdown(&self) -> usize {
        self.send(
            "broadcast",
            BusMessage::AgentShutdown {
                agent_id: self.agent_id.clone(),
            },
        )
    }
}
