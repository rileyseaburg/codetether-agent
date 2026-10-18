use crate::*;

#[test]
fn skips_legacy_state_marker() {
    let raw = r#"{"record_type":"state"}"#;
    assert!(normalize_line(raw, None).unwrap().is_none());
}
