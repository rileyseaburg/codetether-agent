mod common;
use codetether_companion_protocol::{EventKind, ScreenEvent};
use common::{fixtures, round_trip};

#[test]
fn all_sse_kinds_preserve_fields_and_optional_omissions() {
    let f = fixtures();
    for event in f["events"].as_array().unwrap() {
        round_trip::<ScreenEvent>(event);
    }
    let first: ScreenEvent = serde_json::from_value(f["events"][0].clone()).unwrap();
    assert_eq!(first.kind, EventKind::Snapshot);
    assert!(first.captured_at.is_none());
}

#[test]
fn unknown_kinds_and_invalid_sequences_fail_closed() {
    for event in fixtures()["invalid_events"].as_array().unwrap() {
        assert!(serde_json::from_value::<ScreenEvent>(event.clone()).is_err());
    }
}

#[test]
fn unknown_statuses_and_fields_preserve_forward_compatible_decoding() {
    let event = serde_json::json!({
        "type": "snapshot", "seq": 7, "status": "future_status", "future_field": true
    });
    let decoded: ScreenEvent = serde_json::from_value(event).unwrap();
    assert_eq!(decoded.status.as_deref(), Some("future_status"));
}
