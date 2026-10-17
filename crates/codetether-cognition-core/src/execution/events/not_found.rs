//! Check-result event for not found.
use super::*;

/// Construct the matching check-result event.
///
/// # Arguments
/// * `persona_id` - Persona associated with the check.
/// * `tool_name` - Requested tool name.
///
/// # Returns
/// A check-result event with the existing payload and a fresh identity.
///
/// # Examples
/// ```
/// # use serde_json::Value;
/// # fn example(persona_id: &str, tool_name: String) {
/// let _result = codetether_cognition_core::execution::events::not_found(persona_id, tool_name);
/// # }
/// ```
pub fn not_found(persona_id: &str, tool_name: String) -> ThoughtEvent {
    ThoughtEvent {
        id: Uuid::new_v4().to_string(),
        event_type: ThoughtEventType::CheckResult,
        persona_id: Some(persona_id.to_string()),
        swarm_id: None,
        timestamp: Utc::now(),
        payload: serde_json::json!({
            "tool_rejected": true,
            "tool": tool_name,
            "reason": "tool not found in registry",
        }),
    }
}
