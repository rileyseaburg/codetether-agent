//! Check-result event for invalid args.
use super::*;

/// Construct the matching check-result event.
///
/// # Arguments
/// * `persona_id` - Persona associated with the check.
/// * `tool_name` - Requested tool name.
/// * `validation_error` - Existing schema-validation error text.
///
/// # Returns
/// A check-result event with the existing payload and a fresh identity.
///
/// # Examples
/// ```
/// # use serde_json::Value;
/// # fn example(persona_id: &str, tool_name: String, validation_error: String) {
/// let _result = codetether_cognition_core::execution::events::invalid_args(persona_id, tool_name, validation_error);
/// # }
/// ```
pub fn invalid_args(persona_id: &str, tool_name: String, validation_error: String) -> ThoughtEvent {
    ThoughtEvent {
        id: Uuid::new_v4().to_string(),
        event_type: ThoughtEventType::CheckResult,
        persona_id: Some(persona_id.to_string()),
        swarm_id: None,
        timestamp: Utc::now(),
        payload: serde_json::json!({
            "tool_rejected": true,
            "tool": tool_name,
            "reason": "schema_validation_failed",
            "detail": validation_error,
        }),
    }
}
