//! Codex device response interval decoding regressions.
use crate::CodexDeviceCodeResponse;

#[test]
fn parses_codex_device_interval_from_string() {
    let parsed: CodexDeviceCodeResponse =
        serde_json::from_str(r#"{"device_auth_id":"id-1","user_code":"ABCD-EFGH","interval":"7"}"#)
            .expect("expected valid device-code payload");
    assert_eq!(parsed.interval, 7);
}

#[test]
fn parses_codex_device_interval_from_number() {
    let parsed: CodexDeviceCodeResponse =
        serde_json::from_str(r#"{"device_auth_id":"id-1","user_code":"ABCD-EFGH","interval":9}"#)
            .expect("expected valid numeric interval payload");
    assert_eq!(parsed.interval, 9);
}
