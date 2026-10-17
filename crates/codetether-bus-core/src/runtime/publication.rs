//! Envelope construction for scoped senders.
use super::*;

/// Build and publish one envelope on behalf of an agent handle.
///
/// Shared by [`BusHandle::send_with_correlation`](super::BusHandle) and the
/// catalog announcement so envelope construction lives in one place.
pub(super) fn publish(
    bus: &Arc<AgentBus>,
    agent_id: &str,
    topic: String,
    message: BusMessage,
    correlation_id: Option<String>,
) -> usize {
    bus.publish(BusEnvelope {
        id: uuid::Uuid::new_v4().to_string(),
        topic,
        sender_id: agent_id.to_string(),
        correlation_id,
        timestamp: chrono::Utc::now(),
        message,
    })
}
