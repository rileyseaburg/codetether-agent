//! Check-result event for executed.
use super::*;

/// Construct the matching check-result event.
///
/// # Arguments
/// * `persona_id` - Persona associated with the check.
/// * `tool_name` - Requested tool name.
/// * `lease_id` - Capability lease identifier.
/// * `success` - Whether the executed tool reported success.
/// * `result_text` - Clipped output or error text to record.
///
/// # Returns
/// A check-result event with the existing payload and a fresh identity.
///
/// # Examples
/// ```
/// # use serde_json::Value;
/// # fn example( persona_id: &str, tool_name: String, lease_id: String, success: bool, result_text: String, ) {
/// let _result = codetether_cognition_core::execution::events::executed(persona_id, tool_name, lease_id, success, result_text);
/// # }
/// ```
pub fn executed(
    persona_id: &str,
    tool_name: String,
    lease_id: String,
    success: bool,
    result_text: String,
) -> ThoughtEvent {
    ThoughtEvent {
        id: Uuid::new_v4().to_string(),
        event_type: ThoughtEventType::CheckResult,
        persona_id: Some(persona_id.to_string()),
        swarm_id: None,
        timestamp: Utc::now(),
        payload: serde_json::json!({
            "tool_executed": true,
            "tool": tool_name,
            "lease_id": lease_id,
            "success": success,
            "result": result_text,
        }),
    }
}
