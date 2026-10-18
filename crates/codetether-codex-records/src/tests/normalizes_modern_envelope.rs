use crate::*;

#[test]
fn normalizes_modern_envelope() {
    let raw = r#"{"timestamp":"2025-08-28T08:45:13.563Z","type":"session_meta","payload":{"id":"abc","timestamp":"2025-08-28T08:45:13.563Z","cwd":"/tmp"}}"#;
    let rec = normalize_line(raw, None).unwrap().unwrap();
    assert_eq!(rec.kind, "session_meta");
}
