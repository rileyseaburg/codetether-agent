//! Extract silent-refresh inputs from a stored Bedrock Vault secret.

use crate::secrets::ProviderSecrets;

/// Owned copies of the SSO fields needed for a non-interactive refresh.
pub(crate) struct StoredSso {
    pub region: String,
    pub client_id: String,
    pub client_secret: String,
    pub refresh_token: String,
    pub account_id: String,
    pub role_name: String,
}

/// Pull SSO refresh material from a provider secret, if all fields exist.
pub(crate) fn from_secret(secrets: &ProviderSecrets) -> Option<StoredSso> {
    let get = |key: &str| {
        secrets
            .extra
            .get(key)
            .and_then(|v| v.as_str())
            .map(str::to_string)
    };
    Some(StoredSso {
        region: get("sso_region").or_else(|| get("region"))?,
        client_id: get("sso_client_id")?,
        client_secret: get("sso_client_secret")?,
        refresh_token: get("sso_refresh_token")?,
        account_id: get("sso_account_id")?,
        role_name: get("sso_role_name")?,
    })
}
