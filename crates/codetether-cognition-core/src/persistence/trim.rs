//! Apply the existing storage payload limit to retained evidence events.
use crate::ThoughtEvent;

/// Trim event payloads for storage (remove very large fields).
pub(super) fn trim_event_for_storage(event: &ThoughtEvent) -> ThoughtEvent {
    let mut trimmed = event.clone();
    // Trim thinking field if present
    if let Some(thinking) = trimmed.payload.get("thinking").and_then(|v| v.as_str())
        && thinking.len() > 500
    {
        let short = &thinking[..500];
        trimmed.payload["thinking"] = serde_json::Value::String(format!("{}...", short));
    }
    trimmed
}
