//! Relay context drain context updates.
use super::*;

/// Drain queued result messages and merge matching relay context updates.
///
/// # Arguments
/// * `receiver` - Queue drained by this operation, including unrelated messages.
/// * `relay_id` - Relay whose updates should be applied.
/// * `shared` - Destination context buckets.
///
/// # Returns
/// The number of matching deltas merged.
///
/// # Examples
/// ```
/// use codetether_bus_core::AgentBus;
/// use codetether_autochat_core::shared_context::{SharedRelayContext, drain_context_updates};
/// let bus = AgentBus::new().into_arc();
/// let mut receiver = bus.handle("worker");
/// assert_eq!(drain_context_updates(&mut receiver, "demo", &mut SharedRelayContext::default()), 0);
/// ```
pub fn drain_context_updates(
    receiver: &mut BusHandle,
    relay_id: &str,
    shared: &mut SharedRelayContext,
) -> usize {
    let topic_prefix = format!("results.{}", context_result_prefix(relay_id));
    let key_prefix = context_result_prefix(relay_id);
    let mut merged = 0usize;

    while let Some(envelope) = receiver.try_recv() {
        if !envelope.topic.starts_with(&topic_prefix) {
            continue;
        }

        let BusMessage::SharedResult { key, value, .. } = envelope.message else {
            continue;
        };
        if !key.starts_with(&key_prefix) {
            continue;
        }

        if let Ok(payload) = serde_json::from_value::<ContextDeltaEnvelope>(value.clone()) {
            if payload.relay_id == relay_id {
                shared.merge_delta(&payload.delta);
                merged += 1;
            }
            continue;
        }

        if let Ok(delta) = serde_json::from_value::<ContextDelta>(value) {
            shared.merge_delta(&delta);
            merged += 1;
        }
    }

    merged
}
