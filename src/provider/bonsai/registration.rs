//! Separate `bonsai` provider registration, with Vault precedence and explicit local opt-in.
use super::{BonsaiConfig, BonsaiProvider};
use crate::provider::{Provider, ProviderRegistry};
use crate::secrets::ProviderSecrets;
use std::{path::PathBuf, sync::Arc};
pub(crate) fn environment(registry: &mut ProviderRegistry) {
    if registry.get("bonsai").is_some() {
        return;
    }
    if std::env::var_os("BONSAI_MODEL_PATH").is_none()
        && std::env::var("CODETETHER_BONSAI").ok().as_deref() != Some("1")
    {
        return;
    }
    match BonsaiConfig::from_environment().and_then(BonsaiProvider::new) {
        Ok(provider) => registry.register(Arc::new(provider)),
        Err(error) => tracing::warn!(provider="bonsai", %error, "Native provider unavailable"),
    }
}
pub(crate) fn vault(secrets: &ProviderSecrets) -> Option<Arc<dyn Provider>> {
    let model_path = PathBuf::from(secrets.extra.get("model_path")?.as_str()?);
    let tokenizer_path = PathBuf::from(secrets.extra.get("tokenizer_path")?.as_str()?);
    let ordinal = secrets
        .extra
        .get("cuda_ordinal")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let cuda_ordinal = usize::try_from(ordinal).ok()?;
    match BonsaiProvider::new(BonsaiConfig {
        model_path,
        tokenizer_path,
        cuda_ordinal,
    }) {
        Ok(provider) => Some(Arc::new(provider)),
        Err(error) => {
            tracing::warn!(provider="bonsai", %error, "Native provider unavailable");
            None
        }
    }
}
