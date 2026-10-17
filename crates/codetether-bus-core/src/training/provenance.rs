//! Envelope identity and step provenance for training records.
use super::*;
pub(super) fn metadata(env: &BusEnvelope) -> TrainingMetadata {
    let meta = TrainingMetadata {
        bus_kind: bus_message_kind(&env.message),
        envelope_id: env.id.clone(),
        timestamp: env.timestamp.to_rfc3339(),
        topic: env.topic.clone(),
        sender_id: env.sender_id.clone(),
        correlation_id: env.correlation_id.clone(),
        step: envelope_step(&env.message),
    };
    meta
}
