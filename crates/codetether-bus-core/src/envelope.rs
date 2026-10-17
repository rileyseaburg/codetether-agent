use super::*;

// ─── Envelope & Messages ─────────────────────────────────────────────────

/// Metadata wrapper for every message that travels through the bus.
#[derive(Debug, Clone, Serialize, Deserialize)]
///
/// # Examples
/// ```
/// let envelope = codetether_bus_core::BusEnvelope { id: "id".into(), topic: "broadcast".into(), sender_id: "agent".into(), correlation_id: None, timestamp: chrono::Utc::now(), message: codetether_bus_core::BusMessage::Heartbeat { agent_id: "agent".into(), status: "ready".into() } };
/// assert_eq!(envelope.topic, "broadcast");
/// ```
pub struct BusEnvelope {
    /// Unique id for this envelope
    pub id: String,
    /// Hierarchical topic for routing (e.g. `agent.{id}`, `task.{id}`)
    pub topic: String,
    /// The agent that originated this message
    pub sender_id: String,
    /// Optional correlation id (links request → response)
    pub correlation_id: Option<String>,
    /// When the envelope was created
    pub timestamp: DateTime<Utc>,
    /// The payload
    pub message: BusMessage,
}
