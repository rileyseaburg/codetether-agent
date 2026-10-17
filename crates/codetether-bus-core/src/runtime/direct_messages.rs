//! direct messages operations for the shared bus runtime.
use super::*;
impl BusHandle {
    /// Send a direct message to another agent.
    pub fn send_to_agent(&self, to: &str, parts: Vec<Part>) -> usize {
        self.send(
            format!("agent.{to}"),
            BusMessage::AgentMessage {
                from: self.agent_id.clone(),
                to: to.to_string(),
                parts,
            },
        )
    }

    /// Publish a shared result (visible to the entire swarm).
    pub fn publish_shared_result(
        &self,
        key: impl Into<String>,
        value: serde_json::Value,
        tags: Vec<String>,
    ) -> usize {
        let key = key.into();
        self.send(
            format!("results.{}", &key),
            BusMessage::SharedResult { key, value, tags },
        )
    }
}
