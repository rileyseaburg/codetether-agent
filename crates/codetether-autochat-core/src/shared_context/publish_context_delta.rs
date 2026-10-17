//! Relay context publish context delta.
use super::*;

/// Publish a structured context update on the shared result channel.
///
/// # Arguments
/// * `handle` - Publishing agent handle.
/// * `relay_id` - Relay identity.
/// * `from_agent` - Producing agent identity.
/// * `round` - Relay round index.
/// * `turn` - Relay turn index.
/// * `delta` - Context additions to publish.
///
/// # Returns
/// The number of live receivers reached.
///
/// # Examples
/// ```
/// use codetether_bus_core::AgentBus;
/// use codetether_autochat_core::shared_context::{ContextDelta, publish_context_delta};
/// let bus = AgentBus::new().into_arc();
/// let handle = bus.handle("worker");
/// assert!(publish_context_delta(&handle, "demo", "worker", 0, 0, &ContextDelta::default()) >= 1);
/// ```
pub fn publish_context_delta(
    handle: &BusHandle,
    relay_id: &str,
    from_agent: &str,
    round: usize,
    turn: usize,
    delta: &ContextDelta,
) -> usize {
    let envelope = ContextDeltaEnvelope {
        relay_id: relay_id.to_string(),
        from_agent: from_agent.to_string(),
        round,
        turn,
        delta: delta.clone(),
    };

    let value = serde_json::to_value(&envelope).unwrap_or(serde_json::json!({}));
    let tags = vec![
        "relay-context".to_string(),
        format!("relay:{relay_id}"),
        format!("agent:{from_agent}"),
    ];
    handle.publish_shared_result(context_result_key(relay_id, turn), value, tags)
}
