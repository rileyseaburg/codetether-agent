//! Check-result event for expired.
use super::*;

/// Construct the matching check-result event.
///
/// # Arguments
/// * `persona_id` - Persona associated with the check.
/// * `tool_name` - Requested tool name.
/// * `lease_id` - Capability lease identifier.
///
/// # Returns
/// A check-result event with the existing payload and a fresh identity.
///
/// # Examples
/// ```
/// # use serde_json::Value;
/// # fn example(persona_id: &str, tool_name: String, lease_id: String) {
/// let _result = codetether_cognition_core::execution::events::expired(persona_id, tool_name, lease_id);
/// # }
/// ```
pub fn expired(persona_id: &str, tool_name: String, lease_id: String) -> ThoughtEvent {
    ThoughtEvent {
        id: Uuid::new_v4().to_string(),
        event_type: ThoughtEventType::CheckResult,
        persona_id: Some(persona_id.to_string()),
        swarm_id: None,
        timestamp: Utc::now(),
        payload: serde_json::json!({
            "tool_rejected": true,
            "tool": tool_name,
            "reason": "capability_lease_expired",
            "lease_id": lease_id,
        }),
    }
}
