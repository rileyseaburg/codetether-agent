use crate::provider::openai_codex::OpenAiCodexProvider;
use crate::provider::traits::Provider;
use crate::secrets::ProviderSecrets;
use std::sync::Arc;

pub(in crate::provider) fn dispatch(secrets: &ProviderSecrets) -> Option<Arc<dyn Provider>> {
    build(secrets, None)
}

pub(in crate::provider) fn dispatch_vault(
    provider_id: &str,
    secrets: &ProviderSecrets,
) -> Option<Arc<dyn Provider>> {
    build(secrets, Some(provider_id))
}

fn build(secrets: &ProviderSecrets, vault_id: Option<&str>) -> Option<Arc<dyn Provider>> {
    if let Some(key) = secrets.api_key.as_deref().filter(|key| !key.is_empty()) {
        return Some(Arc::new(OpenAiCodexProvider::from_api_key(key.into())));
    }
    let credentials = credentials(secrets)?;
    let provider = match vault_id {
        Some(id) => OpenAiCodexProvider::from_vault_credentials(id, credentials),
        None => OpenAiCodexProvider::from_credentials(credentials),
    };
    Some(Arc::new(provider))
}

pub(in crate::provider) use codetether_codex_auth::lifecycle::credentials;
