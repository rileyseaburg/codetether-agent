use super::auth::ImagesAuth;
use crate::{
    provider::openai_codex::{OAuthCredentials, OpenAiCodexProvider},
    secrets::ProviderSecrets,
};
use anyhow::Result;

pub(super) async fn resolve() -> Result<Option<ImagesAuth>> {
    if let Some(secrets) = crate::secrets::get_provider_secrets("openai").await
        && let Some(key) = valid_key(&secrets)
    {
        return Ok(Some(ImagesAuth::openai(key)));
    }
    let Some(secrets) = crate::secrets::get_provider_secrets("openai-codex").await else {
        return Ok(None);
    };
    if let Some(key) = valid_key(&secrets) {
        return Ok(Some(ImagesAuth::openai(key)));
    }
    let Some(credentials) = oauth_credentials(&secrets) else {
        return Ok(None);
    };
    let provider = OpenAiCodexProvider::from_credentials(credentials);
    Ok(Some(ImagesAuth::chatgpt(
        provider.chatgpt_backend_auth().await?,
    )))
}

fn valid_key(secrets: &ProviderSecrets) -> Option<String> {
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
