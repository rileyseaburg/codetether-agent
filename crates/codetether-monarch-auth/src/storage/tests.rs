//! Configured-backend record conversion never exposes or drops unrelated fields.
use super::record::{decode, updated};
use crate::oauth::Credentials;
use codetether_secrets::ProviderSecrets;
use serde_json::json;

fn fixture() -> Credentials {
    Credentials {
        client_id: "fixture-client".into(),
        access_token: "fixture-access".into(),
        refresh_token: Some("fixture-refresh".into()),
        expires_at: 1234,
        scope: "mcp:read".into(),
    }
}

#[test]
fn record_roundtrip_and_clear_preserve_other_fields() {
    let mut original = ProviderSecrets::default();
    original
        .extra
        .insert("unrelated".into(), json!({"keep":true}));
    let stored = updated(original, Some(&fixture())).unwrap();
    let loaded = decode(Some(stored.clone())).unwrap().unwrap();
    assert_eq!(loaded.client_id, "fixture-client");
    assert_eq!(loaded.refresh_token.as_deref(), Some("fixture-refresh"));
    let cleared = updated(stored, None).unwrap();
    assert_eq!(cleared.extra["unrelated"], json!({"keep":true}));
    assert!(decode(Some(cleared)).unwrap().is_none());
    assert!(decode(None).unwrap().is_none());
}

#[test]
fn invalid_records_fail_without_echoing_secret_fields() {
    let mut stored = ProviderSecrets::default();
    stored.extra.insert(
        "monarch_oauth".into(),
        json!({"access_token":"private-fixture"}),
    );
    let error = decode(Some(stored)).err().unwrap().to_string();
    assert!(!error.contains("private-fixture"));
    assert!(error.contains("Invalid Monarch credential record"));
}
