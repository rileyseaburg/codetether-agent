//! Thread event fixture.
use crate::ThreadEvent;
use serde_json::json;

pub(super) fn event(thread_id: &str, event_id: &str) -> ThreadEvent {
    ThreadEvent {
        event_id: event_id.into(),
        thread_id: thread_id.into(),
        session_id: thread_id.into(),
        turn_id: "turn-1".into(),
        kind: "turn.started".into(),
        timestamp_ms: 1,
        payload: json!({ "ok": true }),
    }
}
