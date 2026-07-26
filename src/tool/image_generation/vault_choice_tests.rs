use super::vault_choice::{self, Choice};
use crate::secrets::ProviderSecrets;
use serde_json::json;

fn oauth_secrets() -> ProviderSecrets {
    let mut secrets = ProviderSecrets::default();
    secrets.extra.insert("access_token".into(), json!("expired"));
    secrets
        .extra
        .insert("refresh_token".into(), json!("invalid"));
    secrets.extra.insert("expires_at".into(), json!(1));
    secrets
}

#[test]
fn api_key_takes_precedence_over_stale_oauth() {
    let mut secrets = oauth_secrets();
    secrets.api_key = Some("valid-key".into());
    let Some(Choice::ApiKey(key)) = vault_choice::select(&secrets) else {
        panic!("API key should take precedence");
    };
    assert_eq!(key, "valid-key");
}

#[test]
fn oauth_is_used_when_api_key_is_absent() {
    assert!(matches!(
        vault_choice::select(&oauth_secrets()),
        Some(Choice::OAuth(_))
    ));
}