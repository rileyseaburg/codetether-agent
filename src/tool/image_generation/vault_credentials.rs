use super::{auth::ImagesAuth, vault_choice::Choice};
use crate::{
    provider::openai_codex::{OAuthCredentials, OpenAiCodexProvider},
    secrets::ProviderSecrets,
};
use anyhow::Result;

pub(super) const IMAGE_PROVIDER_ID: &str = "openai-codex";

pub(super) async fn resolve() -> Result<Option<ImagesAuth>> {
    let Some(secrets) = crate::secrets::get_provider_secrets(IMAGE_PROVIDER_ID).await else {
        return Ok(None);
    };
    let Some(choice) = super::vault_choice::select(&secrets) else {
        return Ok(None);
    };
    match choice {
        Choice::ApiKey(key) => {
            tracing::info!(provider = IMAGE_PROVIDER_ID, kind = "api_key", "Image auth");
            Ok(Some(ImagesAuth::openai(key)))
        }
        Choice::OAuth(credentials) => {
            tracing::info!(provider = IMAGE_PROVIDER_ID, kind = "oauth", "Image auth");
            let provider =
                OpenAiCodexProvider::from_vault_credentials(IMAGE_PROVIDER_ID, credentials);
            Ok(Some(ImagesAuth::chatgpt(
                provider.chatgpt_backend_auth().await?,
            )))
        }
    }
}

pub(super) fn valid_key(secrets: &ProviderSecrets) -> Option<String> {
    secrets.api_key.clone().filter(|key| !key.trim().is_empty())
}

pub(super) fn oauth_credentials(secrets: &ProviderSecrets) -> Option<OAuthCredentials> {
    Some(OAuthCredentials {
        access_token: secrets.extra.get("access_token")?.as_str()?.into(),
        refresh_token: secrets.extra.get("refresh_token")?.as_str()?.into(),
        expires_at: secrets.extra.get("expires_at")?.as_u64()?,
        id_token: value(secrets, "id_token"),
        chatgpt_account_id: value(secrets, "chatgpt_account_id"),
    })
}

fn value(secrets: &ProviderSecrets, name: &str) -> Option<String> {
    secrets.extra.get(name)?.as_str().map(String::from)
}