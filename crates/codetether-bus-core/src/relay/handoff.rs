//! handoff operations for relay participants.
use super::*;
impl ProtocolRelayRuntime {
    /// Send one protocol handoff from `from` to `to`.
    ///
    /// Publishes to both `agent.{to}` and `relay.{relay_id}` so downstream
    /// observers (TUI bus log, metrics) can trace the conversation flow.
    pub fn send_handoff(&self, from: &str, to: &str, text: &str) -> String {
        let payload = BusMessage::AgentMessage {
            from: from.to_string(),
            to: to.to_string(),
            parts: vec![Part::Text {
                text: text.to_string(),
            }],
        };

        let correlation = format!("{}:{}", self.relay_id, Uuid::new_v4());
        let handle = self.bus.handle(from.to_string());

        handle.send_with_correlation(
            format!("agent.{to}"),
            payload.clone(),
            Some(correlation.clone()),
        );
        handle.send_with_correlation(
            format!("relay.{}", self.relay_id),
            payload,
            Some(correlation.clone()),
        );
        correlation
    }
}
