//! vault choice for image generation and editing.
use codetether_provider_codex::OAuthCredentials;
use codetether_secrets::ProviderSecrets;

pub(super) enum Choice {
    ApiKey(String),
    OAuth(OAuthCredentials),
}

pub(super) fn select(secrets: &ProviderSecrets) -> Option<Choice> {
    super::vault_credentials::valid_key(secrets)
        .map(Choice::ApiKey)
        .or_else(|| super::vault_credentials::oauth_credentials(secrets).map(Choice::OAuth))
}
