//! Protocol-first relay runtime for multi-agent handoff flows.
//!
//! This layer keeps orchestration transport-agnostic while using the local
//! `AgentBus` as the default protocol transport. It provides:
//! - Agent lifecycle registration/deregistration
//! - Message handoff publication with correlation ids
//! - Relay topic emission for observability (`relay.{relay_id}`)

use super::{AgentBus, BusMessage};
use crate::a2a::types::Part;
use std::sync::Arc;
use uuid::Uuid;

/// Profile for a relay-participating agent.
#[derive(Debug, Clone)]
pub struct RelayAgentProfile {
    pub name: String,
    pub capabilities: Vec<String>,
}

/// Protocol runtime used by auto-chat and other relay-style orchestrators.
#[derive(Clone)]
pub struct ProtocolRelayRuntime {
    bus: Arc<AgentBus>,
    relay_id: String,
}

impl ProtocolRelayRuntime {
    /// Build a relay runtime with an auto-generated relay id.
    pub fn new(bus: Arc<AgentBus>) -> Self {
        Self {
            bus,
            relay_id: format!("relay-{}", &Uuid::new_v4().to_string()[..8]),
        }
    }

    /// Build a relay runtime with an explicit id.
    pub fn with_relay_id(bus: Arc<AgentBus>, relay_id: impl Into<String>) -> Self {
        Self {
            bus,
            relay_id: relay_id.into(),
        }
    }

    /// The relay id used for correlation and topic publication.
    pub fn relay_id(&self) -> &str {
        &self.relay_id
    }

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

    /// Send one protocol handoff from `from` to `to`.
    ///
    /// Publishes to both `agent.{to}` and `relay.{relay_id}` so downstream
    /// observers (TUI bus log, metrics) can trace the conversation flow.
    pub fn send_handoff(&self, from: &str, to: &str, text: &str) {
        let payload = BusMessage::AgentMessage {
            from: from.to_string(),
            to: to.to_string(),
            parts: vec![Part::Text {
                text: text.to_string(),
            }],
        };

        let correlation = Some(format!("{}:{}", self.relay_id, Uuid::new_v4()));
        let handle = self.bus.handle(from.to_string());

        handle.send_with_correlation(format!("agent.{to}"), payload.clone(), correlation.clone());
        handle.send_with_correlation(format!("relay.{}", self.relay_id), payload, correlation);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::{Duration, timeout};

    #[tokio::test]
    async fn test_register_and_shutdown_agents() {
        let bus = AgentBus::new().into_arc();
        let relay = ProtocolRelayRuntime::with_relay_id(bus.clone(), "relay-test");

        let agents = vec![
            RelayAgentProfile {
                name: "auto-planner".to_string(),
                capabilities: vec!["planning".to_string()],
            },
            RelayAgentProfile {
                name: "auto-coder".to_string(),
                capabilities: vec!["coding".to_string()],
            },
        ];

        relay.register_agents(&agents);

        assert!(bus.registry.get("auto-planner").is_some());
        assert!(bus.registry.get("auto-coder").is_some());

        relay.shutdown_agents(&["auto-planner".to_string(), "auto-coder".to_string()]);

        assert!(bus.registry.get("auto-planner").is_none());
        assert!(bus.registry.get("auto-coder").is_none());
    }

    #[tokio::test]
    async fn test_send_handoff_emits_agent_and_relay_topics() {
        let bus = AgentBus::new().into_arc();
        let relay = ProtocolRelayRuntime::with_relay_id(bus.clone(), "relay-test");
        let mut observer = bus.handle("observer");

        relay.send_handoff("auto-planner", "auto-coder", "handoff payload");

        let first = timeout(Duration::from_millis(200), observer.recv())
            .await
            .expect("first envelope timeout")
            .expect("first envelope missing");
        let second = timeout(Duration::from_millis(200), observer.recv())
            .await
            .expect("second envelope timeout")
            .expect("second envelope missing");

        let topics = [first.topic, second.topic];
        assert!(topics.iter().any(|t| t == "agent.auto-coder"));
        assert!(topics.iter().any(|t| t == "relay.relay-test"));
    }
}
