//! registration operations for relay participants.
use super::*;
impl ProtocolRelayRuntime {
    /// Register relay agents on the protocol bus.
    pub fn register_agents(&self, agents: &[RelayAgentProfile]) {
        for agent in agents {
            let mut capabilities = agent.capabilities.clone();
            if !capabilities.iter().any(|c| c == "relay") {
                capabilities.push("relay".to_string());
            }
            capabilities.push(format!("relay:{}", self.relay_id));

            let handle = self.bus.handle(agent.name.clone());
            handle.announce_ready(capabilities);
        }
    }

    /// Deregister relay agents from the protocol bus.
    pub fn shutdown_agents(&self, agent_ids: &[String]) {
        for agent_id in agent_ids {
            let handle = self.bus.handle(agent_id.clone());
            handle.announce_shutdown();
        }
    }
}
