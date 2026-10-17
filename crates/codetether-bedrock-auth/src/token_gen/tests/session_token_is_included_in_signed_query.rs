//! Preserved token-generation regression.
use super::*;

#[test]
fn session_token_is_included_in_signed_query() {
    let now = Utc.with_ymd_and_hms(2024, 1, 15, 12, 0, 0).unwrap();
    let mut creds = test_creds();
    creds.session_token = Some("FwoGZXIvYXdzEBc/token+chars=".into());
    let token = generate_at(&creds, "us-east-1", 3600, now);
    let raw = base64::engine::general_purpose::STANDARD
        .decode(token.strip_prefix("bedrock-api-key-").unwrap())
        .unwrap();
    let url = String::from_utf8(raw).unwrap();
    assert!(url.contains("X-Amz-Security-Token="));
    assert!(url.contains("X-Amz-Expires=3600"));
    assert!(url.ends_with("&Version=1"));
}
