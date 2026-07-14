use crate::secrets::ProviderSecrets;
use serde_json::json;

#[test]
fn parses_vault_oauth_credentials() {
    let mut secrets = ProviderSecrets::default();
    secrets.extra.insert("access_token".into(), json!("access"));
    secrets.extra.insert("refresh_token".into(), json!("refresh"));
    secrets.extra.insert("expires_at".into(), json!(42));
    secrets
        .extra
        .insert("chatgpt_account_id".into(), json!("account"));
    let credentials = super::vault_credentials::oauth_credentials(&secrets).unwrap();
    assert_eq!(credentials.access_token, "access");
    assert_eq!(credentials.refresh_token, "refresh");
    assert_eq!(credentials.expires_at, 42);
    assert_eq!(credentials.chatgpt_account_id.as_deref(), Some("account"));
}

#[test]
fn rejects_incomplete_vault_oauth_credentials() {
    let mut secrets = ProviderSecrets::default();
    secrets.extra.insert("access_token".into(), json!("access"));
    assert!(super::vault_credentials::oauth_credentials(&secrets).is_none());
}
