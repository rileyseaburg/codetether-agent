//! Runtime approval and capability-lease gate for cognition tools.

use super::{ThoughtEvent, ThoughtEventType, executor::CapabilityLease};
use chrono::Utc;
use serde_json::{Value, json};
use uuid::Uuid;

pub(super) async fn rejection(
    persona_id: &str,
    tool: &str,
    arguments: &Value,
    lease: &CapabilityLease,
) -> Option<ThoughtEvent> {
    if let Some(blocked) = crate::runtime_policy::evaluate_tool_invocation(tool, arguments).await {
        return Some(event(
            persona_id,
            tool,
            "runtime_policy",
            json!(blocked.output),
        ));
    }
    (!lease.is_valid()).then(|| {
        event(
            persona_id,
            tool,
            "capability_lease_expired",
            json!(lease.id),
        )
    })
}

fn event(persona_id: &str, tool: &str, reason: &str, detail: Value) -> ThoughtEvent {
    ThoughtEvent {
        id: Uuid::new_v4().to_string(),
        event_type: ThoughtEventType::CheckResult,
        persona_id: Some(persona_id.to_string()),
        swarm_id: None,
        timestamp: Utc::now(),
        payload: json!({
            "tool_rejected": true,
            "tool": tool,
            "reason": reason,
            "detail": detail,
        }),
    }
}
