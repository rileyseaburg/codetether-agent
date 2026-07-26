use crate::{provider::openai_codex::OAuthCredentials, secrets::ProviderSecrets};

pub(super) enum Choice {
    ApiKey(String),
    OAuth(OAuthCredentials),
}

pub(super) fn select(secrets: &ProviderSecrets) -> Option<Choice> {
    super::vault_credentials::valid_key(secrets)
        .map(Choice::ApiKey)
        .or_else(|| {
            super::vault_credentials::oauth_credentials(secrets).map(Choice::OAuth)
        })
}