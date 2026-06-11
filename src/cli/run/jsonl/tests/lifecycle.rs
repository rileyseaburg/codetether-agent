use super::{RunEvent, event_bytes, event_value};
use crate::provider::Usage;

#[test]
fn started_event_is_tagged_json_line() {
    let out = event_bytes(&RunEvent::Started);
    let value: serde_json::Value = serde_json::from_slice(&out).unwrap();
    assert_eq!(value["type"], "run.started");
    assert!(out.ends_with(b"\n"));
}

#[test]
fn completed_event_includes_response_session_and_usage() {
    let usage = Usage {
        prompt_tokens: 2,
        completion_tokens: 3,
        total_tokens: 5,
        ..Usage::default()
    };
    let value = event_value(&RunEvent::Completed {
        session_id: Some("s1"),
        response: "done",
        usage: Some(&usage),
    });
    assert_eq!(value["type"], "run.completed");
    assert_eq!(value["response"], "done");
    assert_eq!(value["session_id"], "s1");
    assert_eq!(value["usage"]["total_tokens"], 5);
}

#[test]
fn failed_event_can_carry_final_response() {
    let value = event_value(&RunEvent::Failed {
        error: "relay failed",
        response: Some("partial summary"),
    });
    assert_eq!(value["type"], "run.failed");
    assert_eq!(value["error"], "relay failed");
    assert_eq!(value["response"], "partial summary");
}
