//! Codex access/refresh/account metadata for the Vault record.
use codetether_provider_codex::OAuthCredentials;
use std::collections::HashMap;

pub(super) fn fields(
    credentials: &OAuthCredentials,
    chatgpt_account_id: &Option<String>,
) -> HashMap<String, serde_json::Value> {
    let mut extra = HashMap::new();
    extra.insert(
        "access_token".to_string(),
        serde_json::Value::String(credentials.access_token.clone()),
    );
    extra.insert(
        "refresh_token".to_string(),
        serde_json::Value::String(credentials.refresh_token.clone()),
    );
    extra.insert(
        "expires_at".to_string(),
        serde_json::Value::Number(credentials.expires_at.into()),
    );
    if let Some(id_token) = credentials.id_token.as_ref() {
        extra.insert(
            "id_token".to_string(),
            serde_json::Value::String(id_token.clone()),
        );
    }
    if let Some(account_id) = chatgpt_account_id.as_ref() {
        extra.insert(
            "chatgpt_account_id".to_string(),
            serde_json::Value::String(account_id.clone()),
        );
    }

    extra
}
