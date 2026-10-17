//! Parse OAuth fields stored in a provider secret record.
use super::*;

/// Parse Codex OAuth credentials from a provider secret record.
pub fn credentials(secrets: &ProviderSecrets) -> Option<OAuthCredentials> {
    Some(OAuthCredentials {
        access_token: value(secrets, "access_token")?,
        refresh_token: value(secrets, "refresh_token")?,
        expires_at: secrets.extra.get("expires_at")?.as_u64()?,
        id_token: value(secrets, "id_token"),
        chatgpt_account_id: value(secrets, "chatgpt_account_id"),
    })
}

fn value(secrets: &ProviderSecrets, key: &str) -> Option<String> {
    secrets.extra.get(key)?.as_str().map(String::from)
}
