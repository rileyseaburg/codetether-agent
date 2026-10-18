use crate::*;

#[test]
fn normalizes_legacy_meta() {
    let raw =
        r#"{"id":"ce59ef14","timestamp":"2025-08-28T08:45:13.563Z","instructions":"hi","git":{}}"#;
    let rec = normalize_line(raw, None).unwrap().unwrap();
    assert_eq!(rec.kind, "session_meta");
    let meta: CodexSessionMetaPayload = serde_json::from_value(rec.payload).unwrap();
    assert_eq!(meta.id, "ce59ef14");
    assert_eq!(meta.cwd, "");
}
