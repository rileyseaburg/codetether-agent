use crate::*;
use chrono::{DateTime, Utc};

#[test]
fn normalizes_legacy_response_item() {
    let raw = r#"{"type":"message","id":null,"role":"user","content":[{"type":"input_text","text":"hi"}]}"#;
    let ts: DateTime<Utc> = "2025-08-28T08:45:13.563Z".parse().unwrap();
    let rec = normalize_line(raw, Some(ts)).unwrap().unwrap();
    assert_eq!(rec.kind, "response_item");
    assert_eq!(rec.payload.get("type").unwrap(), "message");
}
